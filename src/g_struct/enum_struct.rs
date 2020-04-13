// 枚举类型的struct
#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    // 元祖类型结构体
    Move { x: i32, y: i32 },
    // 字段类型结构体
    Write(String), // 元祖类型结构体
}

#[allow(unused_variables, dead_code)]
pub fn handle() {
    let a: Message = Message::Quit;
    let b: Message = Message::ChangeColor(1, 2, 3);
    let c: Message = Message::Move { x: 3, y: 4 };
    let d: Message = Message::Write(String::from("hello"));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}
