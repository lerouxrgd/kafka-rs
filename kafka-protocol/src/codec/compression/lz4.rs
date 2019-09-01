use std::io::{Cursor, Error};

use lz4::{Decoder, EncoderBuilder};

pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![];
    let mut dec = Decoder::new(src)?;
    std::io::copy(&mut dec, &mut buf)?;
    Ok(buf)
}

pub fn compress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let buf = vec![];
    let mut enc = EncoderBuilder::new().level(4).build(buf)?;
    let mut cur = Cursor::new(src);
    std::io::copy(&mut cur, &mut enc)?;
    let res = enc.finish();
    res.1.map_err(Error::from)?;
    Ok(res.0)
}
