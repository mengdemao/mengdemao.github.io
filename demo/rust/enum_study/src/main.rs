// 1. 定义枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 5. 枚举作为结构体的成员
#[derive(Debug)]
struct  IpAddr {
    kind : IpAddrKind,
    addr : String,
}

// 6. 将数据附加到枚举的变体中
#[derive(Debug)]
enum IpAddrWithType {
    V4(u8, u8, u8, u8),
    V6(String)
}

// 7. 枚举定义方法
#[derive(Debug)]
enum Message {
    Quit,
    Move {_x: i32, _y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

// 8.match 控制流结构
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    // 2. 实例化枚举
    let addr1 = IpAddrKind::V4;
    let addr2 = IpAddrKind::V6;

    println!("枚举学习开始");

    // 3. 简单枚举类型
    println!("{:#?}", addr1);
    println!("{:#?}", addr2);

    // 4. 调用枚举函数
    route(addr1);
    route(addr2);

    // 5. 枚举作为结构体的成员
    let home: IpAddr = IpAddr {
        kind : IpAddrKind::V4,
        addr : String::from("127.0.0.1")
    };

    let loopback: IpAddr = IpAddr {
        kind : IpAddrKind::V4,
        addr : String::from("::1")
    };

    println!("IpAddr {:#?}", home);
    println!("IpAddr {:#?}", loopback);

    println!("IpAddr {:#?} {}", home.kind, home.addr);
    println!("IpAddr {:#?} {}", loopback.kind, loopback.addr);

    // 6. 将数据附加到枚举的变体中
    let home: IpAddrWithType = IpAddrWithType::V4(127, 0, 0, 1);
    let loopback: IpAddrWithType = IpAddrWithType::V6(String::from("::1"));

    println!("IpAddrWithType {:#?}", home);
    println!("IpAddrWithType {:#?}", loopback);

    // 7. 枚举定义方法
    let g = Message::Quit;
    let m = Message::Move { _x: (12), _y: (24) };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    g.call();
    m.call();
    w.call();
    c.call();

    // 8.match 控制流结构
    println!("Coin::Penny {}", value_in_cents(Coin::Penny));
    println!("Coin::Nickel {}", value_in_cents(Coin::Nickel));
    println!("Coin::Dime {}", value_in_cents(Coin::Dime));
    println!("Coin::Quarter {}", value_in_cents(Coin::Quarter));

    println!("枚举学习结束");
}

fn route(ip_kind : IpAddrKind) {
    println!("{:#?}", ip_kind);
}