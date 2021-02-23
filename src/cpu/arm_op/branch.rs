use crate::cpu::arm_op::instruct_execute::InstructExecute;
use crate::cpu::reg::Register;
use fantasy_util::bit::usize::BitUtil;

struct Branch {}

impl Branch {
    ///
    ///
    fn execute(instruct: u32, reg: &mut Register, old_pc: u32) {
        let offset = instruct.extract(0, 24);
        let s_offset = ((offset << 8) as i32 >> 6) as u32;
        // TODO 不知道为什么add8
        reg.set_pc(old_pc.wrapping_add(s_offset).wrapping_add(8));
        if instruct & 0x0100_0000 != 0 {
            // TODO 不知道为什么add4
            reg.set_lr(old_pc.wrapping_add(4));
        }
        unimplemented!()
    }
}