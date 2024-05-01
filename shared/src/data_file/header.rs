/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-30
 */
use std::vec::IntoIter;

use super::Flags;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Header {
    version: u32,
    count: u32,
    longest: u32,
    shortest: u32,
    flags: Flags,
    separator: char,
}
impl Header {
    const DEFAULT_VERSION: u32 = 2;
    const DEFAULT_SEPARATOR: char = '%';

    pub fn new(
        version: u32,
        count: u32,
        longest: u32,
        shortest: u32,
        flags: Flags,
        separator: char,
    ) -> Header {
        Header {
            version,
            count,
            longest,
            shortest,
            flags,
            separator,
        }
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn set_count(&mut self, count: u32) -> &mut Self {
        self.count = count;
        self
    }
    pub fn longest(&self) -> u32 {
        self.longest
    }
    pub fn set_longest(&mut self, longest: u32) -> &mut Self {
        self.longest = longest;
        self
    }
    pub fn shortest(&self) -> u32 {
        self.shortest
    }
    pub fn set_shortest(&mut self, shortest: u32) -> &mut Self {
        self.shortest = shortest;
        self
    }
    pub fn separator(&self) -> char {
        self.separator
    }
    pub fn set_separator(&mut self, c: char) -> &mut Self {
        self.separator = c;
        self
    }
    pub fn flags(&self) -> &Flags {
        &self.flags
    }
    pub fn set_flags(&mut self, flags: Flags) -> &mut Self {
        self.flags = flags;
        self
    }
    pub fn is_random(&self) -> bool {
        self.flags.contains(Flags::Random)
    }
    pub fn set_is_random(&mut self) -> &mut Self {
        self.flags.insert(Flags::Random);
        self
    }
    pub fn is_ordered(&self) -> bool {
        self.flags.contains(Flags::Ordered)
    }
    pub fn set_is_ordered(&mut self) -> &mut Self {
        self.flags.insert(Flags::Ordered);
        self
    }
    pub fn is_rotated(&self) -> bool {
        self.flags.contains(Flags::Rotated)
    }
    pub fn set_is_rotated(&mut self) -> &mut Self {
        self.flags.insert(Flags::Rotated);
        self
    }
    pub fn to_bytes(&self) -> IntoIter<u8> {
        let mut bucket = vec![];

        for byte in self.version().to_be_bytes() {
            bucket.push(byte);
        }
        for byte in self.count().to_be_bytes() {
            bucket.push(byte);
        }
        for byte in self.longest().to_be_bytes() {
            bucket.push(byte);
        }
        for byte in self.shortest().to_be_bytes() {
            bucket.push(byte);
        }
        for byte in u32::from(&self.flags).to_be_bytes() {
            bucket.push(byte);
        }
        for byte in (self.separator as u32).to_le_bytes() {
            bucket.push(byte);
        }
        bucket.into_iter()
    }
}
impl Default for Header {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_VERSION,
            0,
            0,
            0,
            Flags::empty(),
            Self::DEFAULT_SEPARATOR,
        )
    }
}
