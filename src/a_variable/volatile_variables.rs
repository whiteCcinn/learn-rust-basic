// 可变绑定
// 可以对变量重新赋值
#[allow(dead_code)]
pub fn handle() {
    // 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量
    let mut a: f64 = 1.0;
    println!("{:?}", a);

    //改变变量a的值
    a = 2.0;
    println!("{:?}", a);
}
