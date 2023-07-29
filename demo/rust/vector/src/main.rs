fn main() {

    // 1. 新建vector
    let _v: Vec<i32> = Vec::new();

    // 2. 使用宏来初始化vector
    let _v = vec![1, 2, 3];

    // 3. 更新vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 4. 读取vector
    println!("v[0] {}", v[0]); // 5
    println!("v[1] {}", v[1]); // 5

    println!("v[0] {}", &v[0]); // 5
    println!("v[1] {}", &v[1]); // 5

    println!("v[0] {:?}", v.get(0)); // Some(5)
    println!("v[1] {:?}", v.get(1)); // Some(6)

    // 6. 遍历 vector 中的元素
    for i in &v {
        println!("{}", i);
    }

}
