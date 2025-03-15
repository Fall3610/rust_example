struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

pub fn show_rectangle() {
    let rect1 = Rectangle::new(10, 20);
    println!("rect area = {}", rect1.area());
    println!("rect width() = {}", rect1.width());
    println!("rect width = {}", rect1.width);
}
