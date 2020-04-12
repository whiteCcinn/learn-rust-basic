// 基本使用
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let str1 = "Hello, world!"; // 自动推导类型为 &str
    let str2: &str = "Hello, world!";
    println!("str1 = {:?}", str1);
    println!("&str1 = {:?}", &str1);
    println!("str2 = {:?}", str2);
    println!("&str2 = {:?}", &str2);
}
