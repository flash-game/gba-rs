/// Abstract of memory
pub trait Memory {
    //---------------------GET-----------------------

    /// Get a byte from a 32-bit address
    fn get_byte(&self, addr: u32) -> u8;

    /// Get a half-word from a 32-bit address
    fn get_half_word(&self, addr: u32) -> u16 {
        (self.get_byte(addr) as u16) | ((self.get_byte(addr + 1) as u16) << 8)
    }

    /// Get a word from a 32-bit address
    fn get_word(&self, addr: u32) -> u32 {
        (self.get_half_word(addr) as u32) | ((self.get_half_word(addr + 2) as u32) << 16)
    }

    //---------------------SET-----------------------

    /// Set a byte from a 32-bit address
    fn set_byte(&mut self, addr: u32, val: u8);

    /// Set a half-word from a 32-bit address
    fn set_half_word(&mut self, addr: u32, val: u16) {
        self.set_byte(addr, val as u8);
        self.set_byte(addr + 1, (val >> 8) as u8)
    }

    /// Set a word from a 32-bit address
    fn set_word(&mut self, addr: u32, val: u32) {
        self.set_half_word(addr, val as u16);
        self.set_half_word(addr + 1, (val >> 16) as u16)
    }
}
