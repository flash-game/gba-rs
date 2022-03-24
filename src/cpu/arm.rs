use fantasy_util::bit::usize::BitUtil;

use crate::cpu::addrbus::AddressBus;
use crate::cpu::arm_op::branch::Branch;
use crate::cpu::arm_op::branch_exchange::bx_execute;
use crate::cpu::arm_op::cdt::CoprocessorDataTransfers;
use crate::cpu::arm_op::data_process::DataProcess;
use crate::cpu::arm_op::multiply::Multiply;
use crate::cpu::arm_op::multiply_long::MultiplyLong;
use crate::cpu::arm_op::software_interrupt::SoftwareInterrupt;
use crate::cpu::arm_op::swp::SingleDataSwap;
use crate::cpu::arm_op::undefined::Undefined;
use crate::cpu::arm_op_table::{ArmOpType, TABLE};
use crate::cpu::mem::Memory;
use crate::cpu::reg::Register;

pub struct Arm7<'a> {
    reg: &'a mut Register,
    address_bus: &'a mut dyn AddressBus,
}

impl<'a> Arm7<'a> {
    pub fn new(reg: &'a mut Register, address_bus: &'a mut dyn AddressBus) -> Self {
        Self { reg, address_bus }
    }

    pub fn next(&mut self) {
        let old_pc = self.reg.get_pc();
        let instruct = self.address_bus.get_word(self.reg.get_pc());
        self.reg.set_pc(old_pc.wrapping_add(4));
        // cond check
        if !self.cond_check((instruct >> 28) as u8) {
            return;
        }
        let instruct_type = get_instruction_type(instruct);
        match instruct_type {
            ArmOpType::STMDA__ => {}
            ArmOpType::STC____ => CoprocessorDataTransfers::execute(false, instruct, self.reg),
            ArmOpType::MCR____ => {}
            ArmOpType::UMLAL__ => MultiplyLong::execute(false, true, false, instruct, self.reg),
            ArmOpType::SMULLS_ => MultiplyLong::execute(true, false, true, instruct, self.reg),
            ArmOpType::RSC____ => {}
            ArmOpType::ADD____ => DataProcess::add(false, instruct, self.reg),
            ArmOpType::MRS____ => {}
            ArmOpType::MSR____ => {}
            ArmOpType::STMDB__ => {}
            ArmOpType::STMIB__ => {}
            ArmOpType::ADC____ => {}
            ArmOpType::B______ => Branch::execute(instruct, self.reg, old_pc, false),
            ArmOpType::BL_____ => Branch::execute(instruct, self.reg, old_pc, true),
            ArmOpType::STR____ => {}
            ArmOpType::STRT___ => {}
            ArmOpType::STRB___ => {}
            ArmOpType::STRBT__ => {}
            ArmOpType::STRH___ => {}
            ArmOpType::LDR____ => {}
            ArmOpType::LDRSB__ => {}
            ArmOpType::LDRB___ => {}
            ArmOpType::LDRBT__ => {}
            ArmOpType::LDRSH__ => {}
            ArmOpType::LDRH___ => {}
            ArmOpType::LDRT___ => {}
            ArmOpType::ADCS___ => {}
            ArmOpType::BICS___ => {}
            ArmOpType::LDMIA__ => {}
            ArmOpType::MRC____ => {}
            ArmOpType::RSBS___ => {}
            ArmOpType::MULS___ => Multiply::execute(true, false, instruct, self.reg),
            ArmOpType::AND____ => {}
            ArmOpType::MLAS___ => Multiply::execute(true, true, instruct, self.reg),
            ArmOpType::MUL____ => Multiply::execute(false, false, instruct, self.reg),
            ArmOpType::SUBS___ => {}
            ArmOpType::SMULL__ => MultiplyLong::execute(false, false, true, instruct, self.reg),
            ArmOpType::ANDS___ => {}
            ArmOpType::SBC____ => {}
            ArmOpType::SUB____ => {}
            ArmOpType::SMLALS_ => MultiplyLong::execute(true, true, true, instruct, self.reg),
            ArmOpType::BX_____ => bx_execute(instruct, self.reg),
            ArmOpType::MOVS___ => {}
            ArmOpType::MLA____ => Multiply::execute(false, true, instruct, self.reg),
            ArmOpType::EORS___ => {}
            ArmOpType::STMIA__ => {}
            ArmOpType::LDMDA__ => {}
            ArmOpType::RSB____ => {}
            ArmOpType::TSTS___ => DataProcess::tst(instruct, self.reg),
            ArmOpType::MVN____ => {}
            ArmOpType::RSCS___ => {}
            ArmOpType::UMLALS_ => MultiplyLong::execute(true, true, false, instruct, self.reg),
            ArmOpType::EOR____ => {}
            ArmOpType::SMLAL__ => MultiplyLong::execute(false, true, true, instruct, self.reg),
            ArmOpType::SBCS___ => {}
            ArmOpType::MVNS___ => {}
            ArmOpType::CDP____ => {}
            ArmOpType::LDMDB__ => {}
            ArmOpType::ADDS___ => DataProcess::add(true, instruct, self.reg),
            ArmOpType::ORRS___ => {}
            ArmOpType::SWI____ => SoftwareInterrupt::execute(self.reg, old_pc),
            ArmOpType::BIC____ => {}
            ArmOpType::MOV____ => {}
            ArmOpType::CMPS___ => DataProcess::cmp(instruct, self.reg),
            ArmOpType::UMULLS_ => MultiplyLong::execute(true, false, false, instruct, self.reg),
            ArmOpType::LDC____ => CoprocessorDataTransfers::execute(true, instruct, self.reg),
            ArmOpType::SWP____ => SingleDataSwap::execute(false, instruct, self.reg, self.address_bus),
            ArmOpType::SWPB___ => SingleDataSwap::execute(true, instruct, self.reg, self.address_bus),
            ArmOpType::UMULL__ => MultiplyLong::execute(false, false, false, instruct, self.reg),
            ArmOpType::CMNS___ => DataProcess::tst(instruct, self.reg),
            ArmOpType::ORR____ => {}
            ArmOpType::LDMIB__ => {}
            ArmOpType::TEQS___ => DataProcess::teq(instruct, self.reg),
            ArmOpType::Undef__ => Undefined::execute(instruct, self.reg),
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
