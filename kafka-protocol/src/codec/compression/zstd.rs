pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    zstd::decode_all(src)
}

pub fn compress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    zstd::encode_all(src, 0)
}
