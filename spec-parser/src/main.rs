mod parser;
mod templates;

use std::fs::{File, OpenOptions};
use std::io::{prelude::*, stdout, Write};
use std::process::{self, Command};

use docopt::Docopt;
use failure;
use reqwest;
use serde::Deserialize;

use parser::{ReqRespMotif, SpecParser};
use templates::Templater;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
  spec-parser [options]
  spec-parser (-h | --help)
  spec-parser (-V | --version)

Options:
  -o, --out=OUT    Specify output, - for stdout.
  -s, --spec=FILE  Specify input protocol.html spec file.
  -V, --version    Show version.
  -h, --help       Show this screen.
";

#[derive(Debug, Deserialize)]
struct CmdArgs {
    flag_out: Option<String>,
    flag_spec: Option<String>,
    flag_version: bool,
}

fn main() -> Result<(), failure::Error> {
    let args: CmdArgs = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}", VERSION);
        process::exit(0);
    }

    let out_file = match args.flag_out {
        Some(ref out_file) => out_file,
        None => "../kafka-protocol/src/model.rs",
    };

    let mut out: Box<dyn Write> = match out_file {
        "-" => Box::new(stdout()),
        _ => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(out_file)
                .unwrap_or_else(|e| {
                    eprintln!("Output file error: {}", e);
                    process::exit(1);
                }),
        ),
    };

    let protocol_url;
    let raw = match args.flag_spec {
        Some(file) => {
            protocol_url = "local file";
            let mut raw = String::new();
            File::open(&file)?.read_to_string(&mut raw)?;
            raw
        }
        None => {
            protocol_url = "https://kafka.apache.org/23/protocol.html";
            reqwest::get(protocol_url)?.text()?
        }
    };

    println!("Generating Rust code to: {}", out_file);

    let parser = SpecParser::new(&raw)?;
    let templ = Templater::new()?;

    out.write_all(format!("//! Generated from: {}\n", protocol_url).as_bytes())?;
    out.write_all(templ.str_headers().as_bytes())?;
    out.write_all(templ.str_err_codes(&parser.err_code_rows)?.as_bytes())?;
    out.write_all(templ.str_api_keys(&parser.api_key_rows)?.as_bytes())?;

    for req_rep in parser.iter_req_resp() {
        out.write_all(
            templ
                .str_req_resp_enum(&req_rep.enum_name(), &req_rep.enum_vfields())?
                .as_bytes(),
        )?;

        let vstructs = req_rep.mod_vstructs();
        if vstructs.len() > 0 {
            out.write_all(
                templ
                    .str_req_resp_mod(&req_rep.mod_name(), &vstructs)?
                    .as_bytes(),
            )?;
        }
    }

    if out_file != "-" {
        Command::new("rustfmt")
            .arg(out_file)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("Problem with rustfmt: {}", e);
                process::exit(1);
            });
    }

    println!("Done");
    Ok(())
}
