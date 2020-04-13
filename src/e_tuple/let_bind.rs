// 元祖内元素的可选绑定与不可选绑定
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 元祖
    // => a 不可变绑定
    // => b 可变绑定
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    // a 不可变绑定
    // a = false; // error: cannot assign twice to immutable variable `a`

    // b 可变绑定
    b = true;

    // 数据类型比较
    assert_eq!(a, b);
}
