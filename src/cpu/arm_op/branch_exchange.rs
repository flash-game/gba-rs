use crate::cpu::reg::{Register, OpType};
use fantasy_util::bit::usize::BitUtil;

struct BranchAndExchange {}

impl BranchAndExchange {
    ///
    ///
    fn execute(instruct: u32, reg: &mut Register) {
        let rn = instruct.extract(0, 4) as u8;
        let new_pc = reg.reg_val(rn);
        // TODO not sure
        reg.set_pc(new_pc & !1u32);
        // SET ARM or THUMB
        reg.cspr.set_op_type(if new_pc & 0x1 == 0 { OpType::ARM } else { OpType::Thumb });
    }
}