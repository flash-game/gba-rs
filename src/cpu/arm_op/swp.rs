use fantasy_util::bit::usize::BitUtil;

use crate::cpu::addrbus::AddressBus;
use crate::cpu::mem::Memory;
use crate::cpu::reg::Register;

pub struct SingleDataSwap {}

impl SingleDataSwap {
    ///
    ///
    pub fn execute(swap_byte: bool, instruct: u32, reg: &mut Register, addr_bus: &mut dyn AddressBus) {
        let rm = (instruct & 0b1111) as u8;
        let rd = instruct.extract(12, 4) as u8;
        let rn = instruct.extract(16, 4) as u8;
        let rm_val = reg.reg_val(rm);
        let rn_addr_val = if swap_byte {
            let addr = reg.reg_val(rn); // TODO align byte
            addr_bus.set_byte(addr, rm_val as u8);
            addr_bus.get_byte(addr) as u32
        } else {
            let addr = reg.reg_val(rn); // TODO align word
            addr_bus.set_word(addr, rm_val);
            addr_bus.get_word(addr)
        };
        // // If it is not a byte operation then force word align
        // let addr = reg.reg_val(rn) & !((1 - if b { 1 } else { 2 }) * 3);
        // let addr_val = if b { addr_bus.get_byte(addr) as u32 } else { addr_bus.get_word(addr) };

        reg.set_reg(rd, rn_addr_val);
    }
}
