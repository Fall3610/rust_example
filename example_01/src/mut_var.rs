pub fn show_var() {
    // 可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // y 为不可变变量 没有 mut 修饰 Rust 默认任务这是不可变的变量
    let y = 3;
    // Rust 编译器会提示 Cannot assign a new value to an immutable variable more than once [E0384]
    // y = 1;
    println!("The value of y is: {}", y);
}
