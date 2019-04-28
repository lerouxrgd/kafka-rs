mod parser;
mod templates;

use std::io::Write;

use failure;

use parser::{ReqRespMotif, SpecParser};
use templates::Templater;

fn main() -> Result<(), failure::Error> {
    let mut out = std::io::stdout();

    let parser = SpecParser::new().unwrap();
    let templ = Templater::new().unwrap();

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

    Ok(())
}
