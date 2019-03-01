mod templates;

use std::io::prelude::*;
use std::net::TcpStream;

use failure::Error;
use heck::CamelCase;
use lazy_static::*;
use pest::Parser;
use pest_derive::*;
use regex::Regex;
use templates::Templater;

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

fn wip_protocol() -> Result<(), Error> {
    // wget https://kafka.apache.org/21/protocol.html
    let raw = include_str!("protocol.html");

    let templater = Templater::new()?;

    let file = ProtocolParser::parse(Rule::file, &raw)
        .expect("Unsuccessful parsing")
        .next() // there is exactly one { file }
        .unwrap();

    for el in file.into_inner() {
        match el.as_rule() {
            Rule::error_codes => {
                let err_code_rows = el
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
                println!("{}", s.unwrap());
            }

            // _ => println!("====> {:?}", el),
            _ => (),
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;

    let api_key = 18 as i16;
    let api_key = &api_key.to_be_bytes();

    let api_version = 0 as i16;
    let api_version = &api_version.to_be_bytes();

    let correlation_id = 42 as i32;
    let correlation_id = &correlation_id.to_be_bytes();

    let client_id = -1 as i16;
    let client_id = &client_id.to_be_bytes();

    let size = ((api_key.len() + api_version.len() + correlation_id.len() + client_id.len())
        as i32)
        .to_be_bytes();

    let mut buff = Vec::with_capacity(4);
    buff.extend_from_slice(&size);
    buff.write(api_key)?;
    buff.write(api_version)?;
    buff.write(correlation_id)?;
    buff.write(client_id)?;
    stream.write(&dbg!(buff))?;

    let resp: &mut [u8] = &mut [0; 512];
    stream.read(resp)?;
    println!("---> {:?}", resp);

    Ok(())
}
