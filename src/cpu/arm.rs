use std::cell::RefCell;
use std::rc::Rc;

use fantasy_util::bit::usize::BitUtil;

use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::{Mode, OpType, Register};
use crate::util::{combine64, split64};

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
        let old_pc = self.reg.get_pc();
        let instruct = self.address_bus.borrow().get_word(self.reg.get_pc());
        self.reg.set_pc(old_pc.wrapping_add(4));
        // cond check
        if !self.cond_check((instruct >> 28) as u8) { return; }
        let instruct_type = get_instruction_type(instruct);
        match instruct_type {
            InstructionType::BranchAndExchange => {
                // DONE
            }
            InstructionType::Branch => {
                // DONE
            }
            InstructionType::SingleDataSwap => {
                // DONE
            }
            InstructionType::Multiply => {
                // DONE
            }
            InstructionType::HalfwordDataTransfer => {
                // TODO
            }
            InstructionType::MultiplyLong => {
                // DONE
            }
            InstructionType::CoprocessorDataOperation => {
                // TODO
            }
            InstructionType::CoprocessorRegisterTransfer => {
                // TODO
            }
            InstructionType::Undefined => {
                // Done
                return;
            }
            InstructionType::SoftwareInterrupt => {
                // DONE
            }
            InstructionType::BlockDataTransfer => {
                let p = instruct.get_bit_bool(24);
                let u = instruct.get_bit_bool(23);
                let s = instruct.get_bit_bool(22);
                let w = instruct.get_bit_bool(21);
                let l = instruct.get_bit_bool(20);
                let rn = instruct.extract(16, 4) as u8;
                let reglist = instruct.extract(0, 16) as u16;
                let base_addr = self.reg.reg_val(rn);
                let addr_mode = (instruct.extract(20, 5) & 0b1_1001) as u8;
                let a = |x: u32| {
                    return 1;
                };
                match addr_mode {
                    0b1_1001 => { a(1); }
                    0b0_1001 => {}
                    0b1_0001 => {}
                    0b0_0001 => {}
                    0b1_1000 => {}
                    0b0_1000 => {}
                    0b1_0000 => {}
                    0b0_0000 => {}
                    _ => {}
                }
                //  TODO
            }
            InstructionType::CoprocessorDataTransfer => {
                // TODO
            }
            InstructionType::DataProcessing => {
                // TODO
            }
            InstructionType::SingleDataTransfer => {
                let rm = instruct.extract(16, 4) as u8;
                // TODO
            }
        }
    }


    fn cond_check(&self, cond: u8) -> bool {
        let z = self.reg.cspr.flag_z();
        let c = self.reg.cspr.flag_c();
        let v = self.reg.cspr.flag_v();
        let n = self.reg.cspr.flag_n();
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

fn get_instruction_type(op: u32) -> InstructionType {
    for (select_bits, diff, instr_type) in ARM_OPCODE_TABLE.iter() {
        if ((op & select_bits) ^ diff) == 0 {
            return *instr_type;
        }
    }
    return InstructionType::Undefined;
}

#[derive(Copy, Clone)]
enum InstructionType {
    // BX
    BranchAndExchange,
    // SWP
    SingleDataSwap,
    Multiply,
    // HDRF
    HalfwordDataTransfer,
    MultiplyLong,
    CoprocessorDataOperation,
    CoprocessorRegisterTransfer,
    Undefined,
    // SWI
    SoftwareInterrupt,
    // LDM
    BlockDataTransfer,
    // B,BL
    Branch,
    // CDP  LDC, STC
    CoprocessorDataTransfer,
    // ADC,ADD,AND,BIC,CMN,CMP,EOR,MOV,MVN,ORR,RSB,RSC,SBC,SUB,TEQ,TST
    DataProcessing,
    // LDR
    SingleDataTransfer,
}

///
pub const ARM_OPCODE_TABLE: [(u32, u32, InstructionType); 15] = [
    (0x0ffffff0, 0x012fff10, InstructionType::BranchAndExchange),
    (0x0fb00ff0, 0x01000090, InstructionType::SingleDataSwap),
    (0x0fc000f0, 0x00000090, InstructionType::Multiply),
    (0x0e400f90, 0x00000090, InstructionType::HalfwordDataTransfer),
    (0x0f8000f0, 0x00800090, InstructionType::MultiplyLong),
    (0x0e400090, 0x00400090, InstructionType::HalfwordDataTransfer),
    (0x0f000010, 0x0e000000, InstructionType::CoprocessorDataOperation),
    (0x0f000010, 0x0e000010, InstructionType::CoprocessorRegisterTransfer),
    (0x0e000010, 0x06000010, InstructionType::Undefined),
    (0x0f000000, 0x0f000000, InstructionType::SoftwareInterrupt),
    (0x0e000000, 0x08000000, InstructionType::BlockDataTransfer),
    (0x0e000000, 0x0a000000, InstructionType::Branch),
    (0x0e000000, 0x0c000000, InstructionType::CoprocessorDataTransfer),
    (0x0c000000, 0x00000000, InstructionType::DataProcessing),
    (0x0c000000, 0x04000000, InstructionType::SingleDataTransfer),
];

fn get() {
    let x = [[0u8; 16]; 256];
}

