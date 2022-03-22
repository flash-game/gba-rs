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
        let b: [u8; 2] = self.data[start..(start + 2)].try_into().unwrap();
        u16::from_le_bytes(b)
    }

    fn get_word(&self, addr: u32) -> u32 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 4] = self.data[start..(start + 4)].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    fn set_byte(&self, addr: u32, val: u8) {}
    fn set_half_word(&self, addr: u32, val: u16) {}
    fn set_word(&self, addr: u32, val: u32) {}
}

struct RwMem {
    data: Vec<u8>,
    offset: u32,
}

impl Memory for RwMem {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[addr - self.offset]
    }

    fn get_half_word(&self, addr: u32) -> u16 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 2] = self.data[start..(start + 2)].try_into().unwrap();
        u16::from_le_bytes(b)
    }

    fn get_word(&self, addr: u32) -> u32 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 4] = self.data[start..(start + 4)].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    fn set_byte(&mut self, addr: u32, val: u8) {
        self.data[(addr - self.offset) as usize] = val
    }
    fn set_half_word(&mut self, addr: u32, val: u16) {
        let u16_bytes: [u8; 2] = val.to_le_bytes();
        self.data[addr] = u16_bytes[0];
        self.data[addr + 1] = u16_bytes[1];
    }
    fn set_word(&mut self, addr: u32, val: u32) {
        let u32_bytes: [u8; 4] = val.to_le_bytes();
        let addr = addr as usize;
        self.data[addr..addr + 4].copy_from_slice(&u32_bytes);
    }
}