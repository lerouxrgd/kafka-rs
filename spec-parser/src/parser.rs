use std::collections::{HashMap, HashSet};

use failure::{Error, Fail};
use heck::CamelCase;
use lazy_static::*;
use pest::Parser as _;
use pest_derive::*;
use regex::Regex;

use crate::common::{ApiKeyRows, ErrorCodeRows};

/// Describes errors happened while parsing protocol specs.
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

pub struct Parser {
    err_code_rows: ErrorCodeRows,
    api_key_rows: ApiKeyRows,
}

impl Parser {
    fn new() -> Result<Self, Error> {
        let raw = include_str!("protocol.html");
        let parsed_file = ProtocolParser::parse(Rule::file, &raw)?
            .next() // there is exactly one { file }
            .expect("Unreachable file rule");

        let mut err_code_rows = vec![];
        let mut api_key_rows = vec![];

        let mut skip_req_resp = 0;
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
                    // TODO: remove that, skipping stuff is just for dev
                    if skip_req_resp > 0 {
                        skip_req_resp -= 1;
                        continue;
                    }

                    for section in target.into_inner() {
                        match section.as_rule() {
                            Rule::table => {
                                let fields_doc = section
                                    .into_inner() // inner { td }
                                    .map(|tr| {
                                        let row = tr
                                            .into_inner() // inner { td }
                                            .into_iter()
                                            .map(|td| td.into_inner().as_str()) // inner { content }
                                            .collect::<Vec<_>>();
                                        (String::from(row[0]), String::from(row[1]))
                                    })
                                    .collect::<HashMap<_, _>>();
                                println!("{:?}", fields_doc);
                            }

                            Rule::content => {
                                let (name, spec) = parse_struct_spec(section.as_str())?;
                                println!("{}\n{:?}", name, spec);
                            }

                            _ => unreachable!(), // no other rules
                        }
                    }
                }

                _ => (),
            }
        }

        Ok(Parser {
            err_code_rows,
            api_key_rows,
        })
    }
}

fn capped_comment(text: &str, nb_indent: usize) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b.{1,55}\b\W?").expect("Invalid regex");
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Primitive {
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
    /// Represents a sequence of Kafka records as  NULLABLE_BYTES.
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
}

#[derive(Debug, Clone, PartialEq)]
enum Spec<'a> {
    Value(Primitive),
    Array(Box<Spec<'a>>),
    Struct(Vec<(&'a str, Spec<'a>)>),
}

fn parse_struct_spec<'a>(raw: &'a str) -> Result<(String, Spec<'a>), Error> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\w+) (\w+) (\(Version: (\d+)\) )?=>(.*)").expect("Invalid regex");
    }

    #[derive(Debug, Clone)]
    enum Field<'a> {
        Simple(&'a str),
        Array(&'a str),
    }

    impl<'a> Field<'a> {
        fn new(name: &str) -> Field {
            if name.chars().nth(0).expect("no first char") == '['
                && name.chars().last().expect("no last char") == ']'
            {
                Field::Array(&name[1..name.len() - 1])
            } else {
                Field::Simple(name)
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
        name: &'a str,
        kind: Kind<'a>,
    }

    fn insert_spec<'a>(
        mut specs: HashMap<&'a str, Spec<'a>>,
        line: Line<'a>,
    ) -> Result<HashMap<&'a str, Spec<'a>>, Error> {
        match line {
            Line {
                kind: Kind::Value(primitive),
                name,
                ..
            } => {
                specs.insert(name.clone(), Spec::Value(primitive));
            }

            Line {
                kind: Kind::Array(primitive),
                name,
                ..
            } => {
                specs.insert(name.clone(), Spec::Array(Box::new(Spec::Value(primitive))));
            }

            Line {
                kind: Kind::Struct(fields),
                name,
                ..
            } => {
                let mut inner_specs = vec![];
                for field in fields {
                    match field {
                        Field::Simple(name) => {
                            let spec = specs.get(name).ok_or_else(|| {
                                ParserError::new(format!("Missing spec for field: {}", name))
                            })?;
                            inner_specs.push((name, spec.clone()));
                        }
                        Field::Array(name) => {
                            let spec = specs.get(name).ok_or_else(|| {
                                ParserError::new(format!("Missing spec for field: {}", name))
                            })?;
                            inner_specs.push((name, Spec::Array(Box::new(spec.clone()))));
                        }
                    }
                }
                specs.insert(name, Spec::Struct(inner_specs));
            }
        };

        Ok(specs)
    }

    let raw_lines = raw.split('\n').collect::<Vec<_>>();
    let (first, rest) = raw_lines
        .split_first()
        .ok_or_else(|| ParserError::new(format!("Not enough raw lines: {:?}", raw_lines)))?;

    let caps = RE
        .captures(first)
        .ok_or_else(|| ParserError::new(format!("First line didn't match: {:?} {}", *RE, first)))?;

    let name = match (caps.get(1), caps.get(2), caps.get(4)) {
        (Some(name), Some(genre), Some(version)) => {
            format!("{}{}V{}", name.as_str(), genre.as_str(), version.as_str())
        }
        (Some(name), Some(genre), None) => format!("{}{}", name.as_str(), genre.as_str()),
        _ => return Err(ParserError::new(format!("Invalid name match: {:?}", caps)).into()),
    };

    let root = Kind::for_root(caps.get(5).map_or("", |m| m.as_str().trim()));

    let mut lines = rest
        .to_vec()
        .iter()
        .filter(|s| **s != "")
        .map(|s| {
            let parts = s.split(" =>").collect::<Vec<_>>();

            let name = parts.get(0).expect(&format!("Invalid line: {}", s)).trim();
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
                Field::Simple(name) => {
                    let field_spec = fields_spec.get(name).ok_or_else(|| {
                        ParserError::new(format!("Missing spec for root field: {}", name))
                    })?;
                    specs.push((name, field_spec.clone()));
                }
                Field::Array(name) => {
                    let field_spec = fields_spec.get(name).ok_or_else(|| {
                        ParserError::new(format!("Missing spec for root field: {}", name))
                    })?;
                    specs.push((name, Spec::Array(Box::new(field_spec.clone()))));
                }
            }
        }
    }

    Ok((name, Spec::Struct(specs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes() {
        let parser = Parser::new().unwrap();
        for row in parser.err_code_rows {
            println!("{:?}", row);
        }
    }

    #[test]
    fn api_keys() {
        let parser = Parser::new().unwrap();
        for row in parser.api_key_rows {
            println!("{:?}", row);
        }
    }

    #[test]
    fn req_resp() {
        let _parser = Parser::new().unwrap();
    }

    #[test]
    fn spec() {
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

        let (name, spec) = parse_struct_spec(raw).unwrap();

        assert_eq!("CreateTopicsRequestV0", name);
        assert_eq!(
            Struct(vec![
                (
                    "create_topic_requests",
                    Array(Box::new(Struct(vec![
                        ("topic", Value(Primitive::String)),
                        ("num_partitions", Value(Primitive::Int32)),
                        ("replication_factor", Value(Primitive::Int16)),
                        (
                            "replica_assignment",
                            Array(Box::new(Struct(vec![
                                ("partition", Value(Primitive::Int32)),
                                ("replicas", Array(Box::new(Value(Primitive::Int32))))
                            ])))
                        ),
                        (
                            "config_entries",
                            Array(Box::new(Struct(vec![
                                ("config_name", Value(Primitive::String)),
                                ("config_value", Value(Primitive::NullableString))
                            ])))
                        )
                    ])))
                ),
                ("timeout", Value(Primitive::Int32))
            ]),
            spec
        );
    }

}
