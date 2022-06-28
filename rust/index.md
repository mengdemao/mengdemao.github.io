# Rust


RUST学习笔记

====

## 安装

1. 添加环境变量.bashrc/profile

```shell
set RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
set RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

2. 安装工具链

```shell
curl https://mirrors.ustc.edu.cn/rust-static/rustup/rustup-init.sh | sh
```

3. 设置rust的环境变量.bashrc/profile

```shell
source ~/.cargo/env
set PATH=~/.cargo/bin;$PATH
```
## 入门基础
### HelloWorld
1. 创建工程
```shell
cargo new hello_world
```

2. 编译
```shell
cargo build
```

3. 运行
```shell
cargo run
```

### 数据类型
| 长度    | 有符号 | 无符号 |
| ------- | ------ | ------ |
| 8-bit   | i8     | u8     |
| 16-bit  | i16    | u16    |
| 32-bit  | i32    | u32    |
| 64-bit  | i64    | u64    |
| 128-bit | i128   | u128   |
| arch    | isize  | usize  |

```rust
// 创建变量
let _xi32: i32 = 5;
let _xu32: u32 = 5;
let _xi64: i64 = 10;
let _xu64: u64 = 10;
let _xi128: i128 = 5;
let _xu128: u128 = 5;
let _xisize: isize = 10;
let _xusize: usize = 10;
```

### 函数
```rust
// 有返回值
fn function_return() -> i32 {
    println!("Hello, World!");
    return 0;
}

// 无返回值
fn function_noreturn() {
    println!("Hello, World!");
}
```
1. 必须明确表示是否存在返回值
2. 语法校验比较严格,

### 条件语句

```rust
if <cond> {
    do; 
}
```

```rust
if <cond> {
	Do1;
} else {
	Do2;
}
```

```rust
if <cond1> {
	Do1;
} else if <cond2> {
	Do2;
} else {
    Do3;
}
```
