use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::Register;

pub struct Arm7 {
    reg: Register,
    address: Box<dyn AddressBus>,
}