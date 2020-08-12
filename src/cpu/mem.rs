pub trait AddressBus {
    /// Get a byte from a 32-bit address
    fn get_byte(&self, addr: u32) -> u8;

    /// Get a half-word from a 32-bit address
    fn get_half_word(&self, addr: u32) -> u16 {
        (self.get_byte(addr) as u16) | ((self.get_byte(addr + 1) as u16) << 8)
    }

    /// Get a word from a 32-bit address
    fn get_word(&self, addr: u32) -> u32 {
        (self.get_byte(addr) as u32) | ((self.get_byte(addr + 1) as u32) << 16)
    }
}