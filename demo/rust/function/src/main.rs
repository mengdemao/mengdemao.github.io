fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_args(12);
    print_labeled_measurement(13, 'a');

    another_function_with_args(function_with_return_value1());
    another_function_with_args(function_with_return_value2());
    another_function_with_args(function_with_return_value3());
}

/// rust函数可以随便进行定义
/// 不需要进行声明
fn another_function() {
    println!("Another function.");
}

/// rust函数带参数
fn another_function_with_args(x: i32) {
    println!("The value of x is: {x}");
}

/// rust函数带两个参数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/// 带返回值的函数
fn function_with_return_value1() -> i32 {
    println!("带返回值的函数");
    5
}

/// 带返回值的函数
fn function_with_return_value2() -> i32 {
    println!("带返回值的函数在return之前");
    return 5;
    println!("带返回值的函数在return之后");
}

/// 带返回值的函数
fn function_with_return_value3() -> i32 {
    let x = 12;
    println!("带返回值的函数在return之前");
    x
}

// fn function_with_return_value() -> i32 {
//     println!("带返回值的函数");
//     5;
// }
