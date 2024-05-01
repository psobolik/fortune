/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-30
 */
use std::fmt::Display;

use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    pub struct Flags: u32 {
        const Random = 0b00000001; // Are entries random?
        const Ordered = 0b00000010; // Are entries ordered?
        const Rotated = 0b00000100; // Are entries ROT13 encoded?
    }
}
impl From<&Flags> for u32 {
    fn from(value: &Flags) -> Self {
        value.bits()
    }
}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = vec![];
        if self.contains(Flags::Random) {
            flags.push("Random")
        };
        if self.contains(Flags::Ordered) {
            flags.push("Ordered")
        };
        if self.contains(Flags::Rotated) {
            flags.push("Rotated")
        };
        write!(f, "{}", flags.join(", "))
    }
}
