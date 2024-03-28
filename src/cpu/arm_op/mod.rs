use std::ops::Shl;

use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;

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
mod psr_transfer;

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
/// 当指令中指定移位量时，它包含在一个 5 位字段中，可以取 0 到 31 之间的任何值。
/// 逻辑左移 (LSL) 取 Rm 的内容并将每个位移动指定的量到更高位。结果的最低位用零填充，Rm 中没有映射到结果的高位被丢弃，除了最低位丢弃的位成为移位器进位输出，可以锁存到 C 位 当 ALU 操作在逻辑类中时的 CPSR（见上文）。
/// 例如，LSL#5 的效果如图 4-6：逻辑左移
/// ###
/// LSL #0 是一种特殊情况，其中移位器进位是 CPSR C 标志的旧值。 Rm 的内容直接用作第二个操作数。
/// 逻辑右移 (LSR) 类似，但 Rm 的内容在结果中移动到不太重要的位置。 LSR #5 的效果如➲图 4-7：逻辑右移
/// ###
/// 预期对应于 LSR #0 的移位字段的形式用于编码 LSR #32，其结果为零，Rm 的第 31 位作为进位输出。
/// 逻辑右移零是多余的，因为它与逻辑左移零相同，因此汇编器会将 LSR #0（以及 ASR #0 和 ROR #0）转换为 LSL #0，并允许指定 LSR #32。
/// 算术右移 (ASR) 类似于逻辑右移，不同之处在于高位用 Rm 的第 31 位而不是零填充。 这保留了 2 的补码表示法中的符号。 例如，ASR #5 显示在➲图 4-8：算术右移。
///
/// ###
/// 预期会给出 ASR #0 的移位字段的形式用于编码 ASR #32。
/// Rm 的第 31 位再次用作进位输出，操作数 2 的每一位也等于 Rm 的第 31 位。
/// 因此，根据 Rm 的第 31 位的值，结果是全 1 或全 0。
///
/// 循环右移 (ROR) 操作通过在结果的高位重新引入它们来重用逻辑右移操作中“过冲”的位，而不是用于填充逻辑右操作中高位的零。
/// 例如，ROR #5 显示在➲图 4-9：第 4-14 页右转。
///
/// ###
/// 可能会给出 ROR #0 的移位字段的形式用于编码桶形移位器的特殊功能，即右旋转扩展 (RRX)。
/// 这是将 CPSR C 标志附加到 Rm 内容的最高有效端形成的 33 位量的右移一位位置，如 ➲ 图 4-10：右移扩展
pub fn barrel_shifter(instruct: u32, reg: &mut Register) -> (u32, bool) {
    let rm = (instruct & 0b1111) as u8;
    let rm_val = reg.reg_val(rm);
    let shift_type = instruct.extract(5, 2) as u8;
    let shift_amount = if instruct.get_bit_bool(4) {
        let rs = instruct.extract(8, 4) as u8;
        reg.reg_val(rs) & 0b11111
    } else {
        instruct.extract(7, 5)
    };
    match shift_type {
        0b00 => logical_left(rm_val, shift_amount, reg),
        0b01 => logical_right(rm_val, shift_amount),
        0b10 => arithmetic_shift_right(rm_val, shift_amount),
        0b11 => rotate_right(rm_val, shift_amount, reg),
        _ => unreachable!(),
    }
}

/// 逻辑左移
fn logical_left(rm_val: u32, shift_amount: u32, reg: &mut Register) -> (u32, bool) {
    match shift_amount {
        0 => (rm_val, reg.cpsr.flag_c()),
        _ => (rm_val << shift_amount, ((rm_val >> (32 - shift_amount)) & 0b1) == 1),
    }
}

/// 逻辑右移
fn logical_right(rm_val: u32, shift_amount: u32) -> (u32, bool) {
    match shift_amount {
        0 => (0, rm_val.get_bit_bool(31)),
        _ => (rm_val >> shift_amount, ((rm_val >> (shift_amount - 1)) & 0b1) == 1),
    }
}

/// 算术右移
fn arithmetic_shift_right(rm_val: u32, shift_amount: u32) -> (u32, bool) {
    match shift_amount {
        0 => match rm_val.get_bit_bool(31) {
            true => (u32::MAX, true),
            false => (0, false),
        },
        _ => (
            ((rm_val as i32) >> shift_amount) as u32,
            ((rm_val >> (shift_amount - 1)) & 0b1) == 1,
        ),
    }
}

/// 循环右移
fn rotate_right(rm_val: u32, shift_amount: u32, reg: &mut Register) -> (u32, bool) {
    match shift_amount {
        0 => (
            (rm_val >> 1) | (if reg.cpsr.flag_c() { 1u32 << 31 } else { 0 }),
            rm_val & 1 == 1,
        ),
        _ => (rm_val.rotate_right(shift_amount), ((rm_val >> (shift_amount - 1)) & 1) == 1),
    }
}
