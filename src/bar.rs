#[derive(Debug, PartialEq)]
pub struct MemBAR<T> {
    pub address: T,
    pub prefechable: bool,
}

#[derive(Debug, PartialEq)]
pub enum BAR {
    MemBAR32(MemBAR<u32>),
    MemBAR64(MemBAR<u64>),
    IoBAR(u32),
}

impl BAR {
    fn is_io_bar(word: u32) -> bool {
        word & 0b1 == 0b1
    }

    fn is_32bit_mem_bar(word: u32) -> bool {
        word & 0b10 == 0b00
    }

    fn io_bar(word: u32) -> BAR {
        if word & 0b10 == 0b10 {
            log::warn!("Rserved bit set");
        }

        BAR::IoBAR(word & !0b11)
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
}
