use std::convert::TryInto;

use crate::cpu::mem::Memory;

pub struct GbaAddressBus {
    // BIOS - System ROM         (16 KBytes)
    bios: RoAddr,
    // WRAM - On-board Work RAM  (256 KBytes) 2 Wait
    board_ram: RwAddr,
    // WRAM - On-chip Work RAM   (32 KBytes)
    chip_ram: RwAddr,
    // I/O Registers
    io_reg: RwAddr,
    // BG/OBJ Palette RAM        (1 Kbyte)
    palette_ram: RwAddr,
    // VRAM - Video RAM          (96 KBytes)
    video_ram: RwAddr,
    // OAM - OBJ Attributes      (1 Kbyte)
    obj_attr: RwAddr,
    // Game Pak ROM/FlashROM (max 32MB) - Wait State 0
    rom_0: RoAddr,
    // Game Pak ROM/FlashROM (max 32MB) - Wait State 1
    rom_1: RoAddr,
    // Game Pak ROM/FlashROM (max 32MB) - Wait State 2
    rom_2: RoAddr,
    // Game Pak SRAM    (max 64 KBytes) - 8bit Bus width
    sram: RwAddr,
}

const BIOS________START: u32 = 0x00000000;
const BOARD_RAM___START: u32 = 0x02000000;
const CHIP_RAM____START: u32 = 0x03000000;
const IO_REG______START: u32 = 0x04000000;
const PALETTE_RAM_START: u32 = 0x05000000;
const VIDEO_RAM___START: u32 = 0x06000000;
const OBJ_ATTR____START: u32 = 0x07000000;
const ROM_0_______START: u32 = 0x08000000;
const ROM_1_______START: u32 = 0x0A000000;
const ROM_2_______START: u32 = 0x0C000000;
const SRAM________START: u32 = 0x0E000000;

const BIOS________END: u32 = 0x00003FFF;
const BOARD_RAM___END: u32 = 0x0203FFFF;
const CHIP_RAM____END: u32 = 0x03007FFF;
const IO_REG______END: u32 = 0x040003FE;
const PALETTE_RAM_END: u32 = 0x050003FF;
const VIDEO_RAM___END: u32 = 0x06017FFF;
const OBJ_ATTR____END: u32 = 0x070003FF;
const ROM_0_______END: u32 = 0x09FFFFFF;
const ROM_1_______END: u32 = 0x0BFFFFFF;
const ROM_2_______END: u32 = 0x0DFFFFFF;
const SRAM________END: u32 = 0x0E00FFFF;


impl GbaAddressBus {
    pub fn init(bios: Vec<u8>, rom_0: Vec<u8>, rom_1: Vec<u8>, rom_2: Vec<u8>) -> GbaAddressBus {
        GbaAddressBus {
            bios: RoAddr::init(bios, BIOS________START),
            board_ram: RwAddr::new(BIOS________START, BIOS________END),
            chip_ram: RwAddr::new(CHIP_RAM____START, CHIP_RAM____END),
            io_reg: RwAddr::new(IO_REG______START, IO_REG______END),
            palette_ram: RwAddr::new(PALETTE_RAM_START, PALETTE_RAM_END),
            video_ram: RwAddr::new(VIDEO_RAM___START, VIDEO_RAM___END),
            obj_attr: RwAddr::new(OBJ_ATTR____START, OBJ_ATTR____END),
            rom_0: RoAddr::init(rom_0, ROM_1_______START),
            rom_1: RoAddr::init(rom_1, ROM_1_______START),
            rom_2: RoAddr::init(rom_2, ROM_1_______START),
            sram: RwAddr::new(SRAM________START, SRAM________END),
        }
    }

    fn select_mem(&self, addr: u32) -> &dyn Memory {
        match addr {
            BIOS________START..=BIOS________END => &self.bios,
            BOARD_RAM___START..=BOARD_RAM___END => &self.board_ram,
            CHIP_RAM____START..=CHIP_RAM____END => &self.chip_ram,
            IO_REG______START..=IO_REG______END => &self.io_reg,
            PALETTE_RAM_START..=PALETTE_RAM_END => &self.palette_ram,
            VIDEO_RAM___START..=VIDEO_RAM___END => &self.video_ram,
            OBJ_ATTR____START..=OBJ_ATTR____END => &self.obj_attr,
            ROM_0_______START..=ROM_0_______END => &self.rom_0,
            ROM_1_______START..=ROM_1_______END => &self.rom_1,
            ROM_2_______START..=ROM_2_______END => &self.rom_2,
            SRAM________START..=SRAM________END => &self.sram,
            _ => unreachable!()
        }
    }

    fn select_mem_mut(&mut self, addr: u32) -> &mut dyn Memory {
        match addr {
            BIOS________START..=BIOS________END => &mut self.bios,
            BOARD_RAM___START..=BOARD_RAM___END => &mut self.board_ram,
            CHIP_RAM____START..=CHIP_RAM____END => &mut self.chip_ram,
            IO_REG______START..=IO_REG______END => &mut self.io_reg,
            PALETTE_RAM_START..=PALETTE_RAM_END => &mut self.palette_ram,
            VIDEO_RAM___START..=VIDEO_RAM___END => &mut self.video_ram,
            OBJ_ATTR____START..=OBJ_ATTR____END => &mut self.obj_attr,
            ROM_0_______START..=ROM_0_______END => &mut self.rom_0,
            ROM_1_______START..=ROM_1_______END => &mut self.rom_1,
            ROM_2_______START..=ROM_2_______END => &mut self.rom_2,
            SRAM________START..=SRAM________END => &mut self.sram,
            _ => unreachable!()
        }
    }
}

impl Memory for GbaAddressBus {
    fn get_byte(&self, addr: u32) -> u8 {
        self.select_mem(addr).get_byte(addr)
    }

    fn get_half_word(&self, addr: u32) -> u16 {
        self.select_mem(addr).get_half_word(addr)
    }

    fn get_word(&self, addr: u32) -> u32 {
        self.select_mem(addr).get_word(addr)
    }


    fn set_byte(&mut self, addr: u32, val: u8) {
        self.select_mem_mut(addr).set_byte(addr, val)
    }

    fn set_half_word(&mut self, addr: u32, val: u16) {
        self.select_mem_mut(addr).set_half_word(addr, val)
    }

    fn set_word(&mut self, addr: u32, val: u32) {
        self.select_mem_mut(addr).set_word(addr, val)
    }
}

/// 只读地址
struct RoAddr {
    data: Vec<u8>,
    offset: u32,
}

impl RoAddr {
    fn init(init: Vec<u8>, offset: u32) -> RoAddr {
        RoAddr {
            data: init,
            offset,
        }
    }
}

impl Memory for RoAddr {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[(addr - self.offset) as usize]
    }

    fn get_half_word(&self, addr: u32) -> u16 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 2] = self.data[start..(start + 2)].try_into().unwrap();
        u16::from_le_bytes(b)
    }

    fn get_word(&self, addr: u32) -> u32 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 4] = self.data[start..(start + 4)].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    fn set_byte(&mut self, _addr: u32, _val: u8) {}
    fn set_half_word(&mut self, _addr: u32, _val: u16) {}
    fn set_word(&mut self, _addr: u32, _val: u32) {}
}

/// 可读写地址
struct RwAddr {
    data: Vec<u8>,
    offset: u32,
}

impl RwAddr {
    fn new(addr_start: u32, addr_end: u32) -> RwAddr {
        RwAddr {
            data: vec![0u8; (addr_end - addr_start + 1) as usize],
            offset: addr_start,
        }
    }
}

impl Memory for RwAddr {
    fn get_byte(&self, addr: u32) -> u8 {
        self.data[(addr - self.offset) as usize]
    }

    fn get_half_word(&self, addr: u32) -> u16 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 2] = self.data[start..(start + 2)].try_into().unwrap();
        u16::from_le_bytes(b)
    }

    fn get_word(&self, addr: u32) -> u32 {
        let start = (addr - self.offset) as usize;
        let b: [u8; 4] = self.data[start..(start + 4)].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    fn set_byte(&mut self, addr: u32, val: u8) {
        self.data[(addr - self.offset) as usize] = val
    }
    fn set_half_word(&mut self, addr: u32, val: u16) {
        let u16_bytes: [u8; 2] = val.to_le_bytes();
        let addr = addr as usize;
        self.data[addr] = u16_bytes[0];
        self.data[addr + 1] = u16_bytes[1];
    }
    fn set_word(&mut self, addr: u32, val: u32) {
        let u32_bytes: [u8; 4] = val.to_le_bytes();
        let addr = addr as usize;
        self.data[addr..addr + 4].copy_from_slice(&u32_bytes);
    }
}