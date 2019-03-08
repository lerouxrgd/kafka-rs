use std::io::{Read, Write};
use std::ops::Deref;

#[derive(Debug)]
pub struct Varint(pub i32);

#[derive(Debug)]
pub struct Varlong(pub i64);

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
