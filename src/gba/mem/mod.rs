mod mem;


/// 只读地址
struct RoAddr {
    data: Vec<u8>,
    offset: u32,
}

impl RoAddr {
    fn init(init: Vec<u8>, offset: u32) -> RoAddr {
        RoAddr {
            data: init,
            offset,
        }
    }
}

impl Memory for RoAddr {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[(addr - self.offset) as usize]
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

    fn set_byte(&mut self, _addr: u32, _val: u8) {}
    fn set_half_word(&mut self, _addr: u32, _val: u16) {}
    fn set_word(&mut self, _addr: u32, _val: u32) {}
}

/// 可读写地址
struct RwAddr {
    data: Vec<u8>,
    offset: u32,
}

impl RwAddr {
    fn new(addr_start: u32, addr_end: u32) -> RwAddr {
        RwAddr {
            data: vec![0u8; (addr_end - addr_start + 1) as usize],
            offset: addr_start,
        }
    }
}

impl Memory for RwAddr {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[(addr - self.offset) as usize]
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
        let addr = addr as usize;
        self.data[addr] = u16_bytes[0];
        self.data[addr + 1] = u16_bytes[1];
    }
    fn set_word(&mut self, addr: u32, val: u32) {
        let u32_bytes: [u8; 4] = val.to_le_bytes();
        let addr = addr as usize;
        self.data[addr..addr + 4].copy_from_slice(&u32_bytes);
    }
}