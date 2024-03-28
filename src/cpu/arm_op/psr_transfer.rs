use fantasy_util::bit::usize::BitUtil;
use crate::cpu::reg::Register;

pub struct PsrTransfer {}

impl PsrTransfer {
    pub fn mrs(instruct: u32, reg: &mut Register) {
        let rd = instruct.extract(12, 4);
        let src_psr = if instruct.get_bit_bool(22) {
            reg.spsr()
        } else {
            reg.cpsr.value()
        };
    }

    pub fn msr(instruct: u32, reg: &mut Register) {}
}