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


#[test]
fn b_test() {
    let instruct = 0b1110_1011_0000_0000_0000_0000_1111_1111u32;
    let mut reg = Register::new();
    let old_pc = 0;
    Branch::execute(instruct, &mut reg, old_pc, false);
    assert_eq!(reg.get_pc(), 0b0000_0000_0000_0000_0000_0100_0000_0100);
    assert_eq!(reg.get_lr(), 0);
}

#[test]
fn bl_test() {
    let instruct = 0b1110_1011_1111_1111_1111_1111_1111_1000u32;
    let mut reg = Register::new();
    let old_pc = 0b0000_0000_0000_0000_0000_1000_0000_0000;
    Branch::execute(instruct, &mut reg, old_pc, true);
    assert_eq!(reg.get_pc(), 0b0000_0000_0000_0000_0000_0111_1110_1000);
    assert_eq!(reg.get_lr(), 0b0000_0000_0000_0000_0000_1000_0000_0100);
}