// 基本使用
#[allow(dead_code)]
pub fn handle() {
    // boolean type
    let f1 = true;
    let f2: bool = false;
    println!("{:?}", f1);
    println!("{:?}", f2);

    // char type
    let c = 'c';
    println!("{:?}", c);

    // numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = (z - 123.4).abs();
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex: u32 = 0xf23a_b049;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
    println!("{:?}", zero);
    println!("{:?}", bin);
    println!("{:?}", oct);
    println!("{:?}", hex);
}
