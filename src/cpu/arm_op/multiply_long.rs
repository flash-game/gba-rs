use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;
use crate::util::{combine64, split64};

struct MultiplyLong {}

impl MultiplyLong {
    ///
    ///
    fn execute(instruct: u32, reg: &mut Register) {
        // 有无符号bit
        let u = (instruct & 0x0040_0000) != 0;
        // 乘加bit
        let a = (instruct & 0x0020_0000) != 0;
        let s = (instruct & 0x0010_0000) != 0;
        let rdhi = instruct.extract(16, 4) as u8;
        let rdlo = instruct.extract(12, 4) as u8;
        let rn = instruct.extract(8, 4) as u8;
        let rm = instruct.extract(0, 4) as u8;
        let rn_val = reg.reg_val(rn);
        let rm_val = reg.reg_val(rm);
        let result = if !u {
            let prod = (rn_val as u64) * (rm_val as u64);
            prod.wrapping_add(
                if !a { 0u64 } else {
                    combine64(reg.reg_val(rdhi), reg.reg_val(rdlo))
                })
        } else {
            let prod = (rn_val as i64) * (rm_val as i64);
            prod.wrapping_add(
                if !a { 0i64 } else {
                    combine64(reg.reg_val(rdhi), reg.reg_val(rdlo)) as i64
                }) as u64
        };
        let (rdhi_val, rdlo_val) = split64(result);
        reg.set_reg(rdhi, rdhi_val);
        reg.set_reg(rdlo, rdlo_val);
        if s {
            let new_n = (reshi & 0x8000_0000) != 0;
            reg.cspr.set_flag_nzcv(new_n, result == 0, false, false);
        }
    }
}