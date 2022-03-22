use std::cell::RefCell;
use std::rc::Rc;

use fantasy_util::bit::usize::BitUtil;

use crate::cpu::addrbus::AddressBus;
use crate::cpu::arm_op::branch::Branch;
use crate::cpu::arm_op::branch_exchange::bx_execute;
use crate::cpu::arm_op::software_interrupt::SoftwareInterrupt;
use crate::cpu::arm_op::swp::SingleDataSwap;
use crate::cpu::arm_op_table::{ArmOpType, TABLE};
use crate::cpu::reg::Register;

pub struct Arm7<'a> {
    reg: &'a mut Register,
    address_bus: RefCell<Rc<dyn AddressBus>>,
}

impl<'a> Arm7<'a> {
    pub fn new(reg: &'a mut Register, address_bus: RefCell<Rc<dyn AddressBus>>) -> Self {
        Self {
            reg,
            address_bus,
        }
    }

    pub fn next(&mut self) {
        let reg = self.reg;
        let old_pc = reg.get_pc();
        let addr_bus = self.address_bus.borrow().as_ref();
        let instruct = addr_bus.get_word(reg.get_pc());
        reg.set_pc(old_pc.wrapping_add(4));
        // cond check
        if !self.cond_check((instruct >> 28) as u8) { return; }
        let instruct_type = get_instruction_type(instruct);
        match instruct_type {
            ArmOpType::BLX______ => {}
            ArmOpType::SMLALBT__ => {}
            ArmOpType::STMDA____ => {}
            ArmOpType::STC______ => {}
            ArmOpType::MCR______ => {}
            ArmOpType::QDSUB____ => {}
            ArmOpType::UMLAL____ => {}
            ArmOpType::SMULLS___ => {}
            ArmOpType::RSC______ => {}
            ArmOpType::ADD______ => {}
            ArmOpType::MRS______ => {}
            ArmOpType::MSR______ => {}
            ArmOpType::SMLATB___ => {}
            ArmOpType::SMULTT___ => {}
            ArmOpType::STMDB____ => {}
            ArmOpType::STMIB____ => {}
            ArmOpType::ADC______ => {}
            ArmOpType::B________ => Branch::execute(instruct, reg, old_pc, false),
            ArmOpType::BL_______ => Branch::execute(instruct, reg, old_pc, true),
            ArmOpType::SMLAWT___ => {}
            ArmOpType::STR______ => {}
            ArmOpType::STRT_____ => {}
            ArmOpType::STRB_____ => {}
            ArmOpType::STRBT____ => {}
            ArmOpType::STRH_____ => {}
            ArmOpType::STRD_____ => {}
            ArmOpType::LDR______ => {}
            ArmOpType::LDRSB____ => {}
            ArmOpType::LDRB_____ => {}
            ArmOpType::LDRD_____ => {}
            ArmOpType::LDRBT____ => {}
            ArmOpType::LDRSH____ => {}
            ArmOpType::LDRH_____ => {}
            ArmOpType::LDRT_____ => {}
            ArmOpType::ADCS_____ => {}
            ArmOpType::BICS_____ => {}
            ArmOpType::LDMIA____ => {}
            ArmOpType::MRC______ => {}
            ArmOpType::RSBS_____ => {}
            ArmOpType::MULS_____ => {}
            ArmOpType::AND______ => {}
            ArmOpType::MLAS_____ => {}
            ArmOpType::MUL______ => {}
            ArmOpType::SUBS_____ => {}
            ArmOpType::QDADD____ => {}
            ArmOpType::SMULL____ => {}
            ArmOpType::ANDS_____ => {}
            ArmOpType::SBC______ => {}
            ArmOpType::SUB______ => {}
            ArmOpType::SMLALS___ => {}
            ArmOpType::BX_______ => bx_execute(instruct, reg),
            ArmOpType::MOVS_____ => {}
            ArmOpType::MLA______ => {}
            ArmOpType::EORS_____ => {}
            ArmOpType::STMIA____ => {}
            ArmOpType::SMLABB___ => {}
            ArmOpType::LDMDA____ => {}
            ArmOpType::RSB______ => {}
            ArmOpType::TSTS_____ => {}
            ArmOpType::MVN______ => {}
            ArmOpType::QSUB_____ => {}
            ArmOpType::QADD_____ => {}
            ArmOpType::RSCS_____ => {}
            ArmOpType::UMLALS___ => {}
            ArmOpType::EOR______ => {}
            ArmOpType::SMLAL____ => {}
            ArmOpType::SMLATT___ => {}
            ArmOpType::SBCS_____ => {}
            ArmOpType::SMLAWB___ => {}
            ArmOpType::MVNS_____ => {}
            ArmOpType::CDP______ => {}
            ArmOpType::SMLALTB__ => {}
            ArmOpType::SMULWB___ => {}
            ArmOpType::CLZ______ => {}
            ArmOpType::LDMDB____ => {}
            ArmOpType::SMULWT___ => {}
            ArmOpType::ADDS_____ => {}
            ArmOpType::ORRS_____ => {}
            ArmOpType::SWI______ => SoftwareInterrupt::execute(reg, old_pc),
            ArmOpType::BIC______ => {}
            ArmOpType::MOV______ => {}
            ArmOpType::CMPS_____ => {}
            ArmOpType::BKPT_____ => {}
            ArmOpType::SMLABT___ => {}
            ArmOpType::UMULLS___ => {}
            ArmOpType::LDC______ => {}
            ArmOpType::SMULTB___ => {}
            ArmOpType::SWP______ => SingleDataSwap::execute(false, instruct, reg, addr_bus),
            ArmOpType::SMULBB___ => {}
            ArmOpType::SWPB_____ => SingleDataSwap::execute(true, instruct, reg, addr_bus),
            ArmOpType::UMULL____ => {}
            ArmOpType::SMLALTT__ => {}
            ArmOpType::SMULBT___ => {}
            ArmOpType::CMNS_____ => {}
            ArmOpType::SMLALBB__ => {}
            ArmOpType::ORR______ => {}
            ArmOpType::LDMIB____ => {}
            ArmOpType::TEQS_____ => {}
            ArmOpType::Undefined => {}
        }
    }


    fn cond_check(&self, cond: u8) -> bool {
        let z = self.reg.cpsr.flag_z();
        let c = self.reg.cpsr.flag_c();
        let v = self.reg.cpsr.flag_v();
        let n = self.reg.cpsr.flag_n();
        match cond {
            /* EQ */ 0b0000 => z,
            /* NE */ 0b0001 => !z,
            /* CS */ 0b0010 => c,
            /* CC */ 0b0011 => !c,
            /* MI */ 0b0100 => n,
            /* PL */ 0b0101 => !n,
            /* VS */ 0b0110 => v,
            /* VC */ 0b0111 => !v,
            /* HI */ 0b1000 => c && !z,
            // TODO not sure
            /* LS */ 0b1001 => !c || z,
            /* GE */ 0b1010 => n == v,
            /* LT */ 0b1011 => n != v,
            /* GT */ 0b1100 => !z && n == v,
            /* LE */ 0b1101 => z || n != v,
            /* AL */ 0b1110 | 0xF => true,
            _ => unreachable!(),
        }
    }
}

fn get_instruction_type(op: u32) -> ArmOpType {
    let b47 = op.extract(4, 4) as usize;
    let b2027 = op.extract(20, 8) as usize;
    (TABLE[b2027])[b47]
}
