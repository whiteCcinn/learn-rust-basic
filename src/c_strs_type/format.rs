// format
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let str1: &str = "hello";
    let str2: String = String::from("world");
    let str3 = format!("{0} - {1}", str1, str2); // 这里是引用的形式传递，并没有发生所有权转移
    println!("str3 = {:?}", str3);
}