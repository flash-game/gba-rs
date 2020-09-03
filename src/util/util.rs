pub trait BitUtilExt {
    fn extract(self, off: u8, len: u8) -> u32;
}

impl BitUtilExt for u32 {
    /// 获取指定长度的位，其他位置0
    fn extract(self, off: u8, len: u8) -> u32 {
        (self >> off) & ((1u32 << len) - 1)
    }
}