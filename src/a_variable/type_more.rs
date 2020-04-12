// 类型比较
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let a1 = 5; // 自动推导数据类型i32
    let a2: i32 = 5; // 指定数据类型i32
    assert_eq!(a1, a2); // 比较a1与a2的【数据类型】是否一致

    let a1: u32 = 5;
    let a2: i32 = 5;
    // assert_eq!(a1, a2);  expected u32, found i32
}
