mod templates;

use failure::Error;
use heck::CamelCase;
use lazy_static::*;
use pest::Parser;
use pest_derive::*;
use regex::Regex;
use templates::Templater;

// TODO: move all this in a dedicated `parser.rs` module

#[derive(Parser)]
#[grammar = "protocol.pest"]
pub struct ProtocolParser;

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

fn wip_parsing() -> Result<(), Error> {
    // wget https://kafka.apache.org/21/protocol.html
    let raw = include_str!("protocol.html");

    let templater = Templater::new()?;

    let file = ProtocolParser::parse(Rule::file, &raw)
        .expect("Unsuccessful parsing")
        .next() // there is exactly one { file }
        .unwrap();

    let mut skip_req_resp = 19;
    for target in file.into_inner() {
        match target.as_rule() {
            Rule::error_codes => {
                let err_code_rows = target
                    .into_inner() // inner { table }
                    .next() // there is exactly one { table }
                    .unwrap()
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
                let s = templater.str_err_codes(&err_code_rows);
                // println!("{}", s.unwrap());
            }

            Rule::api_keys => {
                let api_key_rows = target
                    .into_inner() // inner { table }
                    .next() // there is exactly one { table }
                    .unwrap()
                    .into_inner() // inner { tr }
                    .into_iter()
                    .map(|tr| {
                        let row = tr
                            .into_inner() // inner { td }
                            .into_iter()
                            .map(|td| {
                                td.into_inner() // inner { a }
                                    .next() // there is exactly one { a }
                                    .unwrap()
                                    .into_inner() // inner { content }
                                    .as_str()
                            })
                            .collect::<Vec<_>>();
                        (String::from(row[0]), String::from(row[1]))
                    })
                    .collect::<Vec<_>>();
                let s = templater.str_api_keys(&api_key_rows);
                // println!("{}", s.unwrap());
            }

            Rule::req_resp => {
                if skip_req_resp > 0 {
                    skip_req_resp -= 1;
                    continue;
                }

                for section in target.into_inner() {
                    match section.as_rule() {
                        Rule::table => {
                            let param_rows = section
                                .into_inner() // inner { td }
                                .map(|tr| {
                                    let row = tr
                                        .into_inner() // inner { td }
                                        .into_iter()
                                        .map(|td| td.into_inner().as_str()) // inner { content }
                                        .collect::<Vec<_>>();
                                    (String::from(row[0]), String::from(row[1]))
                                })
                                .collect::<Vec<_>>();
                            // println!("{:?}", param_rows);
                        }

                        Rule::content => {
                            println!("{}", section.as_str());
                            wip_bnf(section.as_str());
                            // println!("====> {:?}", section.as_str());
                        }

                        _ => unreachable!(),
                    }
                }

                break;
            }

            _ => (),
        }
    }

    Ok(())
}

fn type_for(name: &str) -> String {
    let name = if name.chars().nth(0).expect("no first char") == '['
        && name.chars().last().expect("no last char") == ']'
    {
        &name[1..name.len() - 1]
    } else {
        name
    };
    String::from(name).to_camel_case()
}

#[derive(Debug)]
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

#[derive(Debug)]
enum FieldType<'a> {
    Raw(Primitive),
    Struct(Vec<(&'a str, FieldType<'a>)>),
    Array(Vec<FieldType<'a>>),
}

fn wip_bnf(raw: &str) {
    let raw = "CreateTopics Request (Version: 0) => [create_topic_requests] timeout \n  create_topic_requests => topic num_partitions replication_factor [replica_assignment] [config_entries] \n    topic => STRING\n    num_partitions => INT32\n    replication_factor => INT16\n    replica_assignment => partition [replicas] \n      partition => INT32\n      replicas => INT32\n    config_entries => config_name config_value \n      config_name => STRING\n      config_value => NULLABLE_STRING\n  timeout => INT32";

    println!("{}", raw);
    let yo = raw.split('\n').collect::<Vec<_>>();
    let (first, rest) = yo.split_first().unwrap();

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\w+) (\w+) (\(Version: (\d+)\) )?=>(.*)").expect("Invalid regex");
    }
    let caps = RE.captures(first);
    println!("{:?}", caps);

    #[derive(Debug)]
    enum Type<'a> {
        Primitive(&'a str),
        Struct(Vec<&'a str>),
    }

    #[derive(Debug)]
    struct Line<'a> {
        indent: usize,
        name: &'a str,
        ret: Type<'a>,
    }

    struct Acc<'a> {
        spec: Option<FieldType<'a>>,
        buffer: Vec<Line<'a>>,
        input: Vec<Line<'a>>,
    }

    fn yoyo(mut acc: Acc) -> Acc {
        match acc {
            Acc { spec: None, .. } => {
                acc.spec = Some(FieldType::Raw(Primitive::Int8));
                acc
            }
            _ => {
                acc.spec = Some(FieldType::Raw(Primitive::Int16));
                acc
            }
        }
    };

    use std::collections::HashMap;

    fn yuyu(input: Vec<Line>) -> HashMap<&str, FieldType> {
        let specs = HashMap::new();
        if input.len() == 0 {
            return specs;
        }

        let mut buffer = vec![];
        let mut indent = input[0].indent;

        let mut it = input.into_iter();
        while let Some(line) = it.next() {
            if line.indent >= indent {
                indent = line.indent;
                buffer.push(line);
            } else {
                while buffer.len() > 0 && buffer.last().unwrap().indent <= indent {
                    let line = buffer.pop().unwrap();
                    println!("{:?}", line);
                }
                indent = line.indent;
                buffer.push(line);
            }
        }

        while buffer.len() > 0 && buffer.last().unwrap().indent <= indent {
            let line = buffer.pop().unwrap();
            println!("{:?}", line);
        }

        specs
    }

    let mut spec = FieldType::Struct(vec![]);

    let input = rest
        .to_vec()
        .into_iter()
        .map(|s| {
            let parts = s.split(" =>").collect::<Vec<_>>();

            let indent = parts.get(0).unwrap().matches(' ').count();

            let name = parts.get(0).unwrap().trim();

            let ret = parts
                .get(1)
                .unwrap()
                .split(' ')
                .filter(|p| *p != "")
                .collect::<Vec<_>>();
            let ret = if ret.len() == 1 {
                Type::Primitive(ret[0])
            } else {
                Type::Struct(ret)
            };

            Line { indent, name, ret }
        })
        .collect::<Vec<_>>();

    let mut acc = Acc {
        spec: None,
        buffer: vec![],
        input,
    };

    // TODO: generate root from `first` line
    let root = vec!["[create_topic_requests]", "timeout"];
    for field in root {
        if let FieldType::Struct(ref mut fields) = spec {
            acc = yoyo(acc);
            let spec = acc.spec.take().unwrap();
            fields.push((field, spec));
        }
    }

    // acc.input.into_iter().for_each(|x| println!("{:?}", x));
    yuyu(acc.input);

    println!("{:?}", spec);
}

fn main() {
    // wip_parsing().unwrap();
    wip_bnf("");
}
