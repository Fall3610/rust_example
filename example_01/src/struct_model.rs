// 结构体

#[derive(Debug)] // Debug trait 允许我们以调试格式打印数据结构
struct Car {
    brand: String,
    color: String,
    year: String,
    is_new_energy: bool,
    price: f64,
}
fn build_car(brand: String, color: String, year: String, is_new_energy: bool, price: f64) -> Car {
    Car {
        brand,
        color,
        year,
        is_new_energy,
        price,
    }
}

pub fn create_car() {
    let mut car = build_car(
        String::from("BYD"),
        String::from("Red"),
        String::from("2025-03-16"),
        true,
        19.98,
    );
    println!("the car:{:?}", car);

    // 访问并修改结构体
    car.color = String::from("blue");
    println!("the car new color:{:?}", car);

    // 根据已有的机构体 创建新的结构体实例
    let new_car = Car {
        color: String::from("white"),
        // 其他字段从 car 对象中取 必须在结构体的尾部进行使用
        ..car
    };
    println!("the new_car:{:?}", new_car);
}
