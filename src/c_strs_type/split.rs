// 字符串切割
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // &str
    let mystr: &str = "abcd";
    println!("mystr :{:?}", &mystr[1..2]);  // "bc"
    println!("mystr :{:?}", &mystr[1..]);   // "bcd"
    println!("mystr :{:?}", &mystr[..]);    // "abcd"

    // &String
    let my_str: String = "ABCD".to_string();
    println!("my_str :{:?}", &my_str[1..2]); // "BC"
    println!("my_str :{:?}", &my_str[1..]);  // "BCD"
    println!("my_str :{:?}", &my_str[..]);   // "ABCD"

    let a = &my_str[1..2];

    // &str
    let mystr: &str = "abcd";
//    println!("mystr :{:?}", mystr[1..]); //  =>error

    // &String
    let my_str: String = "ABCD".to_string();
//    println!("my_str :{:?}", my_str[1..]); //  =>error
}