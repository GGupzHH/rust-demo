

pub fn print_type () {
  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Integer subtraction
  println!("1 - 2 = {}", 1 - 2);
  // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations 位运算
  // &	  与	  两个位都为1时，结果才为1
  // |	  或	  两个位都为0时，结果才为0
  // ^	  异或 	两个位相同为0，相异为1
  // ~	  取反	0变1，1变0
  // <<	  左移	各二进位全部左移若干位，高位丢弃，低位补0
  // >>	  右移	各二进位全部右移若干位，对无符号数，高位补0，有符号数，各编译器处理方法不一样，有的补符号位（算术右移），有的补0（逻辑右移）
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

  // Use underscores to improve readability! 整型分割
  println!("One million is written as {}", 1_000_000u32);
}