use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;

/// Coprocessor Data Operations
pub struct CDP {}

impl CDP {
    pub fn execute(instruct: u32, _reg: &mut Register) {
        // Coprocessor operand register
        let _cr_m = instruct & 0b1111;
        // Coprocessor information
        let _cp = instruct.extract(5, 3);
        // Coprocessor number
        let _cp_sharp = instruct.extract(8, 4);
        // Coprocessor destination register
        let _cr_d = instruct.extract(12, 4);
        // Coprocessor operand register
        let _cr_n = instruct.extract(16, 4);
        // Coprocessor operation code
        let _cp_opc = instruct.extract(20, 4);
        // TODO GBA可能没有协处理器
    }
}
