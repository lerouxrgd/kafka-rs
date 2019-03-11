use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Varint(pub i32);

impl Deref for Varint {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Varlong(pub i64);

impl Deref for Varlong {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullableString(pub Option<String>);

impl NullableString {
    pub fn from(s: &str) -> Self {
        NullableString(Some(s.to_string()))
    }
}

impl Deref for NullableString {
    type Target = Option<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
