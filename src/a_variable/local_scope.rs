// 局部作用域
#[allow(dead_code)]
pub fn handle() {
    let long_lived_binding = 1;

    // 局部作用域
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding); //error：short_lived_binding局部作用域内的变量
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}
