use std::io::{Read, Write};
use std::ops::Deref;

struct Varint(i32);

struct Varlong(i64);

impl Deref for Varint {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Varlong {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn zigzag(i: &i64) -> u64 {
    ((i << 1) ^ (i >> 63)) as u64
}

pub fn unzigzag(i: &u64) -> i64 {
    let sign = if (i & 1) != 0 { !0u64 } else { 0 };
    ((i >> 1) ^ sign) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zouzou() {
        println!("---> {:?}", zigzag(&Varlong(-2)));
        println!("---> {:?}", unzigzag(&3));
    }
}
