// ===== 第 11 章：测试演示项目 =====
// 这是一个 binary crate，调用 lib crate 的功能

use testing::{add, Greeting};

fn main() {
    println!("===== 测试项目演示 =====");

    // 使用 lib 里的函数
    println!("2 + 3 = {}", add(2, 3));
    println!("{} 欢迎你！", Greeting::new("Rust 学习者").format());
}
