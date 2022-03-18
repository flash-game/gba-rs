use crate::cpu::reg::Register;
use fantasy_util::bit::usize::BitUtil;
use crate::cpu::addrbus::AddressBus;

struct SingleDataSwap {}

impl SingleDataSwap {
    ///
    ///
    fn execute(swap_byte: bool, instruct: u32, reg: &mut Register, addr_bus: &mut dyn AddressBus) {
        let rm = (instruct & 0b1111) as u8;
        let rd = instruct.extract(12, 4) as u8;
        let rn = instruct.extract(16, 4) as u8;
        let b = instruct.get_bit_bool(22);
        if swap_byte {
            let addr = reg.reg_val(rn);
        } else {
            let addr = reg.reg_val(rn);
        }
        // If it is not a byte operation then force word align
        let addr = reg.reg_val(rn) & !((1 - if b { 1 } else { 2 }) * 3);
        let addr_val = if b { addr_bus.get_byte(addr) as u32 } else { addr_bus.get_word(addr) };
        let reg_val = reg.reg_val(rm);
        if swap_byte {
            addr_bus.set_byte(addr, reg_val as u8)
        } else {
            addr_bus.set_word(addr, reg_val)
        }
        reg.set_reg(rd, addr_val);
    }
}