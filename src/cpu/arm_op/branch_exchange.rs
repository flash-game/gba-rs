use crate::cpu::reg::{Mode, OpType, R0, R1, Register, RegNum};

///  ________________________________________________________________________________
/// |31    28|                                                                 |3  0|
/// |  cond  |                                                                 | Rn |
/// ---------------------------------------------------------------------------------
pub fn bx_execute(instruct: u32, reg: &mut Register) {
    let rn = (instruct & 0b1111) as u8;
    let new_pc = reg.reg_val(rn);
    reg.set_pc(new_pc & !1u32);
    // SET ARM or THUMB
    reg.cpsr.set_op_type(if (new_pc & 0b1) == 0 { OpType::ARM } else { OpType::Thumb });
}

#[test]
fn bx_test() {
    let instruct = 0b1110_0001_0010_1111_1111_1111_0001_0001u32;
    let mut reg = Register::new();

    let mut pc = 0x00002001;
    reg.set_reg(R1, pc);
    bx_execute(instruct, &mut reg);
    assert_eq!(reg.cpsr.op_status(), OpType::Thumb);
    assert_eq!(reg.get_pc(), 0x00002000);

    pc = 0x00002000;
    reg.set_reg(R1, pc);
    bx_execute(instruct, &mut reg);
    assert_eq!(reg.cpsr.op_status(), OpType::ARM);
    assert_eq!(reg.get_pc(), 0x00002000);
}
