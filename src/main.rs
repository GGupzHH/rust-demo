use ferris_says::say;
use std::io::{stdout, BufWriter};

mod demo;
// use crate::demo::City;
// use crate::demo::Color;
use crate::demo::List;
// use crate::demo::Point2D;

fn main() {
    
    // demo 1 print and pr
    // demo::print_function();

    // demo 2 debug 什么。。。看不懂
		let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", demo::reverse(pair));
    demo::fn_debug();

    // demo 3 基础语法
    // demo::variable_fn();

    // demo4 formatting
    // demo::formatting_fn();

    // 遍历demo 4中的数据结构
    // 每一个数据结构的属性都可以规定私有或者公共
    // 数据结构是直接存储到内存的  所以想看的话就需要实现数据结构的  impl fmt::Display 方法
    // fmt::Display 方法内部可以使用 fn fmt去将数据拼接成自己想要的格式输出

    // for city in [
    //     City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    //     City { name: "Oslo", lat: 59.95, lon: 10.75 },
    //     City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    // ].iter() {
    //     println!("{}", *city);
    // }
    // for color in [
    //     Color { red: 128, green: 255, blue: 90 },
    //     Color { red: 0, green: 3, blue: 254 },
    //     Color { red: 0, green: 0, blue: 0 },
    // ].iter() {
    //     // Switch this to use {} once you've added an implementation
    //     // for fmt::Display.
    //     // println!("{:?}", *color);
    //     println!("{}", *color);
    // }

    // demo::point2_d_fn();

    // demo5 Vec
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // demo 6 type
		// demo::print_type();

    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
