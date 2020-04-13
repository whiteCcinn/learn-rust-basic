// ^str -> string
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let str1 = "hello".to_string();
    println!("str1 = {:?}", str1);

    let str2: &str = "hello";
    let str3 = str2.to_string();
    println!("str3 = {:?}", str3);
}