// ===== 13.1 闭包 (Closures) =====
// 用 cargo run --bin closures 运行

// ========================================================
// 闭包是什么？能捕获环境的匿名函数
// ========================================================

use std::thread;
// use std::time::Duration;

fn main() {
    println!("===== 1. 闭包基本语法 =====\n");

    // 普通函数（必须标注类型）
    fn add_fn(a: i32, b: i32) -> i32 { a + b }

    // 闭包（类型可推断，参数和返回值无需标注）
    let add_closure = |a, b| a + b;             // 最简形式
    let add_with_types = |a: i32, b: i32| -> i32 { a + b };  // 带类型（通常省略）

    println!("函数: {}", add_fn(1, 2));          // 3
    println!("闭包: {}", add_closure(1, 2));     // 3
    println!("带类型: {}", add_with_types(1, 2));

    // 闭包结构：|参数| { 函数体 }  或  |参数| 表达式
    // 无参数：|| { ... }
    let say_hi = || println!("Hi!");
    say_hi();

    println!("\n===== 2. 闭包的核心能力：捕获环境 ⭐ =====\n");

    // 闭包能「看到」并使用它定义时周围环境的变量！这是普通函数做不到的
    let x = 10;

    // 普通函数不能用 x（无法访问环境）
    // fn use_x() { println!("{}", x); }  // ❌ 普通函数不能捕获环境

    // 闭包可以！
    let add_x = |y| x + y;     // 闭包「捕获」了 x
    println!("add_x(5) = {}", add_x(5));   // 15（用了外部的 x=10）

    // 更复杂的例子
    let name = String::from("Alice");
    let greet = || println!("Hello, {name}!");   // 捕获了 name
    greet();

    println!("\n===== 3. 捕获的三种方式（Fn / FnMut / FnOnce）=====\n");

    // let mut count = 0;

    // ① 不可变借用（Fn）：只读访问
    let list = vec![1, 2, 3];
    let print_list = || println!("{:?}", list);    // 借用 list（只读）
    print_list();
    println!("list 仍可用: {:?}", list);   // ✅ list 没被消耗

    // ② 可变借用（FnMut）：可修改
    let mut counter = 0;
    let mut increment = || { counter += 1; };      // 可变借用 counter
    increment();
    increment();
    println!("counter = {}", counter);             // 2

    // ③ 获取所有权（FnOnce）：消耗掉变量
    let data = vec![1, 2, 3];
    let consume = move || {                         // move 强制获取所有权
        println!("消耗 data: {:?}", data);
    };
    consume();
    // println!("{:?}", data);   // ❌ data 被 move 进闭包，已消耗

    println!("\n===== 4. move 关键字：强制捕获所有权 =====\n");

    let s = String::from("hello");
    let closure = move || {
        println!("{}", s);   // move 让闭包拿走 s 的所有权
    };
    closure();
    // println!("{}", s);   // ❌ s 已被移动

    // move 在多线程中很有用（把数据所有权转移给新线程）
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {           // ⭐ 多线程必须用 move
        println!("线程中: {:?}", data);
    });
    handle.join().unwrap();

    println!("\n===== 5. 闭包作为参数（impl Fn）=====\n");

    // 闭包可以作为函数参数，用 impl Fn / FnMut / FnOnce 声明
    fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    let double = |n| n * 2;
    let square = |n| n * n;
    println!("apply double: {}", apply(double, 5));   // 10
    println!("apply square: {}", apply(square, 5));   // 25

    // ========================================================
    // ⭐ Fn / FnMut / FnOnce 的关系（重要）
    // ========================================================
    // | Trait  | 能做什么           | 调用次数 |
    // |--------|--------------------|---------|
    // | FnOnce | 消耗捕获的变量      | 只能 1 次 |
    // | FnMut  | 可变借用（能修改）  | 多次    |
    // | Fn     | 不可变借用（只读）  | 多次    |
    //
    // 层级：Fn ⊂ FnMut ⊂ FnOnce
    //   实现了 Fn 的必然也实现了 FnMut 和 FnOnce
    //   所以参数声明 impl Fn 最严格，impl FnOnce 最宽松

    println!("\n===== 6. 经典场景：模拟生成器（保存状态）=====\n");

    // 闭包可以「记住」状态，像个小对象
    let mut next_value = generate_counter(10);
    println!("{}", next_value());   // 10
    println!("{}", next_value());   // 11
    println!("{}", next_value());   // 12
}

// 返回一个闭包（需要 move 和 Box，第 17 章详讲）
// 这里只是演示闭包能保存状态
fn generate_counter(start: i32) -> Box<dyn FnMut() -> i32> {
    let mut count = start;
    Box::new(move || {
        let result = count;
        count += 1;
        result
    })
}
