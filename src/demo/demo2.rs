#[derive(Debug)]
struct Matrix(i32, i32, i32, i32);

pub fn fn_debug () {
  // with `fmt::Debug`.
  // struct UnPrintable(i32);

  // The `derive` attribute automatically creates the implementation
  // required to make this `struct` printable with `fmt::Debug`.
  // #[derive(Debug)]
  // struct DebugPrintable(i32);
  // 不知道在干嘛 

  // 元组
  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // 类似js解构
  let tuple = (1, true);
  let (a, b) = tuple;
  println!("元组{}, {}", a, b);

  let matrix = Matrix(1, 2, 3, 4);
  println!("{:?}", matrix);
}

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // `let` can be used to bind the members of a tuple to variables
  let (integer, boolean) = pair;

  (boolean, integer)
}
