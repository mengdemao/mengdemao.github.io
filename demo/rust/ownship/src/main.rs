fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("Test");  // s 进入作用域
    takes_ownership(s.clone());
    println!("{}", s);

    let x = 5;
    makes_copy(x);

    let s3 = String::from("hello");

    let len = calculate_length(&s3);

    println!("The length of '{}' is {}.", s3, len);

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn calculate_length(s: &String) -> usize {
    s.len()
}
