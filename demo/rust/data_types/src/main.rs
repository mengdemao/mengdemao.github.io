
fn main() {
    // 整形

    let a:i8 = 12;
    let b:u8 = 12;
    println!("{} {}", a, b);

    let c:i16 = 12;
    let d:u16 = 12;
    println!("{} {}", c, d);

    let e:i32 = 12;
    let f:u32 = 12;
    println!("{} {}", e, f);

    let g:i64 = 12;
    let h:u64 = 12;
    println!("{} {}", g, h);

    let i:i128 = 12;
    let j:u128 = 12;
    println!("{} {}", i, j);

    let k:isize = 12;
    let l:usize = 12;
    println!("{} {}", k, l);

    // 浮点型

    let m:f32 = 12.1;
    let n:f64 = 12.2;
    println!("{} {}", m, n);

    // 布尔型

    let o = true;
    let p = false;
    println!("{} {}", o, p);

    // 字符型

    let r = 'M';
    let s = '孟';
    let t = '😃';
    println!("{} {} {}", r, s, t);

    // tuple
    let tup:(i32, i64, i128) = (500, 1000, 1500);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // array
    let months = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "Jun",
        "Jly",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec"
    ];
    let mount_1st = months[0];
    let mount_2nd = months[2];
    println!("mount_1st:{mount_1st} mount_2nd:{mount_2nd}");
}
