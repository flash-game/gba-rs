use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::Register;

pub struct Thumb<'a> {
    reg: &'a mut Register,
    address_bus: &'a mut dyn AddressBus,
}

impl<'a> Thumb<'a> {
    pub fn new(reg: &'a mut Register, address_bus: &'a mut dyn AddressBus) -> Self {
        Self { reg, address_bus }
    }
}