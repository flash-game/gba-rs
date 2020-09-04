use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::{OpType, Register};
use crate::util::{BitUtilExt, combine64, split64};

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
        let op = self.address_bus.borrow().get_word(self.reg.get_pc());
        self.reg.set_pc(old_pc.wrapping_add(4));
        // cond check
        if !self.cond_check((op >> 28) as u8) { return; }
        let instruct_type = get_instruction_type(op);
        match instruct_type {
            InstructionType::BranchAndExchange => {
                let rn = op.extract(0, 4) as u8;
                let new_pc = self.reg.reg_val(rn);
                self.reg.set_pc(new_pc & !1u32);
                // SET ARM or THUMB
                self.reg.set_op_type(if new_pc & 0x1 == 0 { OpType::ARM } else { OpType::Thumb });
            }
            InstructionType::Branch => {
                let offset = op.extract(0, 24);
                let s_offset = ((offset << 8) as i32 >> 6) as u32;
                self.reg.set_pc(old_pc.wrapping_add(s_offset).wrapping_add(8));
                if op & 0x0100_0000 != 0 {
                    self.reg.set_lr(old_pc.wrapping_add(4));
                }
            }
            InstructionType::SingleDataSwap => {
                let rm = op.extract(0, 4) as u8;
                let rd = op.extract(12, 4) as u8;
                let rn = op.extract(16, 4) as u8;
                let b = op.get_bit_bool(22);
                // If it is not a byte operation then force word align
                let addr = self.reg.reg_val(rn) & !((1 - if b { 1 } else { 2 }) * 3);
                let addr_mut = self.address_bus.borrow_mut();
                let addr_val = if b { addr_mut.get_byte(addr) as u32 } else { addr_mut.get_word(addr) };
                let reg_val = self.reg.reg_val(rm);
                if b {
                    addr_mut.set_byte(addr, reg_val as u8)
                } else {
                    addr_mut.set_word(addr, reg_val)
                }
                self.reg.set_reg(rd, addr_val);
            }
            InstructionType::Multiply => {
                let rm = op.extract(0, 4);
                let rs = op.extract(8, 4);
                let rn = op.extract(12, 4);
                let rd = op.extract(16, 4);
                let a = (op & 0x0020_0000) != 0;
                let s = (op & 0x0010_0000) != 0;
                let result = self.reg.reg_val(rm as u8)
                    .wrapping_mul(self.reg.reg_val(rs as u8))
                    .wrapping_add(if a { 0 } else { self.reg.reg_val(rn as u8) });
                self.reg.set_reg(rd as u8, result);
                // 如果需要更改flag
                if s {
                    let new_n = (result & 0x8000_0000) != 0;
                    self.reg.set_flag_nzcv(new_n, result == 0, false, self.reg.flag_v());
                }
            }
            InstructionType::HalfwordDataTransfer => {
                // TODO
            }
            InstructionType::MultiplyLong => {
                let u = (op & 0x0040_0000) != 0;
                let a = (op & 0x0020_0000) != 0;
                let s = (op & 0x0010_0000) != 0;
                let rdhi = op.extract(16, 4) as u8;
                let rdlo = op.extract(12, 4) as u8;
                let rs = op.extract(8, 4);
                let rm = op.extract(0, 4);
                let vs_u = self.reg.reg_val(rs as u8);
                let vm_u = self.reg.reg_val(rm as u8);
                let result: u64 = if !u {
                    let prod = (vs_u as u64) * (vm_u as u64);
                    prod.wrapping_add(if !a { 0u64 } else {
                        combine64(self.reg.reg_val(rdhi), self.reg.reg_val(rdlo))
                    })
                } else {
                    let prod = (vs_u as i64) * (vm_u as i64);
                    prod.wrapping_add(if !a { 0i64 } else {
                        combine64(self.reg.reg_val(rdhi), self.reg.reg_val(rdlo)) as i64
                    }) as u64
                };
                let (reshi, reslo) = split64(result);
                self.reg.set_reg(rdhi, reshi);
                self.reg.set_reg(rdlo, reslo);
                if s {
                    let new_n = (reshi & 0x8000_0000) != 0;
                    self.reg.set_flag_nzcv(new_n, result == 0, false, false);
                }
            }
            InstructionType::CoprocessorDataOperation => { () }
            InstructionType::CoprocessorRegisterTransfer => { () }
            InstructionType::Undefined => { () }
            InstructionType::SoftwareInterrupt => { () }
            InstructionType::BlockDataTransfer => {
                // TODO
            }
            InstructionType::CoprocessorDataTransfer => { () }
            InstructionType::DataProcessing => { () }
            InstructionType::SingleDataTransfer => {}
        }
    }


    fn cond_check(&self, cond: u8) -> bool {
        let z = self.reg.flag_z();
        let c = self.reg.flag_c();
        let v = self.reg.flag_v();
        let n = self.reg.flag_n();
        match cond {
            /* EQ */ 0x0 => z,
            /* NE */ 0x1 => !z,
            /* CS */ 0x2 => c,
            /* CC */ 0x3 => !c,
            /* MI */ 0x4 => n,
            /* PL */ 0x5 => !n,
            /* VS */ 0x6 => v,
            /* VC */ 0x7 => !v,
            /* HI */ 0x8 => c && !z,
            /* LS */ 0x9 => !c || z,
            /* GE */ 0xA => n == v,
            /* LT */ 0xB => n != v,
            /* GT */ 0xC => !z && n == v,
            /* LE */ 0xD => z || n != v,
            /* AL */ 0xE | 0xF => true,
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
    BranchAndExchange,
    SingleDataSwap,
    Multiply,
    HalfwordDataTransfer,
    MultiplyLong,
    CoprocessorDataOperation,
    CoprocessorRegisterTransfer,
    Undefined,
    SoftwareInterrupt,
    BlockDataTransfer,
    Branch,
    CoprocessorDataTransfer,
    DataProcessing,
    SingleDataTransfer,
}

///
const ARM_OPCODE_TABLE: [(u32, u32, InstructionType); 15] = [
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