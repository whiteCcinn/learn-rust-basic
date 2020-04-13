// struct之间的包含
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 创建Point实例
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 创建Rectangle实例
    let rectangle = Rectangle {
        p1: Point { x: point.x + 0.5, y: point.y + 0.5},
        p2: point,
    };
    println!("{:?}", rectangle);
}
