// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let _x = 5;
//     let y = 10;
// }

// fn main() {
//     let (a, mut b): (bool,bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {}, b = {}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }

// struct Struct {
//     e: i32
// }
// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };
//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

// fn main() {
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;
//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x is: {}", x);
// }

// //let x; let x 和 let mut x; x 的区别
// //第一个重复分配内存所以类型可以不同，第二个内存一样所以类型必须相同
// // 字符串类型
// let spaces = "   ";
// // usize数值类型
// let spaces = spaces.len();
