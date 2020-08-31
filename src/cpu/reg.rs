pub struct Register {
    /// Register 0 - 7
    common_reg: [u32; 8],
    /// Register 8 - 12
    high_common_reg: [u32; 5],
    /// FastInterrupt Register 8 - 12
    high_fiq_reg: [u32; 5],

    r13: u32,
    /// LR
    r14: u32,
    /// PC
    r15: u32,
    ///
    r13_svc: u32,
    r14_svc: u32,
    ///
    r13_abt: u32,
    r14_abt: u32,
    ///
    r13_und: u32,
    r14_und: u32,
    ///
    r13_irq: u32,
    r14_irq: u32,
    /// Fast Interrupt Request,FIQ 快中断模式
    r13_fiq: u32,
    r14_fiq: u32,

    ///
    cspr: u32,

    ///
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_und: u32,
    spsr_irq: u32,
    spsr_fiq: u32,

    mode: Mode,
    op_status: OpStatus,
}

impl Register {
    /// Get program counter
    pub fn get_pc(&self) -> u32 {
        self.r15
    }

    pub fn set_pc(&mut self, pc: u32) {
        if (pc & 0xFFFF_FFFC) != 0x00000000 {
            panic!(format!("Error PC value 0x{:X}", pc));
        }
        self.r15 = pc
    }

    pub fn get_sp(&self) -> u32 {
        return match self.mode() {
            Mode::User | Mode::System => self.r13,
            Mode::FastInterrupt => self.r13_fiq,
            Mode::Interrupt => self.r13_irq,
            Mode::Supervisor => self.r13_svc,
            Mode::Abort => self.r13_abt,
            Mode::Undefined => self.r13_und,
        };
    }

    pub fn set_sp(&mut self, sp: u32) {
        match self.mode() {
            Mode::User | Mode::System => self.r13 = sp,
            Mode::FastInterrupt => self.r13_fiq = sp,
            Mode::Interrupt => self.r13_irq = sp,
            Mode::Supervisor => self.r13_svc = sp,
            Mode::Abort => self.r13_abt = sp,
            Mode::Undefined => self.r13_und = sp,
        };
    }

    ///
    /// Get Link register
    pub fn get_lr(&self) -> u32 {
        return match self.mode() {
            Mode::User | Mode::System => self.r14,
            Mode::FastInterrupt => self.r14_fiq,
            Mode::Interrupt => self.r14_irq,
            Mode::Supervisor => self.r14_svc,
            Mode::Abort => self.r14_abt,
            Mode::Undefined => self.r14_und,
        };
    }

    /// Set Link register
    pub fn set_lr(&mut self, lr: u32) {
        match self.mode() {
            Mode::User | Mode::System => self.r14 = lr,
            Mode::FastInterrupt => self.r14_fiq = lr,
            Mode::Interrupt => self.r14_irq = lr,
            Mode::Supervisor => self.r14_svc = lr,
            Mode::Abort => self.r14_abt = lr,
            Mode::Undefined => self.r14_und = lr,
        };
    }

    pub fn set_reg_value(&mut self, rn: u8, val: u32) {
        match rn {
            0x0..=0x7 => self.common_reg[rn as usize] = val,
            0x8..=0xC => if self.mode == Mode::FastInterrupt {
                self.high_fiq_reg[(rn - 8) as usize] = val
            } else { self.high_common_reg[(rn - 8) as usize] = val }
            0xD => self.set_sp(val),
            0xE => self.set_lr(val),
            0xF => self.set_pc(val),
            _ => { panic!(format!("Error register value 0x{:X}", val)); }
        }
    }

    pub fn get_reg_value(&mut self, rn: u8) -> u32 {
        match rn {
            0x0..=0x7 => self.common_reg[rn as usize],
            0x8..=0xC => if self.mode == Mode::FastInterrupt {
                self.high_fiq_reg[(rn - 8) as usize]
            } else { self.high_common_reg[(rn - 8) as usize] }
            0xD => self.get_sp(),
            0xE => self.get_lr(),
            0xF => self.get_pc(),
            n => { panic!(format!("Error register number 0x{:X}", n)); }
        }
    }

    //------------------------------------flag-----------------------------------//
    /// negative 负
    pub fn flag_n(&self) -> bool { self.cspr & 0x8000_0000 != 0 }

    /// ZERO
    pub fn flag_z(&self) -> bool { self.cspr & 0x4000_0000 != 0 }

    /// Carry 进位
    pub fn flag_c(&self) -> bool { self.cspr & 0x2000_0000 != 0 }

    /// Overflow 溢出位
    pub fn flag_v(&self) -> bool { self.cspr & 0x1000_0000 != 0 }

    /// negative 负
    pub fn set_flag_n(&mut self, r: bool) {
        self.cspr = if r { self.cspr | 0x8000_0000 } else { self.cspr & 0x7FFF_FFFF }
    }

    /// ZERO
    pub fn set_flag_z(&mut self, r: bool) {
        self.cspr = if r { self.cspr | 0x4000_0000 } else { self.cspr & 0xBFFF_FFFF }
    }

    /// Carry 进位
    pub fn set_flag_c(&mut self, r: bool) {
        self.cspr = if r { self.cspr | 0x2000_0000 } else { self.cspr & 0xDFFFFFFF }
    }

    /// Overflow 溢出位
    pub fn set_flag_v(&mut self, r: bool) {
        self.cspr = if r { self.cspr | 0x1000_0000 } else { self.cspr & 0xEFFF_FFFF }
    }
//------------------------------------flag-----------------------------------//

//-----------------------------------Control---------------------------------//

    pub fn ctrl_i(&self) -> bool { self.cspr & 0x0000_0080 != 0 }

    pub fn ctrl_f(&self) -> bool { self.cspr & 0x0000_0040 != 0 }

    pub fn ctrl_t(&self) -> bool { self.cspr & 0x0000_0020 != 0 }

    /// Get the current operation status
    pub fn op_status(&self) -> &OpStatus { &self.op_status }

    /// Set the current operation status
    pub fn set_op_status(&mut self, status: OpStatus) {
        match status {
            OpStatus::Thumb => self.cspr = self.cspr | 0x0000_0020,
            OpStatus::ARM => self.cspr = self.cspr & 0xFFFF_FFDF,
        }
        self.op_status = status;
    }

    pub fn mode(&self) -> &Mode { &self.mode }

    pub fn set_mode(&mut self, mode: Mode) {
        let t: u32 = match mode {
            Mode::User => 0b10000,
            Mode::FastInterrupt => 0b10001,
            Mode::Interrupt => 0b10010,
            Mode::Supervisor => 0b10011,
            Mode::Abort => 0b10111,
            Mode::Undefined => 0b11011,
            Mode::System => 0b11111,
        };
        self.cspr = self.cspr & 0xFFFF_FFE0 | t;
        self.mode = mode;
        match self.op_status() {
            OpStatus::Thumb => {
                // TODO
            }
            OpStatus::ARM => {
                // TODO
            }
        }
    }

//-----------------------------------Control---------------------------------//


    pub fn new() -> Self {
        Self {
            common_reg: [0u32; 8],
            high_common_reg: [0u32; 5],
            high_fiq_reg: [0u32; 5],
            r13: 0,
            r14: 0,
            r15: 0,
            r13_svc: 0,
            r14_svc: 0,
            r13_abt: 0,
            r14_abt: 0,
            r13_und: 0,
            r14_und: 0,
            r13_irq: 0,
            r14_irq: 0,
            r13_fiq: 0,
            r14_fiq: 0,
            cspr: 0,
            spsr_svc: 0,
            spsr_abt: 0,
            spsr_und: 0,
            spsr_irq: 0,
            spsr_fiq: 0,
            mode: Mode::User,
            op_status: OpStatus::ARM,
        }
    }
}

pub enum OpStatus {
    Thumb = 1,
    ARM = 0,
}

#[derive(PartialEq)]
pub enum Mode {
    User = 10000,
    /// 快中断
    FastInterrupt = 10001,
    /// 中断
    Interrupt = 10010,
    /// 管理
    Supervisor = 10011,
    /// 中止
    Abort = 10111,
    Undefined = 11011,
    System = 11111,
}