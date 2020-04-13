// 元祖
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 不可变绑定形式的元祖1
    // arg1 => int 32 值
    // arg2 => &string 内存地址
    let tuple1: (i32, &str) = (50, "hello");
    println!("tuple1 = {:?}", tuple1);

    // 不可变绑定形式的元祖2
    let age = 99;
    let name: &str = "hello";
    let tuple2: (i32, &str) = (age, name);
    println!("tuple2 = {:?}", tuple2);
}
