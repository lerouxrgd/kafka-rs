mod templates;

use lazy_static::*;
use pest::Parser;
use pest_derive::*;
use regex::Regex;

#[derive(Parser)]
#[grammar = "protocol.pest"]
pub struct ProtocolParser;

fn cap_comment(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b.{1,50}\b\W?").expect("Invalid regex");
    }
    RE.captures_iter(text)
        .into_iter()
        .filter_map(|c| c.get(0))
        .map(|c| vec!["///", c.as_str()].join(" "))
        .collect::<Vec<String>>()
        .as_slice()
        .join("\n")
}

fn main() {
    // wget https://kafka.apache.org/21/protocol.html
    let raw = include_str!("protocol.html");

    let file = ProtocolParser::parse(Rule::file, &raw)
        .expect("Unsuccessful parsing")
        .next() // there is exactly one { file }
        .unwrap();

    for el in file.into_inner() {
        match el.as_rule() {
            Rule::error_codes => {
                el.into_inner() // inner { table }
                    .next() // there is exactly one { table }
                    .unwrap()
                    .into_inner() // inner { tr }
                    .into_iter()
                    .for_each(|tr| {
                        let kv = tr
                            .into_inner() // inner { td }
                            .into_iter()
                            .map(|td| td.into_inner().as_str()) // inner { content }
                            .collect::<Vec<_>>();
                        println!("{:?}", kv);
                    });
            }

            // _ => println!("====> {:?}", el),
            _ => (),
        }
    }
}
