use crate::cpu::selector::InstructType::Branch;

struct OpSelector {}

impl OpSelector {}


fn select(instruction: u32) {}

enum InstructType {
    DataProcessing,
    Multiply,

    Branch,

}

const CODE: [[InstructType; 16]; 2] = [
    /* 00 0000_0000 */
    [Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch],
    [Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch, Branch],
];