// 绑定其他元祖内的元素
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 不可变绑定形式的元祖1
    // arg1 => int 32 值
    // arg2 => &string 内存地址
    let tuple: (i32, &str) = (50, "hello");
    println!("tuple = {:?}", tuple);

    // 绑定上面定义的元祖的arg1
    let (arg1_1, _) = tuple;
    let arg1_2 = tuple.0;
    println!("arg1_1 = {:?}", arg1_1);
    println!("arg1_2 = {:?}", arg1_2);

    // 绑定上面定义的元祖的arg2
    let arg2 = tuple.1;
    println!("arg2 = {:?}", arg2);
}
