use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::Register;
use crate::util::BitUtilExt;

pub struct Arm7 {
    reg: Register,
    address_bus: Box<dyn AddressBus>,
}

impl Arm7 {
    pub fn next(&mut self) {
        let old_pc = self.reg.get_pc();
        let op = self.address_bus.get_word(self.reg.get_pc());
        self.reg.set_pc(old_pc.wrapping_add(4));
        // cond check
        if !self.cond_check((op >> 28) as u8) { return; }
        let instruct_type = get_instruction_type(op);
        match instruct_type {
            InstructionType::BranchAndExchange => {
                let rn = op.extract(0, 4) as u8;
                ();
            }
            InstructionType::Branch => { () }
            InstructionType::SingleDataSwap => { () }
            InstructionType::Multiply => { () }
            InstructionType::HalfwordDataTransfer => { () }
            InstructionType::MultiplyLong => { () }
            InstructionType::CoprocessorDataOperation => { () }
            InstructionType::CoprocessorRegisterTransfer => { () }
            InstructionType::Undefined => { () }
            InstructionType::SoftwareInterrupt => { () }
            InstructionType::BlockDataTransfer => { () }
            InstructionType::CoprocessorDataTransfer => { () }
            InstructionType::DataProcessing => { () }
            InstructionType::SingleDataTransfer => { () }
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
    /**/ BranchAndExchange,
    /**/ SingleDataSwap,
    /**/ Multiply,
    HalfwordDataTransfer,
    /**/ MultiplyLong,
    /**/ CoprocessorDataOperation,
    /**/ CoprocessorRegisterTransfer,
    /**/ Undefined,
    /**/ SoftwareInterrupt,
    /**/ BlockDataTransfer,
    /**/ Branch,
    /**/ CoprocessorDataTransfer,
    /**/ DataProcessing,
    /**/ SingleDataTransfer,
}

///
const ARM_OPCODE_TABLE: [(u32, u32, InstructionType); 15] = [
    // Branch and Exchange
    (0x0ffffff0, 0x012fff10, InstructionType::BranchAndExchange),
    // Single Data Swap
    (0x0fb00ff0, 0x01000090, InstructionType::SingleDataSwap),
    // Multiply
    (0x0fc000f0, 0x00000090, InstructionType::Multiply),
    // Halfword Data Transfer (register offset)
    (0x0e400f90, 0x00000090, InstructionType::HalfwordDataTransfer),
    // Multiply Long
    (0x0f8000f0, 0x00800090, InstructionType::MultiplyLong),
    // Halfword Data Transfer (immediate offset)
    (0x0e400090, 0x00400090, InstructionType::HalfwordDataTransfer),
    // Coprocessor Data Operation
    (0x0f000010, 0x0e000000, InstructionType::CoprocessorDataOperation),
    // Coprocessor Register Transfer
    (0x0f000010, 0x0e000010, InstructionType::CoprocessorRegisterTransfer),
    // Undefined
    (0x0e000010, 0x06000010, InstructionType::Undefined),
    // Software Interrupt
    (0x0f000000, 0x0f000000, InstructionType::SoftwareInterrupt),
    // Block Data Transfer
    (0x0e000000, 0x08000000, InstructionType::BlockDataTransfer),
    // Branch
    (0x0e000000, 0x0a000000, InstructionType::Branch),
    // Coprocessor Data Transfer
    (0x0e000000, 0x0c000000, InstructionType::CoprocessorDataTransfer),
    // Data Processing / PSR Transfer
    (0x0c000000, 0x00000000, InstructionType::DataProcessing),
    // Single Data Transfer
    (0x0c000000, 0x04000000, InstructionType::SingleDataTransfer),
];