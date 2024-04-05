use std::cell::RefCell;
use std::rc::Rc;

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
    reg: Rc<RefCell<Register>>,
    address_bus: Rc<RefCell<dyn AddressBus>>,
    arm: Arm7<'a>,
    thumb: Thumb<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(address_bus: Rc<RefCell<dyn AddressBus>>) -> Self {
        let reg = Rc::new(RefCell::new(Register::new()));
        let x = address_bus.clone().get_mut();
        let arm = Arm7::new(&mut reg.clone().get_mut(), &mut address_bus.clone().get_mut());
        let thumb = Thumb::new(&mut reg.clone().borrow_mut(), &mut address_bus.clone().borrow_mut());
        Self { reg: reg.clone(), address_bus, arm, thumb }
    }
}