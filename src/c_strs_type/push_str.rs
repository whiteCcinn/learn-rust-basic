// string push_str
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let mut info = String::from("hello");
    println!("info = {:?}", info);

    // 压入字符
    info.push('-');

    // 压入字符串
    info.push_str("world");
    println!("info = {:?}", info);
}