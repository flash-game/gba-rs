mod cpu;

fn main() {
    println!("Hello, world!");
    let a: u32 = (u32::max_value() & 0b_0000_0000_1111_1111_1111_1111_1111_1111) - 100;
    println!("{:32b}", a);

    println!("{}", a as i32);

    println!("{:32b}", a << 2);

    println!("{:32b}", (a << 2) as i32);

    println!("{:32b}", ((a << 8) as i32 >> 6) as u32);
}
