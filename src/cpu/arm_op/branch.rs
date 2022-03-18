use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;

pub struct Branch {}

impl Branch {
    ///
    pub fn execute(instruct: u32, reg: &mut Register, old_pc: u32, link: bool) {
        let offset = instruct.extract(0, 24);
        let s_offset = ((offset << 8) as i32 >> 6) as u32;
        // 根据offset设置偏移量，并提前8个字节（2个字）
        reg.set_pc(old_pc.wrapping_add(s_offset).wrapping_add(8));
        if link {
            // 将下一条指令放入lr中
            reg.set_lr(old_pc.wrapping_add(4));
        }
    }
}