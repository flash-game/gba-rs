use crate::main;

pub struct Register {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
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
    r8_fiq: u32,
    r9_fiq: u32,
    r10_fiq: u32,
    r11_fiq: u32,
    r12_fiq: u32,
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
}

impl Register {
    /// Get program counter
    pub fn get_pc(&self) -> u32 {
        // TODO Add 8 byte?

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
            Mode::User => self.r13,
            Mode::FastInterrupt => self.r13_fiq,
            Mode::Interrupt => self.r13_irq,
            Mode::Supervisor => self.r13_svc,
            Mode::Abort => self.r13_abt,
            Mode::Undefined => self.r13_und,
            Mode::System => self.r13,
        };
    }

    pub fn set_sp(&mut self, sp: u32) {
        match self.mode() {
            Mode::User => self.r13 = sp,
            Mode::FastInterrupt => self.r13_fiq = sp,
            Mode::Interrupt => self.r13_irq = sp,
            Mode::Supervisor => self.r13_svc = sp,
            Mode::Abort => self.r13_abt = sp,
            Mode::Undefined => self.r13_und = sp,
            Mode::System => self.r13 = sp,
        };
    }

    ///
    /// Get Link register
    pub fn get_lr(&self) -> u32 {
        return match self.mode() {
            Mode::User => self.r14,
            Mode::FastInterrupt => self.r14_fiq,
            Mode::Interrupt => self.r14_irq,
            Mode::Supervisor => self.r14_svc,
            Mode::Abort => self.r14_abt,
            Mode::Undefined => self.r14_und,
            Mode::System => self.r14,
        };
    }

    /// Set Link register
    pub fn set_lr(&mut self, lr: u32) {
        match self.mode() {
            Mode::User => self.r14 = lr,
            Mode::FastInterrupt => self.r14_fiq = lr,
            Mode::Interrupt => self.r14_irq = lr,
            Mode::Supervisor => self.r14_svc = lr,
            Mode::Abort => self.r14_abt = lr,
            Mode::Undefined => self.r14_und = lr,
            Mode::System => self.r14 = lr,
        };
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
    pub fn op_status(&self) -> OpStatus { if self.ctrl_t() { OpStatus::Thumb } else { OpStatus::ARM } }

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
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
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
            r8_fiq: 0,
            r9_fiq: 0,
            r10_fiq: 0,
            r11_fiq: 0,
            r12_fiq: 0,
            r13_fiq: 0,
            r14_fiq: 0,
            cspr: 0,
            spsr_svc: 0,
            spsr_abt: 0,
            spsr_und: 0,
            spsr_irq: 0,
            spsr_fiq: 0,
            mode: Mode::User,
        }
    }
}

pub enum OpStatus {
    Thumb = 1,
    ARM = 0,
}

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