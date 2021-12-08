use ferris_says::say;
use std::io::{stdout, BufWriter};
mod demo;

fn main() {
    
    // demo 1 print and pr
    demo::print_function();

    // demo 2 debug 什么。。。看不懂
    demo::fn_debug();

    // demo 3 基础语法
    demo::variable_fn();

    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
