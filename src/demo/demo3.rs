pub fn variable_fn () {
  let a: u64 = 123;
  let b = 345;
  let ac: i64 = -123;
  let mut acc_i32 = 111;
  
  
  acc_i32 = 321;

  // 这里会报warning  因为编译的时候检测到你创建了一个变量直接就改了 没有别的操作  所以他让你直接用后面的值去声明
  // let mut inferred_i32 = 12;
  // inferred_i32 = 88888888888i64;

  println!("let 声明的变量是不可变变量， 在语言层面尽量少的让变量的值可以改变");
  println!("{0} 和 {1} 还有 {2} 可变变量需要加mut {3}", a, b, ac, acc_i32);
}
