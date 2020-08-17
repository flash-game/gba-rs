use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::Register;

pub struct Arm7 {
    reg: Register,
    address_bus: Box<dyn AddressBus>,
}

impl Arm7 {



    pub fn cond_check(&self, cond: u32) -> bool {
        let z = self.reg.flag_z();
        let c = self.reg.flag_c();
        let v = self.reg.flag_v();
        let n = self.reg.flag_n();
        match cond {
            /* EQ */
            0x0 => z,
            /* NE */
            0x1 => !z,
            /* CS */
            0x2 => c,
            /* CC */
            0x3 => !c,
            /* MI */
            0x4 => n,
            /* PL */
            0x5 => !n,
            /* VS */
            0x6 => v,
            /* VC */
            0x7 => !v,
            /* HI */
            0x8 => c && !z,
            /* LS */
            0x9 => !c || z,
            /* GE */
            0xA => n == v,
            /* LT */
            0xB => n != v,
            /* GT */
            0xC => !z && n == v,
            /* LE */
            0xD => z || n != v,
            /* AL */
            0xE => true,
            0xF => true, /* reserved, default to execute */
            _ => unreachable!(),
        }
    }
}