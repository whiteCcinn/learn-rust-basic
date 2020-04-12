// 类型转换
#[allow(dead_code, unused_variables)]
pub fn handle() {
    // float
    let a = 65.4321_f32;
    println!("{:?}", a);

    // float => uint8
    let b = a as u8;
    println!("{:?}", b);

    // uint8 => char
    let c = b as char;
    println!("{:?}", c);
}
