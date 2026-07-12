// ===== 18.2 模式语法全解 =====
// 用 cargo run --bin syntax 运行

#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("===== 1. 字面量匹配 =====\n");
    let x = 1;
    match x {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他"),
    }

    println!("\n===== 2. 命名变量（绑定）=====\n");
    match x {
        1 => println!("一"),
        n => println!("不是一，是 {}", n),   // n 绑定值
    }

    println!("\n===== 3. ⭐ 多模式（| 或）=====\n");
    let x = 3;
    match x {
        1 | 2 => println!("一或二"),
        3 | 4 | 5 => println!("三、四、五"),   // 多个或
        _ => println!("其他"),
    }

    println!("\n===== 4. ⭐ 范围匹配（..= ）=====\n");
    let x = 5;
    match x {
        1..=5 => println!("1 到 5"),        // 包含范围
        6..=10 => println!("6 到 10"),
        _ => println!("其他"),
    }

    // 字符范围
    let c = 'g';
    match c {
        'a'..='j' => println!("a 到 j"),
        'k'..='z' => println!("k 到 z"),
        _ => println!("其他"),
    }

    println!("\n===== 5. 解构结构体 =====\n");
    let p = Point { x: 5, y: 10 };

    // 简写（字段名和变量名相同）
    let Point { x, y } = p;
    println!("解构: x={}, y={}", x, y);

    // 在 match 里解构
    match p {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("y 轴, y={}", y),
        Point { x, y: 0 } => println!("x 轴, x={}", x),
        Point { x, y } => println!("普通点 ({}, {})", x, y),
    }

    println!("\n===== 6. 解构枚举 =====\n");
    let msg = Message::Move { x: 3, y: 7 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }

    println!("\n===== 7. 解构嵌套结构 =====\n");
    let ((a, b), Point { x, y }) = ((1, 2), Point { x: 3, y: 4 });
    println!("嵌套解构: a={}, b={}, x={}, y={}", a, b, x, y);

    println!("\n===== 8. ⭐ 解构引用（& 模式）=====\n");
    let points = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 2 }];
    let sum = points.iter().fold(0, |acc, &Point { x, y }| acc + x + y);
    //                          ↑ &Point{x,y} 解构引用
    println!("x+y 之和: {}", sum);

    println!("\n===== 9. ⭐ 忽略值（_ 和 ..）=====\n");

    // _ 忽略整个值
    let _ = 5;   // 完全忽略

    // _x 忽略但绑定（抑制未使用警告）
    let _x = 5;

    // 在模式里用 _ 忽略部分
    let (x, _) = (1, 2);   // 忽略第二个
    println!("只取 x={}", x);

    // .. 忽略剩余部分
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x={}", x),   // 忽略 y
    }

    // 元组中用 ..
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => println!("第一个 {}, 最后一个 {}", first, last),
    }

    println!("\n===== 10. ⭐ match 守卫（guard）=====\n");
    // 守卫：在模式后加 if 条件

    let num = 4;
    match num {
        x if x % 2 == 0 => println!("偶数 {}", x),   // 守卫：偶数
        x => println!("奇数 {}", x),
    }

    // 守卫可以解决「变量遮蔽」问题
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("是 50"),
        Some(n) if n == y => println!("等于 y ({})", n),   // 用守卫比较
        Some(n) => println!("不等于 y ({})", n),
        None => println!("None"),
    }

    println!("\n===== 11. ⭐ @ 绑定（同时匹配和绑定）=====\n");
    // @ 让你「测试一个值是否在范围内，同时绑定它」

    let age = 25;
    match age {
        n @ 0..=12 => println!("儿童: {}", n),      // 0-12 范围 + 绑定 n
        n @ 13..=19 => println!("少年: {}", n),
        n @ 20..=65 => println!("成年: {}", n),
        n => println!("老年: {}", n),
    }

    // @ 配合枚举
    enum Color { Rgb(u8, u8, u8), Hsv(u8, u8, u8) }
    let c = Color::Rgb(122, 17, 40);
    match c {
        Color::Rgb(r @ 0..=100, g, b) => {
            println!("R 较暗: r={}, g={}, b={}", r, g, b);
        }
        Color::Rgb(r, g, b) => {
            println!("R 较亮: r={}, g={}, b={}", r, g, b);
        }
        Color::Hsv(h, s, v) => {
            println!("HSV: {}, {}, {}", h, s, v);
        }
    }

    println!("\n===== 12. 语法速查 =====\n");
    println!("| 模式          | 含义                  |");
    println!("|--------------|-----------------------|");
    println!("| 字面量 5      | 匹配值 5              |");
    println!("| 变量 x        | 绑定到 x              |");
    println!("| a \\| b        | 匹配 a 或 b           |");
    println!("| 1..=5         | 范围 1 到 5           |");
    println!("| Struct{{x,y}}   | 解构结构体            |");
    println!("| Variant(x)    | 解构枚举              |");
    println!("| _             | 忽略                  |");
    println!("| ..            | 忽略剩余              |");
    println!("| x if cond     | 守卫（额外条件）       |");
    println!("| n @ 1..=5     | 范围匹配 + 绑定到 n    |");
}
