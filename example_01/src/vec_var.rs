// 动态数组

pub fn vec_show() {
    // 显式声明动态数组类型
    let v1: Vec<i32> = Vec::new();

    // 编译器根据元素自动推断类型  注意: 需要将 v2 声明为可变 mut 后 才能进行修改
    let mut v2 = Vec::new();
    v2.push(1);

    // 使用宏 vec! 来创建数组 支持在创建的时候给予初始化值
    let v3 = vec![1, 2, 3];

    // 使用 [初始值;长度] 来创建数组 默认值为 0 初始长度为 3
    let v4 = vec![0; 3];

    // 使用 from 语法来创建数组
    let v5 = Vec::from([0, 0, 0]);
    assert_eq!(v4, v5);

    let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // 通过索引直接访问指定位置的元素
    let third: &i32 = &v1[2];
    println!("第三个元素是:{}", third);

    // 通过 .get() 方法访问 防止下标越界
    match v1.get(2) {
        Some(third) => println!("match 第三个元素是:{}", third),
        None => println!("match 指定下标的元素不存在!"),
    }

    // 通过迭代访问并修改元素
    for i in &mut v1 {
        // 这里的 i 是数组 v1 中元素的可变引用 通过 *i 解引用获取到值 并 +10
        *i += 10;
    }
    println!("v1 = {:?}", v1);

    let mut v2 = vec![1, 2];
    // 检查 v2 是否为空
    assert!(!v2.is_empty());
    // 在指定的索引插入数据 索引值不能大于 v2 的长度
    v2.insert(2, 3);
    println!("v2 = {:?}", v2);

    // 移除指定位置的元素并放回
    assert_eq!(v2.remove(2), 3);
    println!("v2 = {:?}", v2);

    // 移除 v2的尾部元素
    assert_eq!(v2.pop(), Some(2));
    println!("v2 = {:?}", v2);
    // 清空 v2
    v2.clear();
    println!("v2 = {:?}", v2);
}
