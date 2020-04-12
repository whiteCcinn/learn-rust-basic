// 不可对变量重新[赋值]
pub fn handle() {
    let a1 = 5; // 自动推导数据类型i32
    a1 = 10; // error: cannot assign twice to immutable variable `a1`
}
