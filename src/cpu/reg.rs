use minifb::Error;

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

    pub cspr: CPSR,
    ///
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_und: u32,
    spsr_irq: u32,
    spsr_fiq: u32,

}

impl Register {
    /// Get program counter
    pub fn get_pc(&self) -> u32 {
        self.r15
    }

    pub fn set_pc(&mut self, pc: u32) {
        if (pc & 0xFFFF_FFFC) != 0x0000_0000 {
            panic!(format!("Error PC value 0x{:X}", pc));
        }
        self.r15 = pc
    }

    pub fn get_sp(&self) -> u32 {
        return match self.cspr.mode() {
            Mode::User | Mode::System => self.r13,
            Mode::FastInterrupt => self.r13_fiq,
            Mode::Interrupt => self.r13_irq,
            Mode::Supervisor => self.r13_svc,
            Mode::Abort => self.r13_abt,
            Mode::Undefined => self.r13_und,
        };
    }

    pub fn set_sp(&mut self, sp: u32) {
        match self.cspr.mode() {
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
        return match self.cspr.mode() {
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
        match self.cspr.mode() {
            Mode::User | Mode::System => self.r14 = lr,
            Mode::FastInterrupt => self.r14_fiq = lr,
            Mode::Interrupt => self.r14_irq = lr,
            Mode::Supervisor => self.r14_svc = lr,
            Mode::Abort => self.r14_abt = lr,
            Mode::Undefined => self.r14_und = lr,
        };
    }

    /// Set Register value with rn number
    /// Such as : rn=0 val=0xFFFF , r0=0xFFFF.
    pub fn set_reg(&mut self, rn: u8, val: u32) {
        match rn {
            0x0..=0x7 => self.common_reg[rn as usize] = val,
            0x8..=0xC => if self.cspr.mode == Mode::FastInterrupt {
                self.high_fiq_reg[(rn - 8) as usize] = val
            } else { self.high_common_reg[(rn - 8) as usize] = val }
            0xD => self.set_sp(val),
            0xE => self.set_lr(val),
            0xF => self.set_pc(val),
            _ => { panic!(format!("Error register value 0x{:X}", val)); }
        }
    }

    /// Get a Register value with rn number
    pub fn reg_val(&mut self, rn: u8) -> u32 {
        match rn {
            0x0..=0x7 => self.common_reg[rn as usize],
            0x8..=0xC => if self.cspr.mode == Mode::FastInterrupt {
                self.high_fiq_reg[(rn - 8) as usize]
            } else { self.high_common_reg[(rn - 8) as usize] }
            0xD => self.get_sp(),
            0xE => self.get_lr(),
            0xF => self.get_pc(),
            n => { panic!(format!("Error register number 0x{:X}", n)); }
        }
    }


//-----------------------------------Control---------------------------------//


    pub fn spsr(&self) -> u32 {
        match self.cspr.mode() {
            Mode::FastInterrupt => self.spsr_fiq,
            Mode::Interrupt => self.spsr_irq,
            Mode::Supervisor => self.spsr_svc,
            Mode::Abort => self.spsr_abt,
            Mode::Undefined => self.spsr_und,
            n => panic!(format!("Unsupport spsr mode {:?}", n)),
        }
    }

    pub fn set_spsr(&mut self, spsr_val: u32) {
        match self.cspr.mode() {
            Mode::FastInterrupt => self.spsr_fiq = spsr_val,
            Mode::Interrupt => self.spsr_irq = spsr_val,
            Mode::Supervisor => self.spsr_svc = spsr_val,
            Mode::Abort => self.spsr_abt = spsr_val,
            Mode::Undefined => self.spsr_und = spsr_val,
            n => println!("Unsupport spsr mode {:?}", n),
        };
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
            cspr: CPSR::new(),
            spsr_svc: 0,
            spsr_abt: 0,
            spsr_und: 0,
            spsr_irq: 0,
            spsr_fiq: 0,
        }
    }
}

pub enum OpType {
    Thumb = 1,
    ARM = 0,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    User = 0b10000,
    /// 快中断
    FastInterrupt = 0b10001,
    /// 中断
    Interrupt = 0b10010,
    /// 管理
    Supervisor = 0b10011,
    /// 中止
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl From<u8> for Mode {
    fn from(mode_u8: u8) -> Self {
        let option = None;
        let result = option.ok_or(Err(None));
        match mode_u8 & 0b0001_1111 {
            0b10000 => Mode::User,
            0b10001 => Mode::FastInterrupt,
            0b10010 => Mode::Interrupt,
            0b10011 => Mode::Supervisor,
            0b10111 => Mode::Abort,
            0b11011 => Mode::Undefined,
            0b11111 => Mode::System,
            n => panic!(format!("Unknow mode 0x{:X}", n))
        }
    }
}

/// CSPR struct
pub struct CPSR {
    ///
    cpsr_val: u32,

    mode: Mode,
}

impl CPSR {
    pub fn value(&self) -> u32 { self.cpsr_val }

    pub fn set_val(&mut self, cspr_val: u32) {
        self.cpsr_val = cspr_val;
        let mode: Mode = ((cspr_val & 0x1F) as u8).into();
        self.set_mode(mode);
    }

    /// negative 负
    pub fn flag_n(&self) -> bool { self.cpsr_val & 0x8000_0000 != 0 }

    /// ZERO
    pub fn flag_z(&self) -> bool { self.cpsr_val & 0x4000_0000 != 0 }

    /// Carry 进位
    pub fn flag_c(&self) -> bool { self.cpsr_val & 0x2000_0000 != 0 }

    /// Overflow 溢出位
    pub fn flag_v(&self) -> bool { self.cpsr_val & 0x1000_0000 != 0 }

    /// negative 负
    pub fn set_flag_n(&mut self, r: bool) {
        self.cpsr_val = if r { self.cpsr_val | 0x8000_0000 } else { self.cpsr_val & 0x7FFF_FFFF }
    }

    /// ZERO
    pub fn set_flag_z(&mut self, r: bool) {
        self.cpsr_val = if r { self.cpsr_val | 0x4000_0000 } else { self.cpsr_val & 0xBFFF_FFFF }
    }

    /// Carry 进位
    pub fn set_flag_c(&mut self, r: bool) {
        self.cpsr_val = if r { self.cpsr_val | 0x2000_0000 } else { self.cpsr_val & 0xDFFFFFFF }
    }

    /// Overflow 溢出位
    pub fn set_flag_v(&mut self, r: bool) {
        self.cpsr_val = if r { self.cpsr_val | 0x1000_0000 } else { self.cpsr_val & 0xEFFF_FFFF }
    }

    /// Set CSPR Flag value , include flag_n/flag_z/flag_c/flag_v
    pub fn set_flag_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.set_flag_n(n);
        self.set_flag_z(z);
        self.set_flag_c(c);
        self.set_flag_v(v);
    }

    pub fn irq_disable(&self) -> bool { self.cpsr_val & 0x0000_0080 != 0 }

    pub fn set_irq_disable(&mut self, disable: bool) {
        self.cpsr_val = if disable { self.cpsr_val | 0x80 } else { self.cpsr_val & 0xFFFFFF7F }
    }

    pub fn fiq_disable(&self) -> bool { self.cpsr_val & 0x0000_0040 != 0 }

    pub fn set_fiq_disable(&mut self, disable: bool) {
        self.cpsr_val = if disable { self.cpsr_val | 0x40 } else { self.cpsr_val & 0xFFFFFFBF }
    }

    /// Set the current operation status
    pub fn set_op_type(&mut self, status: OpType) {
        match status {
            OpType::Thumb => self.cpsr_val = self.cpsr_val | 0x0000_0020,
            OpType::ARM => self.cpsr_val = self.cpsr_val & 0xFFFF_FFDF,
        }
    }

    pub fn mode(&self) -> &Mode { &self.mode }

    pub fn set_mode(&mut self, mode: Mode) {
        let mode_u32 = (mode as u32).clone();
        self.cpsr_val = self.cpsr_val & 0xFFFF_FFE0 | mode_u32;
        self.mode = mode;
        match self.op_status() {
            OpType::Thumb => {
                // TODO
            }
            OpType::ARM => {
                // TODO
            }
        }
    }

    /// Get the current operation status
    pub fn op_status(&self) -> OpType {
        if (self.cpsr_val & 0x20) != 0 { OpType::Thumb } else { OpType::ARM }
    }

    pub fn new() -> Self {
        Self { cpsr_val: 0, mode: Mode::User }
    }
}