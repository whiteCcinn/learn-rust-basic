// 元祖类型的struct
struct Pair(i32, f64);

#[allow(unused_variables, dead_code)]
pub fn handle() {
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructure a tuple struct
    let Pair(a, b) = pair;
    println!("a = {:?}, b = {:?}", a, b);
}
