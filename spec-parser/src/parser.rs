use std::borrow::Cow;
use std::collections::{HashMap, HashSet, VecDeque};

use failure::{Error, Fail};
use heck::{CamelCase, SnakeCase};
use indexmap::IndexMap;
use lazy_static::*;
use pest::Parser as _;
use pest_derive::*;
use regex::Regex;

use crate::templates::motif;

/// Describes errors that occured while parsing protocol specs.
#[derive(Fail, Debug)]
#[fail(display = "Parser failure: {}", _0)]
pub struct ParserError(String);

impl ParserError {
    pub fn new<S: Into<String>>(msg: S) -> ParserError {
        ParserError(msg.into())
    }
}

#[derive(Parser)]
#[grammar = "protocol.pest"]
pub struct ProtocolParser;

pub struct SpecParser<'a> {
    pub err_code_rows: motif::ErrorCodeRows,
    pub api_key_rows: motif::ApiKeyRows,
    req_resp_specs: IndexMap<String, VersionedSpecs<'a>>,
}

/// Vector of (version, spec, fields_doc = {f_name -> doc_string})
type VersionedSpecs<'a> = Vec<(i16, Spec<'a>, HashMap<Cow<'a, str>, String>)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Spec<'a> {
    Value(Primitive),
    Array(Box<Spec<'a>>),
    Struct(Vec<(Cow<'a, str>, Spec<'a>)>),
}

impl<'a> SpecParser<'a> {
    pub fn new(raw: &'a str) -> Result<Self, Error> {
        let parsed_file = ProtocolParser::parse(Rule::file, &raw)?
            .next() // there is exactly one { file }
            .expect("Unreachable file rule");

        let mut err_code_rows = vec![];
        let mut api_key_rows = vec![];
        let mut req_resp_specs = IndexMap::new();

        for target in parsed_file.into_inner() {
            match target.as_rule() {
                Rule::error_codes => {
                    err_code_rows = target
                        .into_inner() // inner { table }
                        .next() // there is exactly one { table }
                        .expect("Unreachable error_codes table rule")
                        .into_inner() // inner { tr }
                        .into_iter()
                        .map(|tr| {
                            let row = tr
                                .into_inner() // inner { td }
                                .into_iter()
                                .map(|td| td.into_inner().as_str()) // inner { content }
                                .collect::<Vec<_>>();
                            (
                                String::from(row[0]).to_camel_case(),
                                String::from(row[1]),
                                capped_comment(&format!("{} Retriable: {}.", row[3], row[2]), 4),
                            )
                        })
                        .collect::<Vec<_>>();
                }

                Rule::api_keys => {
                    api_key_rows = target
                        .into_inner() // inner { table }
                        .next() // there is exactly one { table }
                        .expect("Unreachable api_keys table rule")
                        .into_inner() // inner { tr }
                        .into_iter()
                        .map(|tr| {
                            let row = tr
                                .into_inner() // inner { td }
                                .into_iter()
                                .map(|td| {
                                    td.into_inner() // inner { a }
                                        .next() // there is exactly one { a }
                                        .expect("Unreachable api_keys a rule")
                                        .into_inner() // inner { content }
                                        .as_str()
                                })
                                .collect::<Vec<_>>();
                            (String::from(row[0]), String::from(row[1]))
                        })
                        .collect::<Vec<_>>();
                }

                Rule::req_resp => {
                    let parsed_spec = ProtocolParser::parse(Rule::spec, target.as_str())?
                        .next() // there is exactly one { spec }
                        .expect("Unreachable spec rule");

                    let mut curr_name = None;
                    let mut curr_version = None;
                    let mut curr_spec = None;

                    for section in parsed_spec.into_inner() {
                        match section.as_rule() {
                            Rule::content => {
                                let (name, version, spec) = parse_struct_spec(section.as_str())?;
                                curr_name = Some(name);
                                curr_version = Some(version);
                                curr_spec = Some(spec);
                            }

                            Rule::table => {
                                let fields_doc = section
                                    .into_inner() // inner { td }
                                    .map(|tr| {
                                        let row = tr
                                            .into_inner() // inner { td }
                                            .into_iter()
                                            .map(|td| td.into_inner().as_str()) // inner { content }
                                            .collect::<Vec<_>>();
                                        (clean_name(row[0]), String::from(row[1]))
                                    })
                                    .collect::<HashMap<_, _>>();

                                let name = curr_name.take().expect("unreachable no name parsed");
                                let version = (
                                    curr_version.take().expect("unreachable no version parsed"),
                                    curr_spec.take().expect("unreachable no spec parsed"),
                                    fields_doc,
                                );

                                match req_resp_specs.get_mut(&name) {
                                    None => {
                                        req_resp_specs.insert(name, vec![version]);
                                    }
                                    Some(versions) => {
                                        versions.push(version);
                                    }
                                };
                            }

                            _ => unreachable!("No other rules"),
                        }
                    }
                }

                _ => (),
            }
        }

        Ok(SpecParser {
            err_code_rows,
            api_key_rows,
            req_resp_specs,
        })
    }

    pub fn iter_req_resp(&self) -> impl Iterator<Item = (&String, &VersionedSpecs)> {
        let mut i = 0;

        std::iter::from_fn(move || {
            let req_resp = self.req_resp_specs.get_index(i);
            i += 1;
            req_resp
        })
    }
}

pub trait ReqRespMotif {
    fn enum_name(&self) -> String;
    fn enum_vfields(&self) -> motif::EnumVfields;
    fn mod_name(&self) -> String;
    fn mod_vstructs(&self) -> motif::ModVstructs;
}

impl<'a> ReqRespMotif for (&'a String, &'a VersionedSpecs<'a>) {
    fn enum_name(&self) -> String {
        self.0.clone()
    }

    fn enum_vfields(&self) -> motif::EnumVfields {
        fn rust_type_for(
            field_name: &str,
            field_spec: &Spec,
            enum_name: &str,
            version: &i16,
        ) -> String {
            match field_spec {
                Spec::Value(primitive) => primitive.rust_type(),
                Spec::Array(inner) => format!(
                    "Vec<{}>",
                    rust_type_for(field_name, &*inner, enum_name, version)
                ),
                Spec::Struct(_) => format!(
                    "{}::v{}::{}",
                    enum_name.to_snake_case(),
                    version,
                    field_name.to_camel_case()
                ),
            }
        }

        self.1
            .iter()
            .map(|(version, spec, docs)| {
                let fields: motif::Fields = if let Spec::Struct(fields) = spec {
                    fields
                        .iter()
                        .map(|(field_name, field_spec)| {
                            (
                                field_name.to_string(),
                                rust_type_for(field_name, field_spec, self.0, version),
                                docs.get(field_name).map_or_else(
                                    || String::default(),
                                    |doc| capped_comment(doc, 8),
                                ),
                            )
                        })
                        .collect::<Vec<_>>()
                } else {
                    unreachable!("All specs are Spec::Struct(_)");
                };
                fields
            })
            .collect::<Vec<_>>()
    }

    fn mod_name(&self) -> String {
        self.0.to_snake_case()
    }

    fn mod_vstructs(&self) -> motif::ModVstructs {
        fn rust_type_for(field_name: &str, field_spec: &Spec) -> String {
            match field_spec {
                Spec::Value(primitive) => primitive.rust_type(),
                Spec::Array(inner) => format!("Vec<{}>", rust_type_for(field_name, &*inner)),
                Spec::Struct(_) => field_name.to_camel_case(),
            }
        }

        fn spec_deps<'a>(spec: &'a Spec<'_>) -> Vec<(String, &'a Spec<'a>)> {
            let mut deps = Vec::new();
            let mut q = VecDeque::new();

            // Initializes the specs exploration queue
            if let Spec::Struct(fields) = spec {
                for (f_name, f_spec) in fields {
                    match f_spec {
                        Spec::Value(_) => (),
                        Spec::Array(inner) => q.push_back((f_name.to_camel_case(), &**inner)),
                        Spec::Struct(_) => q.push_back((f_name.to_camel_case(), f_spec)),
                    }
                }
            } else {
                unreachable!("All specs are Spec::Struct(_)");
            }

            // Builds the specs dependencies stack
            while let Some((ref f_name, ref f_spec)) = q.pop_front() {
                match (f_name, f_spec) {
                    (_, Spec::Value(_)) => (),
                    (_, Spec::Array(inner)) => q.push_back((f_name.to_camel_case(), &**inner)),
                    (_, Spec::Struct(fields)) => {
                        for (f_name, f_spec) in fields {
                            match f_spec {
                                Spec::Value(_) => (),
                                Spec::Array(inner) => {
                                    q.push_back((f_name.to_camel_case(), &**inner))
                                }
                                Spec::Struct(_) => q.push_back((f_name.to_camel_case(), f_spec)),
                            }
                        }
                        deps.push((f_name.to_camel_case(), &**f_spec));
                    }
                }
            }

            deps
        }

        self.1
            .iter()
            .map(|(_, spec, docs)| {
                let structs: Vec<(String, &Spec)> = spec_deps(spec);
                structs
                    .iter()
                    .map(|(struct_name, struct_spec)| {
                        let struct_fields: motif::Fields = if let Spec::Struct(fields) = struct_spec
                        {
                            fields
                                .iter()
                                .map(|(field_name, field_spec)| {
                                    (
                                        field_name.to_string(),
                                        rust_type_for(field_name, field_spec),
                                        docs.get(field_name).map_or_else(
                                            || String::default(),
                                            |doc| capped_comment(doc, 12),
                                        ),
                                    )
                                })
                                .collect::<Vec<_>>()
                        } else {
                            unreachable!("All specs are Spec::Struct(_)");
                        };
                        (struct_name.clone(), struct_fields)
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|versions| versions.len() > 0)
            .collect::<Vec<_>>()
    }
}

fn clean_name(s: &str) -> Cow<'_, str> {
    if s.contains('\'') {
        s.replace('\'', "").into()
    } else {
        s.into()
    }
}

fn capped_comment(text: &str, nb_indent: usize) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b.{1,70}\b\W?").expect("Invalid regex");
    }
    let comment = if nb_indent > 0 {
        format!("{}///", " ".repeat(nb_indent))
    } else {
        String::from("///")
    };
    RE.captures_iter(text)
        .into_iter()
        .filter_map(|c| c.get(0))
        .map(|c| format!("{} {}", comment, c.as_str()))
        .collect::<Vec<_>>()
        .as_slice()
        .join("\n")
}

fn parse_struct_spec<'a>(raw: &'a str) -> Result<(String, i16, Spec<'a>), Error> {
    #[derive(Debug, Clone)]
    enum Field<'a> {
        Simple(Cow<'a, str>),
        Array(Cow<'a, str>),
    }

    impl<'a> Field<'a> {
        fn new(name: &str) -> Field {
            if name.chars().nth(0).expect("no first char") == '['
                && name.chars().last().expect("no last char") == ']'
            {
                Field::Array(clean_name(&name[1..name.len() - 1]))
            } else {
                Field::Simple(clean_name(name))
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Kind<'a> {
        Value(Primitive),
        Array(Primitive),
        Struct(Vec<Field<'a>>),
    }

    impl<'a> Kind<'a> {
        fn for_root(raw: &str) -> Kind {
            let fields = raw
                .split(' ')
                .filter(|s| *s != "")
                .collect::<Vec<_>>()
                .iter()
                .map(|name| Field::new(name))
                .collect::<Vec<_>>();
            Kind::Struct(fields)
        }

        fn for_field(raw: &str) -> Kind {
            lazy_static! {
                static ref ARRAY: Regex = Regex::new(r"ARRAY\((.+?)\)").expect("Invalid regex");
            }

            let kind = raw.split(' ').filter(|s| *s != "").collect::<Vec<_>>();
            if kind.len() == 1 {
                let field = kind[0];
                if Primitive::is_valid(field) {
                    Kind::Value(Primitive::from(field))
                } else if ARRAY.is_match(field) {
                    let inner = ARRAY
                        .captures(field)
                        .expect("unreachable field kind parsing")
                        .get(1)
                        .expect("unreachable field kind capture")
                        .as_str();
                    if Primitive::is_valid(inner) {
                        Kind::Array(Primitive::from(inner))
                    } else {
                        Kind::Struct(vec![Field::new(inner)])
                    }
                } else {
                    Kind::Struct(vec![Field::new(field)])
                }
            } else {
                let fields = kind.iter().map(|name| Field::new(name)).collect::<Vec<_>>();
                Kind::Struct(fields)
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Line<'a> {
        name: Cow<'a, str>,
        kind: Kind<'a>,
    }

    fn insert_spec<'a>(
        mut specs: HashMap<Cow<'a, str>, Spec<'a>>,
        line: Line<'a>,
    ) -> Result<HashMap<Cow<'a, str>, Spec<'a>>, Error> {
        match line {
            Line {
                kind: Kind::Value(primitive),
                name,
                ..
            } => {
                specs.insert(name.into(), Spec::Value(primitive));
            }

            Line {
                kind: Kind::Array(primitive),
                name,
                ..
            } => {
                specs.insert(name.into(), Spec::Array(Box::new(Spec::Value(primitive))));
            }

            Line {
                kind: Kind::Struct(fields),
                name,
                ..
            } => {
                let mut inner_specs = vec![];
                for field in fields {
                    match field {
                        Field::Simple(ref name) => {
                            let spec = specs.get(name).ok_or_else(|| {
                                ParserError::new(format!("Missing spec for field: {}", name))
                            })?;
                            inner_specs.push((name.clone(), spec.clone()));
                        }
                        Field::Array(ref name) => {
                            let spec = specs.get(name).ok_or_else(|| {
                                ParserError::new(format!("Missing spec for field: {}", name))
                            })?;
                            inner_specs.push((name.clone(), Spec::Array(Box::new(spec.clone()))));
                        }
                    }
                }
                specs.insert(name.into(), Spec::Struct(inner_specs));
            }
        };

        Ok(specs)
    }

    lazy_static! {
        static ref HEADER: Regex =
            Regex::new(r"(\w+) (\w+) \(Version: (\d+)\) =>(.*)").expect("Invalid regex");
    }

    let raw_lines = raw.split('\n').collect::<Vec<_>>();
    let (first, rest) = raw_lines.split_first().expect("Unreachable split fail");

    let header = HEADER.captures(first).ok_or_else(|| {
        ParserError::new(format!("First line didn't match: {:?} {}", *HEADER, first))
    })?;

    let (name, version) = match (header.get(1), header.get(2), header.get(3)) {
        (Some(name), Some(genre), Some(version)) => {
            let version: i16 = version.as_str().parse()?;
            let name = format!("{}{}", name.as_str(), genre.as_str());
            (name, version)
        }
        _ => return Err(ParserError::new(format!("Invalid name match: {:?}", header)).into()),
    };

    let root = Kind::for_root(header.get(4).map_or("", |m| m.as_str().trim()));

    let mut lines = rest
        .to_vec()
        .iter()
        .filter(|s| **s != "")
        .map(|s| {
            let parts = s.split(" =>").collect::<Vec<_>>();

            let name = clean_name(parts.get(0).expect(&format!("Invalid line: {}", s)).trim());
            let kind = Kind::for_field(parts.get(1).expect(&format!("Invalid line: {}", s)));

            Line { name, kind }
        })
        .collect::<Vec<_>>();

    let mut fields_spec = HashMap::new();
    lines.reverse();
    for line in lines {
        fields_spec = insert_spec(fields_spec, line.clone())?;
    }

    let mut specs = vec![];
    if let Kind::Struct(fields) = root {
        for field in fields {
            match field {
                Field::Simple(ref name) => {
                    let field_spec = fields_spec.get(name).ok_or_else(|| {
                        ParserError::new(format!("Missing spec for root field: {}", name))
                    })?;
                    specs.push((name.clone(), field_spec.clone()));
                }
                Field::Array(ref name) => {
                    let field_spec = fields_spec.get(name).ok_or_else(|| {
                        ParserError::new(format!("Missing spec for root field: {}", name))
                    })?;
                    specs.push((name.clone(), Spec::Array(Box::new(field_spec.clone()))));
                }
            }
        }
    }

    Ok((name, version, Spec::Struct(specs)))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Primitive {
    /// Represents a boolean value in a byte. Values 0 and 1 are used to
    /// represent false and true respectively. When reading a boolean value,
    /// any non-zero value is considered true.
    Boolean,
    /// Represents an integer between -2^7 and 2^7-1 inclusive.
    Int8,
    /// Represents an integer between -2^15 and 2^15-1 inclusive.
    /// The values are encoded using two bytes in network byte order (big-endian).
    Int16,
    /// Represents an integer between -2^31 and 2^31-1 inclusive.
    /// The values are encoded using four bytes in network byte order (big-endian).
    Int32,
    /// Represents an integer between -2^63 and 2^63-1 inclusive.
    /// The values are encoded using eight bytes in network byte order (big-endian).
    Int64,
    /// Represents an integer between 0 and 2^32-1 inclusive.
    /// The values are encoded using four bytes in network byte order (big-endian).
    Uint32,
    /// Represents an integer between -2^31 and 2^31-1 inclusive.
    /// Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
    Varint,
    /// Represents an integer between -2^63 and 2^63-1 inclusive.
    /// Encoding follows the variable-length zig-zag encoding from Google Protocol Buffers.
    Varlong,
    /// Represents a sequence of characters. First the length N is given as an INT16.
    /// Then N bytes follow which are the UTF-8 encoding of the character sequence.
    /// Length must not be negative.
    String,
    /// Represents a sequence of characters or null. For non-null strings,
    /// first the length N is given as an INT16. Then N bytes follow which are
    /// the UTF-8 encoding of the character sequence. A null value is encoded with
    /// length of -1 and there are no following bytes.
    NullableString,
    /// Represents a raw sequence of bytes. First the length N is given as an INT32.
    /// Then N bytes follow.
    Bytes,
    /// Represents a raw sequence of bytes or null. For non-null values,
    /// first the length N is given as an INT32. Then N bytes follow.
    /// A null value is encoded with length of -1 and there are no following bytes.
    NullableBytes,
    /// Represents a sequence of Kafka records as NULLABLE_BYTES.
    Records,
}

impl Primitive {
    fn from(raw: &str) -> Primitive {
        match raw {
            "BOOLEAN" => Primitive::Boolean,
            "INT8" => Primitive::Int8,
            "INT16" => Primitive::Int16,
            "INT32" => Primitive::Int32,
            "INT64" => Primitive::Int64,
            "UINT32" => Primitive::Uint32,
            "VARINT" => Primitive::Varint,
            "VARLONG" => Primitive::Varlong,
            "STRING" => Primitive::String,
            "NULLABLE_STRING" => Primitive::NullableString,
            "BYTES" => Primitive::Bytes,
            "NULLABLE_BYTES" => Primitive::NullableBytes,
            "RECORDS" => Primitive::Records,
            _ => unreachable!("Unknown primitive: {}", raw),
        }
    }

    fn is_valid(raw: &str) -> bool {
        lazy_static! {
            static ref VALIDS: HashSet<String> = {
                let s: HashSet<_> = vec![
                    "BOOLEAN",
                    "INT8",
                    "INT16",
                    "INT32",
                    "INT64",
                    "UINT32",
                    "VARINT",
                    "VARLONG",
                    "STRING",
                    "NULLABLE_STRING",
                    "BYTES",
                    "NULLABLE_BYTES",
                    "RECORDS",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect();
                s
            };
        }
        VALIDS.contains(raw)
    }

    fn rust_type(&self) -> String {
        match *self {
            Primitive::Boolean => "bool".to_string(),
            Primitive::Int8 => "i8".to_string(),
            Primitive::Int16 => "i16".to_string(),
            Primitive::Int32 => "i32".to_string(),
            Primitive::Int64 => "i64".to_string(),
            Primitive::Uint32 => "u32".to_string(),
            Primitive::Varint => "crate::types::Varint".to_string(),
            Primitive::Varlong => "crate::types::Varlong".to_string(),
            Primitive::String => "String".to_string(),
            Primitive::NullableString => "crate::types::NullableString".to_string(),
            Primitive::Bytes => "crate::types::Bytes".to_string(),
            Primitive::NullableBytes => "crate::types::NullableBytes".to_string(),
            Primitive::Records => "crate::types::Records".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn parse_error_codes() {
        let raw = include_str!("protocol.html");
        let parser = SpecParser::new(raw).unwrap();
        for row in parser.err_code_rows {
            println!("{:?}", row);
        }
    }

    #[test]
    #[ignore]
    fn parse_api_keys() {
        let raw = include_str!("protocol.html");
        let parser = SpecParser::new(raw).unwrap();
        for row in parser.api_key_rows {
            println!("{:?}", row);
        }
    }

    #[test]
    #[ignore]
    fn parse_req_resp() {
        let raw = include_str!("protocol.html");
        let parser = SpecParser::new(raw).unwrap();
        println!("{:?}", parser.req_resp_specs.get_index(0));
        println!(
            "{:?}",
            parser
                .req_resp_specs
                .get_index(parser.req_resp_specs.len() - 1)
        );
    }

    #[test]
    fn parse_enum_vfields() {
        let raw = include_str!("protocol.html");
        let parser = SpecParser::new(raw).unwrap();
        let mut it = parser.iter_req_resp();
        let req_resp = it.next().unwrap();
        let vfields = req_resp.enum_vfields();
        println!("{:?}", req_resp.0);
        println!("{:?}", req_resp.1.get(0).unwrap());
        println!("{:?}", vfields);
    }

    #[test]
    fn parse_mod_vstructs() {
        let raw = include_str!("protocol.html");
        let parser = SpecParser::new(raw).unwrap();
        let mut it = parser.iter_req_resp();
        let req_resp = it.next().unwrap();
        let req_resp = it.next().unwrap();
        let vstructs = req_resp.mod_vstructs();
        println!("{:?}", req_resp.0.to_snake_case());
        println!("{:?}", req_resp.1.get(0).unwrap());
        println!("{:?}", vstructs);
    }

    #[test]
    fn parse_spec() {
        use super::Spec::*;

        let raw = "CreateTopics Request (Version: 0) => [create_topic_requests] timeout 
  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries] 
    topic => STRING
    num_partitions => INT32
    replication_factor => INT16
    replica_assignment => partition [replicas] 
      partition => INT32
      replicas => INT32
    config_entries => config_name config_value 
      config_name => STRING
      config_value => NULLABLE_STRING
  timeout => INT32";

        let (name, version, spec) = parse_struct_spec(raw).unwrap();

        assert_eq!("CreateTopicsRequest", name);
        assert_eq!(0, version);
        assert_eq!(
            Struct(vec![
                (
                    "create_topic_requests".into(),
                    Array(Box::new(Struct(vec![
                        ("topic".into(), Value(Primitive::String)),
                        ("num_partitions".into(), Value(Primitive::Int32)),
                        ("replication_factor".into(), Value(Primitive::Int16)),
                        (
                            "replica_assignment".into(),
                            Array(Box::new(Struct(vec![
                                ("partition".into(), Value(Primitive::Int32)),
                                ("replicas".into(), Array(Box::new(Value(Primitive::Int32))))
                            ])))
                        ),
                        (
                            "config_entries".into(),
                            Array(Box::new(Struct(vec![
                                ("config_name".into(), Value(Primitive::String)),
                                ("config_value".into(), Value(Primitive::NullableString))
                            ])))
                        )
                    ])))
                ),
                ("timeout".into(), Value(Primitive::Int32))
            ]),
            spec
        );
    }

}
