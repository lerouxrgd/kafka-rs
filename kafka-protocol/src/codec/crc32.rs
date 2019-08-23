// Copied from rust-snappy (snap)
// https://github.com/BurntSushi/rust-snappy/blob/master/src/crc32.rs

use lazy_static::lazy_static;

const CASTAGNOLI_POLY: u32 = 0x82f63b78;

lazy_static! {
    static ref TABLE: [u32; 256] = make_table(CASTAGNOLI_POLY);
    static ref TABLE16: [[u32; 256]; 16] = {
        let mut tab = [[0; 256]; 16];
        tab[0] = make_table(CASTAGNOLI_POLY);
        for i in 0..256 {
            let mut crc = tab[0][i];
            for j in 1..16 {
                crc = (crc >> 8) ^ tab[0][crc as u8 as usize];
                tab[j][i] = crc;
            }
        }
        tab
    };
}

pub fn crc32c(buf: &[u8]) -> u32 {
    crc32c_slice8(buf)
}

/// Returns the CRC32 checksum of `buf` using the Castagnoli polynomial.
fn crc32c_slice8(mut buf: &[u8]) -> u32 {
    let tab = &*TABLE;
    let tab8 = &*TABLE16;
    let mut bytes = [0u8; 4];
    let mut crc: u32 = !0;
    while buf.len() >= 8 {
        bytes.copy_from_slice(&buf[0..4]);
        crc ^= u32::from_le_bytes(bytes);
        crc = tab8[0][buf[7] as usize]
            ^ tab8[1][buf[6] as usize]
            ^ tab8[2][buf[5] as usize]
            ^ tab8[3][buf[4] as usize]
            ^ tab8[4][(crc >> 24) as u8 as usize]
            ^ tab8[5][(crc >> 16) as u8 as usize]
            ^ tab8[6][(crc >> 8) as u8 as usize]
            ^ tab8[7][(crc) as u8 as usize];
        buf = &buf[8..];
    }
    for &b in buf {
        crc = tab[((crc as u8) ^ b) as usize] ^ (crc >> 8);
    }
    !crc
}

fn make_table(poly: u32) -> [u32; 256] {
    let mut tab = [0; 256];
    for i in 0u32..256u32 {
        let mut crc = i;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
        }
        tab[i as usize] = crc;
    }
    tab
}
