use std::{fs::File, io::ErrorKind};

fn main() {
    // 1. panic! 处理不可以恢复的错误
    // 比较类似C/C++的assert语法
    // panic!("Hello, world!");

    // 2. Result!可以恢复的错误
    // let f = File::open("Hello.txt");

    // 3. 处理Result的一种方法match表达式
    // match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error);
    //     }
    // };

    // 4. 针对不同的错误执行不同的处理
    // match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind()  {
    //         ErrorKind::NotFound => match File::create("Hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file {:?}", e),
    //         },
    //         oe => panic!("Error opening file {:?}", oe),
    //     },
    // };

    // let _f = File::open("Hello.txt").unwrap();
    // let _f = File::open("Hello.txt").expect("无法打开文件");
}
