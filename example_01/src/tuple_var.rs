// 元组 允许多个不同元素类型的数据放在一起

pub fn tuple_show() {
    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "这是一个元组");

    // 解构 tup 变量 将其中第二个元素绑定给变量x
    let (_, x, ..) = tup;
    // let (_, ..,x) = tup;
    println!("the value of x is:{}", x);

    // 使用 . 来访问指定的索引处的元素
    let first = tup.0;
    let second = tup.1;
    println!("the value of first is:{}", first);
    println!("the value of second is:{}", second);

    let s = String::from("hello, hackquest!");
    // 函数将返回值为元组类型
    let (s1, len) = calculate_length(s);
    println!("The length of '{}' is {}.", s1, len)
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
