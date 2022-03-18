use crate::cpu::reg::{OpType, Register};

///  ________________________________________________________________________________
/// |31    28|                                                                 |3  0|
/// |  cond  |                                                                 | Rn |
/// ---------------------------------------------------------------------------------
pub fn bx_execute(instruct: u32, reg: &mut Register) {
    let rn = (instruct & 0b1111) as u8;
    let new_pc = reg.reg_val(rn);
    reg.set_pc(new_pc & !1u32);
    // SET ARM or THUMB
    reg.cpsr.set_op_type(if new_pc & 0b1 == 0 { OpType::ARM } else { OpType::Thumb });
}