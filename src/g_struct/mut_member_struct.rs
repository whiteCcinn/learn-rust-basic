// 结构体成员可变
// 主要利用了Cell, Cell只能用于实现了Copy trait的类型
use std::cell::Cell;

#[derive(Debug)]
struct Point {
    x: i32,
    y: Cell<i32>,
}

#[allow(unused_variables, dead_code)]
pub fn handle() {
    let point = Point { x: 5, y: Cell::new(6) };
    println!("{:?}", point);

    point.y.set(7);
    println!("{:?}", point);
}
