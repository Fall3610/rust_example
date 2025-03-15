struct Point {
    x: i32,
    y: i32,
}


fn process_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("坐标在原点"),
        Point { x, y } => println!("坐标在 ({}, {})", x, y),
    }
}

pub fn show_point(){
    let p = Point { x: 0, y: 0 };
    let p2 = Point { x: 1, y: 111 };
    process_point(p);
    process_point(p2);
}