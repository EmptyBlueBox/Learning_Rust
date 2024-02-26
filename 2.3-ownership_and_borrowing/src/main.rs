//rust 的所有权：malloc的指针只能存在一份，不能很多指针都能访问这块 malloc 的内存
// let s1 = String::from("hello"); 这里相当于 malloc
// let x: &str = "hello, world"; 这里相当于引用了一个全局字符串
// String类型默认“移动”就是浅复制，原子类型默认“复制”

// fn main() {
//     let _s1 = gives_ownership();         // gives_ownership 将返回值
//                                         // 移给 s1

//     let s2 = String::from("hello");     // s2 进入作用域

//     let _s3 = takes_and_gives_back(s2);  // s2 被移动到
//                                         // takes_and_gives_back 中,
//                                         // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
//                                              // 调用它的函数

//     let some_string = String::from("hello"); // some_string 进入作用域.

//     some_string                              // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

//     a_string  // 返回 a_string 并移出给调用的函数
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}",s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

//同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用 
//拥有：处于作用域
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束
    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束