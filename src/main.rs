use ferris_says::say;
use std::io::{stdout, BufWriter};
mod demo;
use crate::demo::City;
use crate::demo::Color;

fn main() {
    
    // demo 1 print and pr
    // demo::print_function();

    // demo 2 debug 什么。。。看不懂
    // demo::fn_debug();

    // demo 3 基础语法
    // demo::variable_fn();

    // demo4 formatting
    // demo::formatting_fn();

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }



    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
