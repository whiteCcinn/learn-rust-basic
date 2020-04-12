/*
i8、i32 => int8、int32
u8、u32 => uint8、uint32
f32、f64 => float32、float64
数值如果不指定iN、uN、fN，则编译器自动使用 i32 为整数, f64 作为浮点数。
*/
#[allow(dead_code, unused_variables)]
pub fn handle() {
    let x1 = 1u8; // 告诉编译器使用 uint8 分配内存来存储 1
    let x2: u8 = 1; // 同上
    let x3 = 1i8; // 告诉编译器使用 int8 分配内存来存储 1
    let x4: i8 = 1; // 同上

    let y1 = 2u32; // 告诉编译器使用 uint32 分配内存来存储 2
    let y2: u32 = 2;
    let y3 = 2i32; // 告诉编译器使用 uint32 分配内存来存储 1
    let y4: i32 = 2;

    let z1 = 3f32; // 告诉编译器使用 float32 分配内存来存储 3
    let z2: f32 = 3f32;
    let z3: f64 = 3f64; // 告诉编译器使用 float64 分配内存来存储 3
    let z4 = 9.9;
}
