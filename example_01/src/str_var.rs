// 字符串字面量
pub fn show_str() {
    let a: &str = "hello world";
    println!("{} go go go", a);

    let hello = &a[..5];
    println!("{} go go", hello);

    let s1: String = String::from("hello world");
    let s2: &str = &s1[0..5];
    println!("{} 动态字符串 go go", s2);

    // 动态字符串转成字符串字面量
    let s3: &str = s1.as_str();
    println!("{} 动态字符串转成字符串字面量", s3);

    let mut dynamic_var = String::from("hello ");
    // 追加字符串
    dynamic_var.push_str("world");
    println!("追加字符串 :{}", dynamic_var);
    // 追加字符
    dynamic_var.push('!');
    println!("追加字符 char :{}", dynamic_var);

    // 插入字符 修改原来的字符 需要指定索引的位置 索引从 0 开始
    dynamic_var.insert(5, ',');
    println!("插入字符 char :{}", dynamic_var);
    // 插入字符串
    dynamic_var.insert_str(6, "I like");
    println!("插入字符串  :{}", dynamic_var);

    // replace 替换字符串
    let str_old = String::from(" I like rust,rust,rust!");
    let str_new = str_old.replace("rust", "RUST");
    println!("原字符串长度为:{},内存地址:{:p}", str_old, &str_old);
    println!("新字符串长度为:{},内存地址:{:p}", str_new, &str_new);

    // pop删除操作 修改原来的字符串 相当于单出字符串数组的最后一个字符
    // 返回值是删除的字符 Option 类型 如果字符串为空 则返回 None
    // 注意: pop 是按照字符的维度进行的 而不是字节
    let mut string_pop = String::from("删除操作,rust,中文!");
    let p1 = string_pop.pop();
    println!("p1:{:?}", p1);
    let p2 = string_pop.pop();
    println!("p2:{:?}", p2);
    println!("string_pop:{}", string_pop);
}
