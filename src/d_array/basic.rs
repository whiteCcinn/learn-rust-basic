// 不可变数组 可变数组
#[allow(unused_variables, dead_code)]
pub fn handle() {

    // 不可变数组
    let arr1 = [0, 1, 2, 3, 4];
    println!("{:?}", arr1);

    let arr2 = &arr1[1..3];
    println!("{:?}", arr2);


    // 可变数组

    // [i32; 3] => [数组元素类型 : 数组长度]
    // [0; 3] => 使用0填充3个位置
    let mut array: [i32; 3] = [0; 3];

    for x in &array {
        print!("{} ", x);
    }
    print!("\n");

    // 修改数组[i]
    array[1] = 1;
    array[2] = 2;

    //
    for x in &array {
        print!("{} ", x);
    }
    print!("\n");
}
