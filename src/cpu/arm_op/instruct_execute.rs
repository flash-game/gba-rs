use crate::cpu::reg::Register;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub trait InstructExecute {
    fn execute(op: u32, reg: &mut Register);
}