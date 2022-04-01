use crate::cpu::reg::Register;
use fantasy_util::bit::usize::BitUtil;
use std::ops::Shl;

pub mod branch;
pub mod branch_exchange;
pub mod cdp;
pub mod cdt;
pub mod data_process;
pub mod instruct_execute;
pub mod multiply;
pub mod multiply_long;
pub mod singledata_transfer;
pub mod software_interrupt;
pub mod swp;
pub mod undefined;

/// 桶形移位器
/// ```
/// _______________________
/// |11           4|3    0|
/// |     Shift    |  Rm  |
/// -------|----------|----
///        |          └ 偏移寄存器
///        └ 应用于Rm的移位
/// ```
///
/// ## DOC
/// 当第二个操作数指定为移位寄存器时，桶形移位器的操作由指令中的 Shift 域控制。
/// 此字段指示要执行的移位类型（逻辑左移或右移、算术右移或右移）。
/// 寄存器应移位的量可以包含在指令的立即数字段中，或包含在另一个寄存器的底部字节中（R15 除外）。
/// ```
/// ______________
/// |11    7|65|4|
/// |       |  |0|
/// ---|-----|----
///    |     └ 移位类型
///    └ 移位数
///
/// ______________
/// |11  8|7|65|4|
/// |  Rs |0|  |1|
/// ---|-----|----
///    |     └ 移位类型
///    └ 存移位数的寄存器
/// ```
///
/// ### 指令指定移位量
/// 当指令中指定移位量时，它包含在一个 5 位字段中，可以取 0 到 31 之间的任何值。逻辑左移 (LSL) 取 Rm 的内容并将每个位移动指定的量到 更显着的地位。
/// 结果的最低位用零填充，Rm 中没有映射到结果的高位被丢弃，除了最低位丢弃的位成为移位器进位输出，可以锁存到 C 位 当 ALU 操作在逻辑类中时的 CPSR（见上文）。
/// 例如，LSL#5 的效果如图 4-6：逻辑左移

pub fn barrel_shifter(instruct: u32, reg: &mut Register) {
    let rm = (instruct & 0b1111) as u8;
    let rm_val = reg.reg_val(rm);
    let shift_type = instruct.extract(5, 2) as u8;
    let shift_amount = if instruct.get_bit_bool(4) {
        let rs = instruct.extract(8, 4) as u8;
        reg.reg_val(rs) & 0b11111
    } else {
        instruct.extract(7, 5)
    };
    let i = match shift_type {
        0b00 => rm_val << shift_amount,
        0b01 => rm_val >> shift_amount,
        0b10 => ((rm_val as i32) >> shift_amount) as u32,
        0b11 => rm_val.rotate_right(shift_amount),
        _ => unreachable!(),
    };
}
