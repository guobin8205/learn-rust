// ===== 3.1 变量与可变性 (Variables and Mutability) =====
// 用 cargo run --bin variables 运行本文件

// ① const 常量：不能用 mut，必须注明类型，命名全大写+下划线
const MAX_POINTS: u32 = 100_000;  // 下划线分隔数字，提高可读性

fn main() {
    // ---------- ② 变量默认不可变 ----------
    let x = 5;
    println!("x = {x}");
    // x = 6;  // ❌ 编译错误！默认不可变。取消注释会报错：cannot assign twice to immutable value `x`

    // ---------- ③ mut 可变变量 ----------
    let mut y = 5;
    println!("y = {y}");
    y = 6;  // ✅ 加了 mut 就可以修改
    println!("y = {y}");

    // ---------- ④ const 常量 ----------
    println!("常量 MAX_POINTS = {MAX_POINTS}");
    // 常量特点：
    //   1. 不能用 mut（常量永远不可变）
    //   2. 必须显式标注类型
    //   3. 只能绑定常量表达式，不能是运行时计算的结果（如函数返回值）
    //   4. 可以在任何作用域声明（包括全局）

    // ---------- ⑤ Shadowing（隐藏）----------
    let z = 5;
    let z = z + 1;     // shadowing：新建一个 z，值为 6
    {
        let z = z * 2; // 内层作用域再 shadowing，z = 12
        println!("内层作用域的 z = {z}");  // 12
    } // ← 内层的 z 在这里被 drop
    println!("外层的 z = {z}");  // 6（内层的 z 已结束，回到外层 z=6）

    // ---------- ⑥ Shadowing 的杀手锏：改变类型 ----------
    let spaces = "   ";       // &str 类型（字符串字面量）
    let spaces = spaces.len(); // usize 类型——shadowing 可以改变类型！
    println!("空格数 = {spaces}");

    // ---------- ⑦ 对比：用 mut 无法改变类型 ----------
    // let mut count = 5;
    // count = "five";  // ❌ 编译错误！类型不匹配（期望 i32，得到 &str）
    // 这正是 shadowing 存在的意义之一：当你需要对同一个值做类型转换时
    // 用 shadowing 可以复用变量名，而不用担心类型冲突

    // ---------- ⑧ let 和变量绑定的总结 ----------
    // | 特性        | let          | let mut      | const         |
    // |-------------|--------------|--------------|---------------|
    // | 可变性      | 不可变       | 可变         | 不可变        |
    // | 可重复绑定  | ✅(shadowing)| ✅(shadowing)| ❌            |
    // | 类型可变    | ✅(shadowing)| ✅(shadowing)| ❌            |
    // | 类型推断    | ✅           | ✅           | ❌ 必须标注   |
    // | 设置时机    | 运行时       | 运行时       | 编译时        |
}
