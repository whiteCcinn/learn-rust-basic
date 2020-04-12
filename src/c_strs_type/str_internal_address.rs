// &str内部的地址
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let mystr: &str = "abcd";
    println!("mystr  address:  {:p}", &mystr);
    println!("mystr  address:  {:p}", &mystr[0..]);
    println!("mystr  address:  {:p}", &mystr[1..]);
    println!("mystr  address:  {:p}", &mystr[2..]);
    println!("mystr  address:  {:p}", &mystr[3..]);
}

/*
mystr 与 mystr[i] 内存地址段明显不一样
mystr => 存储在局部栈帧
mystr[i] => 常量字符存储在符号表中
*/