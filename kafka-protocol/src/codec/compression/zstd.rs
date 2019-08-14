pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    zstd::decode_all(src)
}
