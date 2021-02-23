use crate::cpu::reg::Register;
use fantasy_util::bit::usize::BitUtil;

struct Multiply {}

impl Multiply {
    ///
    ///
    fn execute(instruct: u32, reg: &mut Register) {
        let rm = instruct.extract(0, 4) as u8;
        let rs = instruct.extract(8, 4) as u8;
        let rn = instruct.extract(12, 4) as u8;
        let rd = instruct.extract(16, 4) as u8;
        let a = (instruct & 0x0020_0000) != 0;
        let s = (instruct & 0x0010_0000) != 0;
        let result = reg.reg_val(rm)
            .wrapping_mul(reg.reg_val(rs))
            .wrapping_add(if a { 0 } else { reg.reg_val(rn) });
        reg.set_reg(rd, result);
        // 如果需要更改flag
        if s {
            let new_n = (result & 0x8000_0000) != 0;
            reg.cspr.set_flag_nzcv(new_n, result == 0, false, reg.cspr.flag_v());
        }
    }
}