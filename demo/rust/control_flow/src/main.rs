fn main() {

    // 1. if条件语句
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 2. 使用 else if 处理多重条件
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 3.在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // 4. 使用 loop 重复执行代码
    let mut x = 0;
    loop {
        println!("again! {}", x);
        x += 1;
        if x > 12 {
            break;
        }
    }

    // 5. 从循环返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
    // 你可以选择在一个循环上指定一个 循环标签（loop label），
    // 然后将标签与 break 或 continue 一起使用，
    // 使这些关键字应用于已标记的循环而不是最内层的循环

    // 6. 循环标签：在多个循环之间消除歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // 7. while 条件循环
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // 8. 使用 for 遍历集合
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


}
