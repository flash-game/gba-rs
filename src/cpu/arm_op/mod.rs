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

///  ______________________
/// |11           4|3    0|
/// |     Shift    |  Rm  |
/// -------|----------|----
///        |          └ 偏移寄存器
///        └ 应用于Rm的移位
///
///  _____________
/// |11    7|65|4|
/// |       |  |0|
/// ---|-----|----
///    |     └ 移位类型
///    └ 移位数
///
///  _____________
/// |11  8|7|65|4|
/// |  Rs |0|  |1|
/// ---|-----|----
///    |     └ 移位类型
///    └ 存移位数的寄存器
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