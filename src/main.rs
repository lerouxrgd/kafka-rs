use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "protocol.pest"]
pub struct ProtocolParser;

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
                            .take(2) // keep first 2 columns
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
