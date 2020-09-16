use std::sync::{Arc, Mutex};

use crate::cpu::arm_op::instruct_execute::InstructExecute;
use crate::cpu::reg::Register;

struct DataProcess {}

impl InstructExecute for DataProcess {
    fn execute(reg: &mut Register) {}
}