use crate::cpu::reg::Register;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub trait InstructExecute {
    fn execute(reg: &mut Register);
}