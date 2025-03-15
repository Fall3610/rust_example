extern crate core;

mod Option_var;
mod enum_var;
mod mut_var;
mod process_control;
mod enum_match;
mod struct_match;
mod func_var;

use crate::enum_match::show_shape_area;
use crate::Option_var::divide;
use crate::enum_var::{TrafficLight, enum_show};
use crate::mut_var::show_var;
use crate::process_control::{
    for_and_while_compare, loop_cycle, process_control, show_for, show_item,
};
use crate::struct_match::show_point;

fn main() {
    println!("Hello, world!");
    // 不可变变量
    show_var();
    // 流程控制
    process_control();
    // 循环控制
    show_item();
    // for 循环与所有权
    show_for();
    // for 和 while 比较
    for_and_while_compare();
    // loop 无限循环
    loop_cycle();
    // 枚举
    enum_show();
    // Option
    let result = divide(1.0, 1.0);
    // let mut div_result =0.0;
    let div_result = match result {
        Some(res) => {
            // div_result = res;
            println!("Result: {}", res);
            res
        }
        None => {
            println!("Divide by zero");
            0.0
        }
    };
    println!("div_result is {}", div_result);
    // match 匹配模式扩展
    show_shape_area();
    // 结构体匹配
    show_point();
}
