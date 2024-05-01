/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-30
 */

use std::vec::IntoIter;

pub use flags::Flags;
pub use header::Header;

pub mod flags;
pub mod header;

#[derive(Debug, Default)]
pub struct DataFile {
    pub header: Header,
    pub offsets: Vec<u32>,
}
impl DataFile {
    pub fn to_bytes(&self) -> IntoIter<u8> {
        let mut bucket = vec![];
        for byte in self.header.to_bytes() {
            bucket.push(byte);
        }
        for offset in &self.offsets {
            for byte in offset.to_be_bytes() {
                bucket.push(byte);
            }
        }
        bucket.into_iter()
    }
}
