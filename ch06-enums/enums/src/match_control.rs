// ===== 6.3 match 控制流运算符 =====
// 用 cargo run --bin match_control 运行

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),   // 变体携带数据
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
}

fn main() {
    // ========================================================
    // 一、match：模式匹配（Rust 最强大的控制结构）
    // ========================================================
    let coin = Coin::Quarter(UsState::Alaska);
    println!("硬币价值: {} 美分", value_in_cents(coin));

    // ========================================================
    // 二、match 绑定变体内部的值 ⭐
    // ========================================================
    let coin2 = Coin::Quarter(UsState::NewYork);
    if let Coin::Quarter(state) = coin2 {
        println!("这是来自 {:?} 的 25 美分硬币", state);
    }

    // ========================================================
    // 三、match 处理 Option<T>（经典用法）
    // ========================================================
    let five = Some(5);
    let six = plus_one(five);     // Some(6)
    let none = plus_one(None);    // None
    println!("five + 1 = {:?}, None + 1 = {:?}", six, none);

    // ========================================================
    // 四、⭐ match 必须穷尽所有可能（exhaustive）
    // ========================================================
    // 漏掉任何一个分支 → 编译错误！这是 Rust 安全性的保证
    // 比如下面的 plus_one 如果漏掉 None 分支：
    //   fn plus_one(x: Option<i32>) -> Option<i32> {
    //       match x {
    //           Some(i) => Some(i + 1),
    //           // ❌ 缺少 None 分支 → non-exhaustive patterns 编译错误
    //       }
    //   }

    // ========================================================
    // 五、通配符 _ 和 占位符
    // ========================================================
    let dice = 6;
    match dice {
        3 => println!("掷出了 3，加分！"),
        6 => println!("掷出了 6，大奖！"),
        _ => println!("掷出了 {}，普通", dice),   // _ 匹配其他所有情况
    }

    // ---------- _ 绑定 vs _ 不绑定 ----------
    let dice2 = 5;
    match dice2 {
        3 => println!("3"),
        n => println!("不是 3，是 {}", n),   // n 绑定了值，可以使用
    }

    match dice2 {
        3 => println!("3"),
        _ => println!("不是 3"),            // _ 不绑定，想忽略值时用
    }

    // ========================================================
    // 六、if let：只关心一种情况的简写
    // ========================================================
    // 当 match 只有两个分支，其中一个用 _，用 if let 更简洁

    let config_max = Some(3u8);

    // 用 match 写：
    match config_max {
        Some(max) => println!("最大值是 {}", max),
        _ => (),
    }

    // 用 if let 写（更简洁）：
    if let Some(max) = config_max {
        println!("最大值是 {}", max);
    }

    // ---------- if let 配合 else ----------
    let coin3 = Coin::Dime;
    let mut count = 0;
    if let Coin::Quarter(state) = coin3 {
        println!("25 美分来自 {:?}", state);
    } else {
        count += 1;     // 不是 Quarter 的情况
        println!("不是 25 美分，count = {count}");
    }

    // ========================================================
    // 七、match 是表达式，有返回值
    // ========================================================
    let boolean = true;
    let text = match boolean {
        true => "是",
        false => "否",
    };
    println!("boolean = {text}");

    // 各分支的返回值类型必须相同！
    // let bad = match boolean {
    //     true => 1,
    //     false => "no",   // ❌ 类型不匹配
    // };

    // ========================================================
    // 八、模式匹配的强大之处（预览，第 18 章详讲）
    // ========================================================
    // ---------- 解构结构体 ----------
    struct Point { x: i32, y: i32 }
    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;   // 解构
    println!("x={x}, y={y}");

    // ---------- 解构枚举（match 的本质）----------
    let msg = Message::Write(String::from("hi"));
    match msg {
        Message::Quit => println!("退出"),
        Message::Write(text) => println!("写入: {text}"),   // 绑定 text
        Message::Move { x, y } => println!("移动到 {x},{y}"),
        Message::ChangeColor(r, g, b) => println!("颜色 {r},{g},{b}"),
    }

    // ---------- 匹配范围 ----------

    let age = 25;
    match age {
        0..=12 => println!("儿童"),       // 范围匹配 0~12
        13..=19 => println!("青少年"),
        _ => println!("成年"),
    }

    // ========================================================
    // ⭐ match vs if let 怎么选
    // ========================================================
    // | 情况                       | 用什么       |
    // |---------------------------|-------------|
    // | 需要处理多个分支           | match       |
    // | 只关心一种情况，其他忽略   | if let      |
    // | 编译器强制穷尽检查很有用   | match       |
    // | 想要简洁                   | if let      |
    //
    // 💡 if let 是 match 的「语法糖」，不要求穷尽，代价是失去穷尽检查
}

enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {        // 匹配并绑定内部值
            println!("来自 {:?} 的 25 美分", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
