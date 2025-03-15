// 枚举

// 简单枚举
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub enum TrafficLightWithTime {
    Red(u8),
    Yellow(char),
    Green(String),
}

pub fn enum_show() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;

    let red_with_time = TrafficLightWithTime::Red(10);
    let yellow_with_time = TrafficLightWithTime::Yellow('3');
    let green_with_time = TrafficLightWithTime::Green(String::from("路灯持续一分钟"));
}
