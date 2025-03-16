// 符合数据类型 动态字符串切片

pub fn show_slice() {
    let s = String::from("hello world");
    // 通过索引和范围来指定字符串的一部分
    let hello: &str = &s[0..5];
    println!("{}", hello);

    let word: &str = &s[6..];
    println!("{}", word);

    let s: String = String::from("hello, hackquest!");
    // 起始从 0 开始 ..代表一个或者多个索引
    let slice1: &str = &s[0..2];
    println!("{}", slice1);
    // 默认也是从 0 开始
    let slice2: &str = &s[..2];
    println!("{}", slice2);

    let len = s.len();
    println!("{}", len);
}
