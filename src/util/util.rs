pub trait BitUtilExt {
    fn extract(self, off: u8, len: u8) -> u32;

    fn get_bit_bool(self, bit: u8) -> bool;
}

impl BitUtilExt for u32 {
    /// 获取指定长度的位，其他位置0
    fn extract(self, off: u8, len: u8) -> u32 {
        (self >> off) & ((1u32 << len) - 1)
    }

    fn get_bit_bool(self, bit: u8) -> bool {
        ((self >> bit) & 0x0000_0001) != 0
    }
}

/// Combines two 32 bit words into a 64 bit word
#[inline]
pub fn combine64(hi: u32, lo: u32) -> u64 {
    ((hi as u64) << 32) | (lo as u64)
}

/// Splits a 64 bit word into two 32 bit words
/// The return value is a tuple of (hi, lo)
#[inline]
pub fn split64(quad: u64) -> (u32, u32) {
    ((quad >> 32) as u32, quad as u32)
}
