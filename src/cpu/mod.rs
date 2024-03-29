use std::sync::Arc;
use crate::cpu::addrbus::AddressBus;
use crate::cpu::arm::Arm7;
use crate::cpu::reg::Register;
use crate::cpu::thumb::Thumb;

mod addrbus;
mod arm;
mod arm_op;
mod arm_op_map;
pub mod mem;
mod reg;
mod thumb_op_map;
mod thumb;

pub struct CPU<'a> {
    reg: Arc<Register>,
    address_bus: &'a mut dyn AddressBus,
    arm: Arm7<'a>,
    thumb: Thumb<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(address_bus: &'a mut dyn AddressBus) -> Self {
        let mut reg = Arc::new(Register::new());
        reg.
        let arm = Arm7::new(&mut reg.br, address_bus);
        let thumb = Thumb::new(&mut reg.clone(), address_bus);
        Self { reg, address_bus, arm, thumb }
    }
}