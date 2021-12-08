use ferris_says::say;
use std::io::{stdout, BufWriter};
mod demo;

fn main() {
    
    // demo 1 print and pr
    demo::print_function();

    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
