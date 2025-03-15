mod mut_var;
mod process_control;

use crate::mut_var::show_var;
use crate::process_control::{
    for_and_while_compare, loop_cycle, process_control, show_for, show_item,
};

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
}
