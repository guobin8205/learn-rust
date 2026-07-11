// ===== 3.4 控制流 (Control Flow) =====
// 用 cargo run --bin control_flow 运行本文件

fn main() {
    // ========================================================
    // 一、if 表达式
    // ========================================================

    // ---------- ① 基本用法 ----------
    let number = 6;
    if number % 4 == 0 {
        println!("能被 4 整除");
    } else if number % 3 == 0 {
        println!("能被 3 整除");
    } else {
        println!("既不能被 4 也不能被 3 整除");
    }

    // ---------- ② if 是表达式，可用于赋值 ----------
    let condition = true;
    let n = if condition { 5 } else { 6 };
    println!("if 表达式: n = {n}");

    // ⚠️ if 各分支的类型必须相同！
    // let bad = if condition { 5 } else { "six" };  // ❌ 类型不匹配

    // ⚠️ 条件必须是 bool，不会自动转换
    // let x = 1;
    // if x { ... }  // ❌ 编译错误！不像 C/JS 会隐式转 bool

    // ========================================================
    // 二、循环：loop、while、for
    // ========================================================

    // ---------- ③ loop：无限循环，靠 break 退出 ----------
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("count 到 3，退出 loop");
            break;
        }
    }

    // loop 可以返回值（break 带值）—— 很实用！
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;   // break 带返回值，整个 loop 表达式的值
        }
    };
    println!("loop 的返回值: {result}");   // 20

    // ---------- ④ while：条件循环 ----------
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("发射！");

    // ---------- ⑤ for：遍历集合（最常用！）----------
    let arr = [10, 20, 30, 40, 50];
    // 遍历数组
    for element in arr {
        println!("元素: {element}");
    }

    // 遍历范围（Range）
    for i in 0..5 {        // 0,1,2,3,4（半开范围，不含 5）
        print!("{i} ");
    }
    println!();
    for i in 1..=3 {       // 1,2,3（包含范围）
        print!("{i} ");
    }
    println!();

    // 反向遍历
    for i in (1..4).rev() {
        print!("{i} ");    // 3 2 1
    }
    println!();

    // ========================================================
    // 三、循环对比（什么时候用哪个）
    // ========================================================
    // | 循环类型 | 适用场景                              |
    // |---------|---------------------------------------|
    // | loop    | 无限循环、需要 break 返回值           |
    // | while   | 基于条件的循环（条件不满足时停止）     |
    // | for     | 遍历集合/范围（最安全最常用）          |
    //
    // 💡 Rust 中 for 比遍历索引访问更安全高效（不会有越界 panic）
    //    尽量用 for 遍历，而不是 while + 索引
}
