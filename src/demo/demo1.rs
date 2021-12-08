
pub fn print_function() {
  println!("Hello, world!");
  print!("print 不带换行的控制台输出");
  print!("print 不带换行的控制台输出 但是可以使用换行符换行\\n \n");
  println!("换行符 \\n 转义");
  println!("println 带换行的控制台输出");

   // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    // 打印
    let test_string = String::from("第一次打印的东西");
    let test_b = String::from("第二次打印的东西");
    println!("{} 你看这是啥", test_string);
    println!("{0} 你看这是啥 {1}", test_string, test_b);
    println!("{test}, {test_a}, {test_string}",
        test="还可以这样",
        test_a=test_b,
        test_string=test_string
    );

    // Special formatting can be specified after a `:`. 这里的:b 是二进制
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));

}
