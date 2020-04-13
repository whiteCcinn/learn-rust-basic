// type 类型别名
#[allow(unused_variables, dead_code)]
pub fn handle() {
    // 常量
    {
        const N: i32 = 5;
    }

    // 静态全局变量
    {
        static N: i32 = 5; // 注意：必须标注一个静态的类型
    }

    // 静态变量可变性
    {
        static mut N: i32 = 5;
    }

    // 静态全局变量的多线程同步
    {
        static mut N: i32 = 5;

        // 读写包裹在unsafe{}块中执行
        unsafe {
            N += 1;
            println!("N: {}", N);
        }
    }
}