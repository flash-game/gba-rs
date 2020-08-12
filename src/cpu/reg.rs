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
}

impl Register {
    /// Get program counter
    pub fn get_pc(&self) -> u32 { self.r15 }

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
        }
    }
}