// fn main() {
//     let guess:i32= "42".parse().expect("Not a number!");
// }

// fn main() {
//     let a : u8 = 255;
//     let b = a.wrapping_add(20);
//     println!("{}", b);  // 19
// }

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     println!("abc (f32)");
//     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("         0.3: {:x}", (abc.2).to_bits());
//     println!();

//     println!("xyz (f64)");
//     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();

//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

// fn main() {
//   // 编译器会进行自动推导，给予twenty i32的类型
//   let twenty = 20;
//   // 类型标注
//   let twenty_one: i32 = 21;
//   // 通过类型后缀的方式进行类型标注：22是i32类型
//   let twenty_two = 22i32;

//   // 只有同样类型，才能运算
//   let addition = twenty + twenty_one + twenty_two;
//   println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

//   // 对于较长的数字，可以用_进行分割，提升可读性
//   let one_million: i64 = 1_000_000;
//   println!("{}", one_million.pow(2));

//   // 定义一个f32数组，其中42.0会自动被推导为f32类型
//   let forty_twos = [
//     42.0f32,
//     42f32,
//     42.0_f32,
//   ];

//   // 打印数组中第一个值，并控制小数位为2位
//   println!("{:.2}", forty_twos[0]);
// }

// fn main()
// {
// for i in 'a'..='z' {
//     println!("{}",i);
// }
// }

use num::complex::Complex;
fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a / b;

   println!("{} + {}i", result.re, result.im)
 }

