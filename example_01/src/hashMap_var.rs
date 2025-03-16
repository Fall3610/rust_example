use std::collections::HashMap;

pub fn hashmap_show() {
    let mut strudent = HashMap::new();
    strudent.insert("Alice", 12);

    // 创建指定大小的 HashMap
    let mut stu_score = HashMap::with_capacity(3);
    stu_score.insert("Bob", 12);
    stu_score.insert("tt", 11);
    stu_score.insert("kk", 13);

    let mut user_list = Vec::new();
    user_list.push(("Alice", 12));
    user_list.push(("Alice2", 13));
    user_list.push(("Alice3", 14));
    // 使用迭代器 和 collect 方法将数组转换为 HashMap
    let mut user_map: HashMap<&str, i32> = user_list.into_iter().collect();
    println!("user_map = {:?}", user_map);

    // 通过 key 获取对应的值
    let alice_score = user_map["Alice"];
    println!("alice_score = {}", alice_score);

    // 通过 hashmap.get(key) 获取对应的值 返回值是 Option 枚举类型
    let alice_score: Option<&i32> = user_map.get("Alice");
    println!("alice_score = {:?}", alice_score);
    // 不存在的 key 返回值是 None 不会报错
    let ttt_score: Option<&i32> = user_map.get("TTT");
    println!("ttt_score = {:?}", ttt_score);

    // 覆盖已有的值 insert 操作 会返回旧的值
    let old = user_map.insert("Alice", 1000);
    assert_eq!(old, Some(12));
    println!("user_map = {:?}", user_map);

    // or_insert 如果存在则返回旧的值 如果不存在 则返回其当前值
    let v = user_map.entry("Alice5").or_insert(2000);
    println!("v = {:?}", v);
    assert_eq!(v, &2000);
    println!("user_map = {:?}", user_map);
}
