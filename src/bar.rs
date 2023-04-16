use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct MemBAR<T> {
    pub address: T,
    pub prefechable: bool,
}

#[derive(Debug, PartialEq)]
pub struct IoBAR {
    pub address: u32,
}

#[derive(Debug, PartialEq)]
pub enum BAR {
    MemBAR32(MemBAR<u32>),
    MemBAR64(MemBAR<u64>),
    IoBAR(IoBAR),
}

impl BAR {
    fn is_io_bar(word: u32) -> bool {
        word & 0b1 == 0b1
    }

    fn is_32bit_mem_bar(word: u32) -> bool {
        word & 0b110 == 0b000
    }

    fn io_bar(word: u32) -> BAR {
        if word & 0b10 == 0b10 {
            log::warn!("Rserved bit set");
        }

        BAR::IoBAR(IoBAR {
            address: word & !0b11,
        })
    }

    fn mem_32bit_bar(word: u32) -> BAR {
        BAR::MemBAR32(MemBAR {
            address: word & !0b1111,
            prefechable: word & 0b1000 == 0b1000,
        })
    }

    fn mem_64bit_bar(word0: u32, word1: u32) -> BAR {
        BAR::MemBAR64(MemBAR {
            address: (word1 as u64) << 32 | (word0 & !0b1111) as u64,
            prefechable: word0 & 0b1000 == 0b1000,
        })
    }

    pub fn new(b: &[u8]) -> Vec<BAR> {
        let mut offset = 0;
        let mut bars = vec![];

        while offset < b.len() {
            let word = u32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
            if Self::is_io_bar(word) {
                bars.push(Self::io_bar(word));
                offset += 4;
            } else if Self::is_32bit_mem_bar(word) {
                bars.push(Self::mem_32bit_bar(word));
                offset += 4;
            } else {
                offset += 4;
                let next_word = u32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
                bars.push(Self::mem_64bit_bar(word, next_word));
                offset += 4;
            }
        }

        bars
    }

    pub fn is_allocated(&self) -> bool {
        match self {
            BAR::IoBAR(b) => b.address != 0,
            BAR::MemBAR32(b) => b.address != 0,
            BAR::MemBAR64(b) => b.address != 0,
        }
    }

    pub fn to_string(&self, _: u8) -> String {
        match self {
            BAR::IoBAR(b) => format!("I/O ports at {:0>4x}", b.address),
            BAR::MemBAR32(b) => format!(
                "Memory at {:0>8x} (32-bit, {}prefetchable)",
                b.address,
                if b.prefechable { "" } else { "non-" }
            ),
            BAR::MemBAR64(b) => format!(
                "Memory at {:0>8x} (64-bit, {}prefetchable)",
                b.address,
                if b.prefechable { "" } else { "non-" }
            ),
        }
    }
}

impl Display for BAR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(0))
    }
}
