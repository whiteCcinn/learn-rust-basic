// type 类型别名
#[allow(unused_variables, dead_code)]
pub fn handle() {
    type NanoSecond = u64;
    type Point = (u8, u8);

    let x: NanoSecond = 19999;
    let y: Point = (1, 2);

    println!("x = {:?}", x);
    println!("y = {:?}", y);
}