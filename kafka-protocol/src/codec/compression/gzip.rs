use std::io::prelude::*;

use flate2::read::GzDecoder;

pub fn uncompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut d = GzDecoder::new(src);
    let mut buffer: Vec<u8> = vec![];
    d.read_to_end(&mut buffer)?;
    Ok(buffer)
}
