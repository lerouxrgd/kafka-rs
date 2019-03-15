mod templates;

use failure::Error;
use heck::CamelCase;
use lazy_static::*;
use pest::Parser;
use regex::Regex;
use templates::Templater;

mod protocol {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "protocol.pest"]
    pub struct Parser;
}

mod bnf {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "bnf.pest"]
    pub struct Parser;
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

fn wip_parsing() -> Result<(), Error> {
    // wget https://kafka.apache.org/21/protocol.html
    let raw = include_str!("protocol.html");

    let templater = Templater::new()?;

    let file = protocol::Parser::parse(protocol::Rule::file, &raw)
        .expect("Unsuccessful parsing")
        .next() // there is exactly one { file }
        .unwrap();

    let mut skip_req_resp = 19;
    for target in file.into_inner() {
        match target.as_rule() {
            protocol::Rule::error_codes => {
                // let err_code_rows = target
                //     .into_inner() // inner { table }
                //     .next() // there is exactly one { table }
                //     .unwrap()
                //     .into_inner() // inner { tr }
                //     .into_iter()
                //     .map(|tr| {
                //         let row = tr
                //             .into_inner() // inner { td }
                //             .into_iter()
                //             .map(|td| td.into_inner().as_str()) // inner { content }
                //             .collect::<Vec<_>>();
                //         (
                //             String::from(row[0]).to_camel_case(),
                //             String::from(row[1]),
                //             capped_comment(&format!("{} Retriable: {}.", row[3], row[2]), 4),
                //         )
                //     })
                //     .collect::<Vec<_>>();
                // let s = templater.str_err_codes(&err_code_rows);
                // // println!("{}", s.unwrap());
            }

            protocol::Rule::api_keys => {
                // let api_key_rows = target
                //     .into_inner() // inner { table }
                //     .next() // there is exactly one { table }
                //     .unwrap()
                //     .into_inner() // inner { tr }
                //     .into_iter()
                //     .map(|tr| {
                //         let row = tr
                //             .into_inner() // inner { td }
                //             .into_iter()
                //             .map(|td| {
                //                 td.into_inner() // inner { a }
                //                     .next() // there is exactly one { a }
                //                     .unwrap()
                //                     .into_inner() // inner { content }
                //                     .as_str()
                //             })
                //             .collect::<Vec<_>>();
                //         (String::from(row[0]), String::from(row[1]))
                //     })
                //     .collect::<Vec<_>>();
                // let s = templater.str_api_keys(&api_key_rows);
                // // println!("{}", s.unwrap());
            }

            protocol::Rule::req_resp => {
                if skip_req_resp > 0 {
                    skip_req_resp -= 1;
                    continue;
                }

                for section in target.into_inner() {
                    match section.as_rule() {
                        protocol::Rule::table => {
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

                        protocol::Rule::content => {
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

fn wip_bnf(raw: &str) {
    let spec = bnf::Parser::parse(bnf::Rule::spec, &raw)
        .expect("Unsuccessful parsing")
        .next() // there is exactly one { spec }
        .unwrap();

    println!("{:?}", spec);
}

fn main() {
    wip_parsing().unwrap();
}
