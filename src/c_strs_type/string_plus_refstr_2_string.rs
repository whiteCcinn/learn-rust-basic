// string = string + &str
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let info = String::from("hello");
    let new = info + "world";
    // println!("info = {:?}", info); //error: use of moved value: `info`
    println!("new = {:?}", new);
}