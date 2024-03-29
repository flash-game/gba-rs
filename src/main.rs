use std::sync::{Arc, Mutex};

use fantasy_util::bit::usize::BitUtil;
use crate::gba::mem::mem::GbaAddressBus;

mod cpu;
mod gba;
mod util;

fn main() {
    let gba_bus = GbaAddressBus::init(vec![0u8; 1024], vec![0u8; 1024], vec![0u8; 1024], vec![0u8; 1024]);

    let i = !((1 - 0) * 3) as u32;
    println!("{:b}", i);
    let a: u32 = 0b_0000_0000_1111_1111_1111_1111_0000_1011;
    println!("{:b}", a.extract(0, 4));
    let arc = Arc::new(Mutex::new(String::new()));
    let _arc1 = arc.lock().unwrap().clone();
    println!("hello")
}
