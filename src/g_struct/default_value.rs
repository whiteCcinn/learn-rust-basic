#[derive(Default)] // 成员初始化默认值
#[derive(Debug)]   // println!("{:?}", struct实例);
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 创建struct实例，并给成员默认初始化
    let origin = Point3d::default();
    println!("{:?}", origin);

    // 赋值实例的成员y
    let point = Point3d { y: 1, ..origin };
    println!("{:?}", point);

    // 获取struct实例的成员x、成员y的值
    let Point3d { x: x0, y: y0, .. } = point;
    println!("{} - {}", x0, y0);
}
