fn main() {
    let mut _c = 12;
    let _d = _c;

    // _d = 12;     // 编译报错
    let _d = 12;    // 编译通过

    // 定义常量
    const HOURS_IN_SECONDS: u32 = 60 * 60;

    println!("The value of _c is: {_c}");
    println!("The value of _d is: {_d}");

    println!("The value of HOURS_IN_SECONDS is: {HOURS_IN_SECONDS}");
}
