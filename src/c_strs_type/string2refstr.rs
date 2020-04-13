// string -> &str
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // String
    let info = String::from("hello");

    // &str 1
    let info_str1 = info.as_str();
    println!("info_str1 = {:?}", info_str1);

    // &str 2
    let info_str2 = &info;
    println!("info_str2 = {:?}", info_str2);
}