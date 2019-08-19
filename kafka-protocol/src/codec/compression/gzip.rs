use std::io::prelude::*;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![];
    GzDecoder::new(src).read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn compress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![];
    GzEncoder::new(&mut buffer, Compression::default()).write_all(src)?;
    Ok(buffer)
}
