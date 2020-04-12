// 也可以对变量重新绑定
#[allow(unused_mut,dead_code)]
pub fn handle() {
    // 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量
    let mut a: f64 = 1.0;
    println!("{:?}", a);

    // 重新绑定为不可变
    let a1 = a;
    println!("{:?}", a1);

    // 重新绑定为可变
    let mut a2 = a;
    println!("{:?}", a2);
}
