fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    // let _z = x + y;
    let _z = x + y.unwrap();
}
