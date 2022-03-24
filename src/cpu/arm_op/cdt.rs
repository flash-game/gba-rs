use crate::cpu::reg::Register;
use fantasy_util::bit::usize::BitUtil;

pub struct CoprocessorDataTransfers {}

impl CoprocessorDataTransfers {
    pub fn execute(is_load: bool, instruct: u32, reg: &mut Register) {
        let offset = instruct & 0b1111_1111;
        let coprocessor_num = instruct.extract(8, 4);
        let cr_d = instruct.extract(12, 4);
        let rn = instruct.extract(16, 4);
        // TODO
    }
}
