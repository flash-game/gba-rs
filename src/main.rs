use crate::util::BitUtilExt;
use std::sync::{Arc, Mutex};

mod cpu;
mod util;

fn main() {
    let i = !((1 - 0) * 3) as u32;
    println!("{:b}", i);
    let a: u32 = 0b_0000_0000_1111_1111_1111_1111_0000_1011;
    println!("{:b}", a.extract(0, 4));
    let arc = Arc::new(Mutex::new(String::new()));
    let arc1 = arc.lock().unwrap().clone();
}
