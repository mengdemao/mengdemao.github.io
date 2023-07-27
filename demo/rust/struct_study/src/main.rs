/// 使用struct关键字为整个struct命名
/// 在花括号内， 为所有的字段定义名称和类型
#[derive(Debug)]
struct User {
    username : String,
    password : String,
    sign_in_count : u64,
    active : bool,
}

/// 默认初始化user的函数
///
fn build_user(username : String, password : String) -> User {
    User {
        username : username,
        password : password,
        sign_in_count : 0,
        active : true,
    }
}

/// 默认初始化user的函数
/// 简写 >>>>
fn build_user_simplify(username : String, password : String) -> User {
    User {
        username,
        password,
        sign_in_count : 0,
        active : true,
    }
}

fn print_user(user : User) {
    println!("username {}", user.username);
    println!("password {}", user.password);
    println!("sign_in_count {}", user.sign_in_count);
    println!("active {}", user.active);
}

fn main() {
    println!("struct学习开始");

    // 1. 实例化结构体
    let user1 = User {
        username : String::from("Hello"),
        password : String::from("Text"),
        sign_in_count : 0,
        active : true,
    };

    // 2. 获取某个成员的值,使用点标记法
    println!("username {}", user1.username);
    println!("password {}", user1.password);
    println!("sign_in_count {}", user1.sign_in_count);
    println!("active {}", user1.active);

    // 3. 结构体整体不可以直接打印
    // 需要使用调试配置参数执行打印
    println!("User {:?}", user1);
    println!("User {:#?}", user1);

    // 4. 一旦结构体设置为可变的,那么所有的成员都是可变的
    let mut user2 = User {
        username : String::from("World"),
        password : String::from("Test"),
        sign_in_count : 12,
        active : false,
    };

    println!("username {}", user2.username);
    println!("password {}", user2.password);
    println!("sign_in_count {}", user2.sign_in_count);
    println!("active {}", user2.active);

    // 5. 单独修改成员
    user2.username = String::from("Tom");

    // 6. 结构体直接赋值
    user2 = user1;
    print_user(user2);

    // 7. 结构体作为返回值
    let user3 = build_user(String::from("Hello"), String::from("Test"));
    print_user(user3);

    // 8. 简化结构体
    let user3 = build_user_simplify(String::from("Hello"), String::from("Test"));
    print_user(user3);

    // 9. tuple struct
    // - Tuple struct整体有名字,但是元素没有名字
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 10. Unit-Like Struct(没有任何字段的)

    println!("struct学习结束");
}
