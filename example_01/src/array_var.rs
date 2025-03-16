// 静态数组 长度固定

pub fn array_show() {
    let arr = [3, 5, 6];
    let first = arr[0];
    println!("{}", first);
    let last = arr[arr.len() - 1];
    println!("{}", last);

    // 二维数组
    let arrays = [[1, 2], [3, 4], [5, 6]];
    for x in arrays {
        println!("第一层:{:?}", x);
        for y in x {
            println!("第一层 元素:{}", y);
        }
    }
}
