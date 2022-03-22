use fantasy_util::bit::usize::BitUtil;
use crate::cpu::reg::Register;

/// Coprocessor Data Operations
pub struct CDP {}

impl CDP {
    pub fn execute(instruct: u32, reg: &mut Register) {
        // Coprocessor operand register
        let cr_m = instruct & 0b1111;
        // Coprocessor information
        let cp = instruct.extract(5, 3);
        // Coprocessor number
        let cp_sharp = instruct.extract(8, 4);
        // Coprocessor destination register
        let cr_d = instruct.extract(12, 4);
        // Coprocessor operand register
        let cr_n = instruct.extract(16, 4);
        // Coprocessor operation code
        let cp_opc = instruct.extract(20, 4);
        // TODO GBA可能没有协处理器
    }
}

