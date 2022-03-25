use fantasy_util::bit::usize::BitUtil;
use crate::cpu::addrbus::AddressBus;
use crate::cpu::reg::Register;

struct SingleDataTransfer {}

impl SingleDataTransfer {
    ///  ________________________________________________________________________________
    /// |31    28|27|26|25|24|23|22|21|20|19       16|15         12|11                 0|
    /// |  cond  | 0| 1| I| P| U| B| W| L|    Rn     |      Rd     |       Offset       |
    /// ----------------|--|--|--|--|--|-------------------------------------------------
    ///                 |  |  |  |  |  └ 加载/存储 位 0=保存到内存 1=从内存加载
    ///                 |  |  |  |  └ 回写位 字节/字 0=不回写 1 = write address into base
    ///                 |  |  |  └ 字节/字 位 0=字 1=字节
    ///                 |  |  └ 加/减 位 0 = down; 从base减去偏移量 1 = up;向base增加偏移量
    ///                 |  └ 前/后 索引 bit 0=后;向后增加偏移 1=前;向前增加偏移
    ///                 └ 立即数/寄存器 bit 偏移寻址 0=立即数 1=寄存器
    /// I = 0，Offset为12bit无符号偏移量
    /// I = 1, ______________________
    ///       |11           4|3    0|
    ///       |     Shift    |  Rm  |
    ///       -------|----------|----
    ///              |          └ 偏移寄存器
    ///              └ 应用于Rm的移位
    /// ## param
    /// load 0 = store to memory  1 = load from memory
    fn execute(load: bool, instruct: u32, reg: &mut Register, addr_bus: &mut dyn AddressBus) {
        let rd = instruct.extract(12, 4) as u8;
        let rn = instruct.extract(16, 4) as u8;
        let w = instruct.get_bit_bool(21);
        let b = instruct.get_bit_bool(22);
        let u = instruct.get_bit_bool(23);
        let p = instruct.get_bit_bool(24);
        let i = instruct.get_bit_bool(25);

        let offset = if i {
            // We use the same logic here as for DataProc0
            let shift = instruct.extract(7, 5);
            let shift_type = instruct.extract(5, 2);
            let rm = instruct.extract(0, 4) as u8;
            let rm_val = reg.reg_val(rm);
            let (shifted, _) = if shift == 0 {
                let c = cpsr.get_bit(cpsr::C);
                arg_shift0(rm_val, shift_type, c)
            } else {
                arg_shift(rm_val, shift, shift_type)
            };
            shifted
        } else {
            instruct.extract(0, 12)
        };

        let base = reg.reg_val(rn).wrapping_add(((rn == reg::PC) as u32) * 4);
        let post_addr = if u {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };

        let addr = if !p { base } else { post_addr };

        if load {
            let new_rd_val = if b {
                addr_bus.get_byte(addr) as u32
            } else {
                addr_bus.get_word(addr)
            };
            reg.set_reg(rd, new_rd_val);
        } else {
            // store
            let val = reg.reg_val(rd).wrapping_add(((rd == reg::PC) as u32) * 8);
            if b {
                // TODO  force alignment of the store
                addr_bus.set_byte(addr, val as u8);
            } else {
                addr_bus.set_word(addr, val);
            };
        };

        // post-indexing implies writeback.make sure we don't overwrite rd if it was a load
        if (!p || w) && (rd != rn || !load) {
            reg.set_reg(rn, post_addr);
        }
    }

    pub fn exe2(byte: bool, load: bool, inst: u32, reg: &mut Register, addr_bus: &mut dyn AddressBus) {
        let offset_reg = inst.get_bit_bool(25);
        let p = inst.get_bit(24);
        let u = inst.get_bit(23);
        let write_back = inst.get_bit_bool(21);

        let rn = inst.extract(16, 4) as u8;
        let rd = inst.extract(12, 4) as u8;

        let offset = if offset_reg {
            // We use the same logic here as for DataProc0
            let shift = inst.extract(7, 5);
            let shift_type = inst.extract(5, 2);

            let rm = inst.extract(0, 4) as u8;

            let valm = reg.reg_val(rm);

            let (shifted, _) = if shift == 0 {
                let cpsr_c = reg.cpsr.flag_c();
                let c = cpsr.get_bit(cpsr::C);
                arg_shift0(valm, shift_type, c)
            } else {
                arg_shift(valm, shift, shift_type)
            };
            shifted
        } else {
            inst.extract(0, 12)
        };
        let base = reg.reg_val(rn).wrapping_add(((rn == 15) as u32) * 4);
        let post_addr = if u == 0 {
            base.wrapping_sub(offset)
        } else {
            base.wrapping_add(offset)
        };

        let addr = if p == 0 { base } else { post_addr };

        if !load {
            // store
            let val = reg.reg_val(rd).wrapping_add(((rd == 15) as u32) * 8);
            if !byte {
                // force alignment of the store
                addr_bus.set_word(addr, val);
            } else {
                addr_bus.set_byte(addr, val as u8);
            };
        } else {
            let a = if !byte {
                addr_bus.get_word(addr)
            } else {
                addr_bus.get_byte(addr) as u32
            };
            reg.set_reg(rd, a)
        };

        // post-indexing implies writeback
        // make sure w  e don't overwrite rd if it was a load
        if (p == 0 || write_back) && (rd != rn || !load) {
            reg.set_reg(rn, post_addr);
        }
    }
}
