use crate::cpu::reg::Register;

pub trait InstructExecute {
    fn execute(op: u32, reg: &mut Register);
}
