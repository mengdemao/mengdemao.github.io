use std::collections::HashMap;

fn main() {

    // 1. 新建Hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2. 访问Hashmap的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 3. 更新Hashmap的值

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 4. 只在键没有对应值时插入键值对
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(30);
    println!("{:?}", scores);

    // 5. 统计词频
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
