use std::convert::TryInto;
use crate::cpu::mem::Memory;

pub struct AddressBus {
    bios: [u8; 0x00003FFF],
}

impl AddressBus {
    fn select_mem(&mut self, addr: u32) -> &mut [u8] {
        match addr {
            0..=0x00003FFF => &mut self.bios, //BIOS - System ROM
            _ => unreachable!()
        }
    }
}

impl Memory for AddressBus {
    fn get_byte(&self, addr: u32) -> u8 {
        todo!()
    }

    fn set_byte(&self, addr: u32, val: u8) {
        todo!()
    }
}


struct RoMem {
    data: Vec<u8>,
    offset: u32,
}

impl Memory for RoMem {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[addr - self.offset]
    }

    fn get_half_word(&self, addr: u32) -> u16 {
        let start = (addr - self.offset) as usize;

        let b = (&self.data[start..(start + 2)]).try_into()?;
        u16::from_le_bytes(*b)
    }

    fn get_word(&self, addr: u32) -> u32 {
        todo!()
    }

    fn set_byte(&self, addr: u32, val: u8) {}
    fn set_half_word(&self, addr: u32, val: u16) {}
    fn set_word(&self, addr: u32, val: u32) {}
}