// ===== 18.1 模式可以出现的位置 =====
// 用 cargo run --bin positions 运行

fn main() {
    println!("===== 1. match 分支 =====\n");
    // 第 6 章学过：match 的每个分支都是一个模式

    let x = 1;
    match x {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他"),
    }

    println!("\n===== 2. if let 条件表达式 =====\n");
    // if let 也是模式匹配

    let favorite: Option<i32> = Some(7);
    if let Some(n) = favorite {
        println!("喜欢的是 {}", n);
    } else {
        println!("没有喜欢的");
    }

    println!("\n===== 3. while let 条件循环 =====\n");
    // while let：模式匹配成功就持续循环

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    println!("\n===== 4. for 循环 =====\n");
    // for 循环里的变量也是模式

    let v = vec!['a', 'b', 'c'];
    // 解构元组（enumerate 产出 (索引, 值)）
    for (index, value) in v.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    println!("\n===== 5. let 语句（也是模式！）=====\n");
    // ⭐ 这是最容易被忽略的：let 语句本质也是模式匹配

    let (a, b, c) = (1, 2, 3);   // 元组解构
    println!("a={}, b={}, c={}", a, b, c);

    let x = 5;   // 这也是一个模式！x 匹配任何值
    // 等价于：let x @ _ = 5;

    // 忽略某些值
    let (_, y, _) = (1, 2, 3);   // 只取 y
    println!("只取 y={}", y);

    println!("\n===== 6. 函数参数 =====\n");
    // 函数参数也是模式！

    fn print_point(&(x, y): &(i32, i32)) {
        // 参数 &(x, y) 是模式，解构引用的元组
        println!("点: ({}, {})", x, y);
    }
    let p = (10, 20);
    print_point(&p);

    // 更复杂的参数模式
    fn foo((x, y): (i32, i32)) { println!("{}, {}", x, y); }
    foo((1, 2));

    println!("\n===== 7. 闭包参数 =====\n");
    // 闭包参数和函数参数一样，也是模式

    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    pairs.iter().for_each(|(num, name)| {
        println!("{}: {}", num, name);
    });

    println!("\n===== 8. 模式位置总结 =====\n");

    println!("| 位置              | 例子                          |");
    println!("|------------------|------------------------------|");
    println!("| match 分支        | match x {{ 1 => ..., _ => ... }} |");
    println!("| if let            | if let Some(v) = x {{}}        |");
    println!("| while let         | while let Some(v) = x.pop() {{}} |");
    println!("| for 循环          | for (i, v) in x.enumerate()   |");
    println!("| let 语句          | let (a, b) = tuple            |");
    println!("| 函数参数          | fn f((a, b): (i32, i32))      |");
    println!("| 闭包参数          | \\|(a, b)\\| ...                 |");

    // ⭐ 关键洞察：模式在 Rust 里无处不在
    //    几乎所有「绑定变量」的地方都可以用模式
}

// ========================================================
// 可反驳 vs 不可反驳模式
// ========================================================
// 不可反驳（irrefutable）：总能匹配成功
//   let x = 5;          // x 匹配任何值，必然成功
//   let (a, b) = (1, 2); // 元组解构，必然成功
//
// 可反驳（refutable）：可能匹配失败
//   if let Some(x) = opt {}   // opt 可能是 None
//   if let 5 = x {}           // x 可能不是 5
//
// 规则：
//   - let 语句、函数参数、for 循环：只能用不可反驳模式
//   - if let、while let、match：可以用可反驳模式
//
// 例子：
//   let Some(x) = opt;   // ❌ 编译错误！可反驳模式用在 let 里
//   if let Some(x) = opt { ... }   // ✅ 正确，用 if let
