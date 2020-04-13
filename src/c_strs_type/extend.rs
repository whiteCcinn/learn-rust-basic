// extend
#[allow(unused_variables, dead_code)]
pub fn handle() {
    let a = &['a', 'b', 'c'];
    let mut message = String::from("==> ");
    message.extend(a);
    println!("message = {:?}", message);
}