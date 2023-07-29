fn main() {
    // 1.创建字符串
    let mut _s = String::new();

    // 2.创建空字符串
    let data = "initial contents";

    let _s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let _s = "initial contents".to_string();

    // 直接对字符串进行转化
    let _s = String::from("initial contents");

    // 3. 更新字符串
    // 3. 1 使用 push_str 和 push 附加字符串
    let mut s = String::from("Hello");
    s.push_str("World");
    println!("{}", s);

    s.push('!');
    println!("{}", s);

    // 3.2 使用 + 运算符或 format! 宏拼接字符串

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    println!("{} {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);

    // 4. 索引字符串

    let _s1 = String::from("hello");
    // let h = s1[0]; // error[E0277]: the type `String` cannot be indexed by `{integer}

    // 5. 字符串 slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s);

    // 6. 遍历字符串
    for c in "Зд".chars() {
        println!("{c}");
    }

}
