// 指针变量的值不能改变
// [内存地址] as *const T
#[allow(unused_variables, dead_code)]
pub fn handle() {
    {
        let x = 5;

        //【不可变绑定】对变量取地址
        let ptr = &x as *const i32;
        println!("ptr = {:p}", ptr);

        // unsafe{...} 对指针进行【解地址】读取内存空间的数据
        let val = unsafe { *ptr };
        println!("val = {:?}", val);
    }

    {
        let x = 5;
        let y = 6;

        //【可变绑定】对变量取地址
        let mut ptr = &x as *const i32;
        println!("ptr = {:p}", ptr);

        // 解地址读取内存空间的数据
        println!("*ptr = {:?}", unsafe { *ptr });

        // 重新指向其他变量标记的内存地址
        ptr = &y;
        println!("*ptr = {:?}", unsafe { *ptr });
    }

    // const指针转换为mut指针
    {
        // 栈帧上局部变量
        let x = 5;

        // *const T 类型指针
        let ptr1 = &x as *const i32;
        // let ptr1 : *const i32 = &x as *const i32; // 同上
        println!("*ptr1 = {:?}", unsafe { *ptr1 });

        // *const T => *mut T
        let ptr2 = ptr1 as *mut i32;

        // 修改 *mut T 类型指针变量指向的内存空间中的数据
        unsafe { *ptr2 = 6 };
        println!("*ptr2 = {:?}", unsafe { *ptr2 });
    }

    //  直接定义 mut 类型变量与指针变量
    {
        let mut x = 2;

        // 获取可变绑定类型变量的内存地址
        let ptr = &mut x as *mut i32;

        // 读取地址空间中的数据
        println!("*ptr = {:?}", unsafe {*ptr});

        // 修改地址空间中的数据
        unsafe {*ptr = 3};
        println!("*ptr = {:?}", unsafe {*ptr});
    }
}
