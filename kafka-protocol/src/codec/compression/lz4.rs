use lz4::Decoder;

pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![];
    let mut d = Decoder::new(src)?;
    std::io::copy(&mut d, &mut buffer)?;
    Ok(buffer)
}
