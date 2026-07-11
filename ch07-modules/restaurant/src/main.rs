// ===== 第 7 章：模块系统演示 =====
// 用 cargo run 运行

// ========================================================
// 一、声明模块（mod）：告诉 Rust「有一个叫 xxx 的子模块」
// ========================================================
// 模块的两种定义方式：
//   方式 1：mod xxx { ... }  ← 直接在当前文件里写模块内容
//   方式 2：mod xxx;         ← 内容放在 src/xxx.rs 或 src/xxx/mod.rs 里

mod front_of_house;     // 方式 2：内容在 src/front_of_house.rs
mod back_of_house;      // 方式 2：内容在 src/back_of_house.rs

// ========================================================
// 二、use：把路径引入作用域，写起来更短
// ========================================================
use front_of_house::hosting;     // 引入后可以直接用 hosting::...
use back_of_house::Breakfast;    // 引入结构体

// ========================================================
// 三、crate 根：src/main.rs 就是 crate 的「根模块」
// ========================================================
// 在 main.rs 里声明的 mod，就是 crate 顶层模块
// main.rs / lib.rs 叫做「crate root」（crate 根）

fn main() {
    // ---------- 用绝对路径调用 ----------
    // 绝对路径：从 crate 根开始，用 crate:: 开头
    crate::front_of_house::hosting::add_to_waitlist();

    // ---------- 用 use 引入后，路径变短 ----------
    hosting::add_to_waitlist();   // 因为 use 了 front_of_house::hosting

    // ---------- 创建 Breakfast 实例 ----------
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");   // ✅ toast 是 pub 字段，能访问
    println!("早餐：toast={}", meal.toast);
    // println!("{}", meal.seasonal_fruit);  // ❌ 私有字段，编译错误（演示可见性）

    // ---------- 访问枚举 ----------
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("前菜：{:?} {:?}", order1, order2);

    // ---------- 模块路径演示 ----------
    println!("\n===== 模块树结构 =====");
    println!("crate (main.rs)");
    println!("├── front_of_house");
    println!("│   ├── hosting");
    println!("│   │   ├── add_to_waitlist()  [pub]");
    println!("│   │   └── seating_at_table() [私有]");
    println!("│   └── serving");
    println!("│       ├── take_order()       [私有]");
    println!("│       └── serve_order()      [私有]");
    println!("└── back_of_house");
    println!("    ├── Appetizer (enum)      [pub]");
    println!("    ├── Breakfast (struct)    [pub，字段可私有]");
    println!("    └── fix_incorrect_order() [pub]");
}

// ========================================================
// 四、pub 关键字：控制可见性（核心！）
// ========================================================
// 默认情况下，Rust 里所有东西都是「私有」的！
// 要让外部能访问，必须加 pub。
//
// | 位置                | 默认可见性 | 加 pub 后            |
// |---------------------|-----------|---------------------|
// | 函数 fn             | 私有       | 公开                 |
// | 结构体 struct       | 私有       | 公开（但字段仍私有）   |
// | 结构体字段           | 私有       | 需要单独给字段加 pub  |
// | 枚举 enum           | 私有       | 公开（所有变体也公开）|
// | 模块 mod            | 私有       | 公开                 |
// | 常量 const          | 私有       | 公开                 |
//
// ⚠️ struct 和 enum 的 pub 行为不同！
//   struct 加 pub → 公开类型本身，但字段默认私有（要逐个加 pub）
//   enum 加 pub   → 公开类型，所有变体自动公开
