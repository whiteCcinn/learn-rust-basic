// 定义结构体与使用

struct Point {
    x: f64,
    y: f64,
}

#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 创建struct实例
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 绑定获取struct实例成员的值
    let Point { x: my_x, y: my_y } = point;
    println!("my_x = {}, my_y = {}", my_x, my_y);
}
