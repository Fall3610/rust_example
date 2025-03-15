// 流程控制
pub fn process_control() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("process_control The value of number is: {}", number);
}
// 循环控制
pub fn show_item() {
    for item in 1..5 {
        if item == 2 {
            continue;
        }
        if item == 4 {
            break;
        }
        println!("循环控制 this item is: {}", item);
    }
}

// for 循环
pub fn show_for() {
    let vec1 = vec![1, 2, 3];
    // into_iter() 转移所有权
    for x in vec1.into_iter() {
        println!("转移所有权 into_iter() Item:{}", x)
    }
    // vec1 所有权已经转移 无法再次使用

    // 借用情况
    let vec2 = vec![1, 2, 3];
    for x in &vec2 {
        println!("借用引用 &vec2 Item:{}", x)
    }
    println!("借用引用 &vec2 Item:{:?}", vec2);

    // 借用引用
    let vec3 = vec![1, 2, 3];
    for item in vec3.iter() {
        println!("借用引用 iter() Item:{}", item)
    }
    println!("借用引用 iter() Item:{:?}", vec3);

    let mut vec4 = vec![1, 2, 3];
    for item in vec4.iter_mut() {
        println!("借用引用 iter_mut() Item:{}", item)
    }
    println!("借用引用 iter_mut() Item:{:?}", vec4)
}

// for 和 while 比较
pub fn for_and_while_compare() {
    let array = [1, 2, 3, 4, 5];

    // while 循环使用索引
    let mut index = 0;
    while index < array.len() {
        println!("value:{}", array[index]);
        index += 1;
    }

    for item in array.iter() {
        println!("for 和 while 比较 Item:{}", item)
    }
}

// loop 无限循环
pub fn loop_cycle() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result is {}", result);
}
