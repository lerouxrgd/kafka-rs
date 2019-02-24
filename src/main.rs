use pest::Parser;
use pest_derive::*;
use scraper::{Html, Selector};

#[derive(Parser)]
#[grammar = "protocol.pest"]
pub struct ProtocolParser;

fn main() {
    // wget https://kafka.apache.org/21/protocol.html
    let raw = include_str!("protocol.html");

    // let protocol = Html::parse_document(raw);
    // let s = Selector::parse("table.data-table:nth-child(58) tr").unwrap();
    // let c = protocol.select(&s).skip(1).next().unwrap();
    // println!("------> {:?}", c.inner_html());

    let file = ProtocolParser::parse(Rule::file, &raw)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for el in file.into_inner() {
        match el.as_rule() {
            Rule::h5 => {
                println!("====> {:?}", el);
                // let mut inner_rules = el.into_inner();
            }
            // _ => println!("====> {:?}", el),
            _ => (),
        }
    }

    // let a = r##"<h5><a id="protocol_partitioning_strategies" href="#protocol_partitioning_strategies">Partitioning Strategies</a></h5>"##;
    // let parsed = ProtocolParser::parse(Rule::h5, a);
    // println!("-----> {:?}", parsed);
    // println!("-----> {:?}", parsed.unwrap().next().unwrap());
}
