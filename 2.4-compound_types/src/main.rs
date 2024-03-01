
/*
insert改变原始字符串
*/

// fn main() {
//     let mut s = String::from("Hello rust!");
//     s.insert(5, ',');
//     println!("插入字符 insert() -> {}", s);
//     s.insert_str(6, " I like");
//     println!("插入字符串 insert_str() -> {}", s);
// }

/*
只有他俩和concatenate生成新的字符串！！！！！
replace、replacen产生一个新的字符串，所以可以适用于 &str 类型
*/
// fn main() {
//     let string_replace = String::from("I like rust. Learning rust is my favorite!");
//     let new_string_replace = string_replace.replace("rust", "RUST");
//     println!("{}",new_string_replace);
//     let new=new_string_replace.clone()+&new_string_replace;
//     println!("{}",new);
// }


/*
replace_range改变原始字符串
*/
// fn main() {
//     let mut string_replace_range = String::from("I like rust!");
//     string_replace_range.replace_range(7..8, "R");
//     dbg!(string_replace_range);
// }

/*
所有删除操作都是改变原始字符串，包括remove()、remove_range()、truncate()、clear()、pop()
而且都是按照字节来处理的
*/

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}