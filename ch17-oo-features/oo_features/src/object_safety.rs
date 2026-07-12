// ===== 17.2 对象安全（Object Safety）=====
// 用 cargo run --bin object_safety 运行

use std::fmt;

// ========================================================
// 什么是对象安全？
// ========================================================
// 不是所有 trait 都能做成 trait 对象（dyn Trait）
// 一个 trait 是「对象安全」的，必须满足两条规则：
//   1. 方法不能返回 Self 类型
//   2. 方法不能有泛型类型参数
//   （所有方法必须能用 vtable 实现）

// ---------- ✅ 对象安全的 trait ----------
pub trait Draw {
    fn draw(&self);   // 接收 &self，无 Self 返回，无泛型 → 安全
}

// ---------- ✅ 另一个对象安全的 trait ----------
pub trait Greet: fmt::Display {
    fn greet(&self) {
        println!("Hello, {}!", self);
    }
}

// ---------- ❌ 不对象安全的 trait ----------
// trait Clone {
//     fn clone(&self) -> Self;   // ❌ 返回 Self！
// }
// 为什么不安全？trait 对象不知道具体类型，无法返回 Self
// （dyn Clone 不知道自己是 Button 还是 TextBox）

// ---------- ❌ 另一个不对象安全的 trait ----------
// trait MyTrait {
//     fn do_it<T>(&self, x: T);   // ❌ 有泛型参数！
// }
// 为什么不安全？泛型会单态化，每种 T 生成一个方法
// vtable 无法容纳无限多的方法

// ========================================================
// 验证：对象安全的 trait 可以做 trait 对象
// ========================================================

struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Greet for Point {}

fn main() {
    println!("===== 1. 对象安全的 trait 可以做 dyn =====\n");

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    // ✅ Draw 是对象安全的
    // （但 Point 没实现 Draw，这里用 Greet 演示）
    let items: Vec<Box<dyn Greet>> = vec![
        Box::new(p1),
        Box::new(p2),
    ];
    for item in &items {
        item.greet();
    }

    println!("\n===== 2. 不对象安全的 trait 不能做 dyn =====\n");

    // ❌ Clone 不对象安全（返回 Self）
    // let clones: Vec<Box<dyn Clone>> = vec![...];   // 编译错误
    println!("❌ let v: Vec<Box<dyn Clone>> = ...");
    println!("   错误：the trait Clone cannot be made into an object");
    println!("   原因：clone() 返回 Self，dyn 不知道具体类型");

    // ❌ 有泛型方法的 trait
    println!();
    println!("❌ trait with fn f<T>(&self) 不能做 dyn");
    println!("   原因：泛型会单态化，vtable 装不下无限方法");

    println!("\n===== 3. 对象安全的两条规则 =====\n");

    println!("trait 要对象安全，必须满足：");
    println!();
    println!("规则 1：方法不能返回 Self 类型");
    println!("  ❌ fn clone(&self) -> Self");
    println!("  ✅ fn draw(&self)");
    println!();
    println!("规则 2：方法不能有泛型类型参数");
    println!("  ❌ fn f<T>(&self, x: T)");
    println!("  ✅ fn f(&self, x: i32)");
    println!();
    println!("（隐含：所有方法能用固定大小的 vtable 表示）");

    println!("\n===== 4. 标准库中对象安全的 trait =====\n");

    println!("| trait        | 对象安全？ | 原因                     |");
    println!("|-------------|----------|------------------------|");
    println!("| Display      | ✅       | 无 Self 返回，无泛型      |");
    println!("| Debug        | ✅       | 无 Self 返回，无泛型      |");
    println!("| Write        | ✅       | 无 Self 返回，无泛型      |");
    println!("| Iterator     | ❌       | next() 返回 Self::Item    |");
    println!("| Clone        | ❌       | clone() 返回 Self         |");
    println!("| Default      | ❌       | default() 返回 Self       |");
    println!("| Sized        | ❌       | 标记 trait，与 dyn 冲突    |");

    println!("\n===== 5. 遇到不对象安全怎么办？=====\n");

    println!("方案 1：重构 trait，去掉不安全的方法");
    println!("方案 2：用泛型代替 trait 对象（静态分发）");
    println!("方案 3：把不安全的方法拆到另一个 trait");

    // 方案 3 例子
    println!();
    println!("// 把不安全的方法拆出去：");
    println!("trait Draw {{ fn draw(&self); }}        // 安全，可做 dyn");
    println!("trait Cloneable: Draw {{ fn clone_box(&self) -> Box<dyn Draw>; }}");
    println!("// clone_box 返回 Box<dyn Draw> 而不是 Self → 安全");

    println!("\n===== 6. 检查 trait 是否对象安全 =====\n");

    // 可以用编译器检查：如果 trait 不安全，写 dyn Trait 会报错
    // 错误信息会告诉你「the trait X cannot be made into an object」
    // 并指出哪些方法导致不安全

    println!("方法：尝试写 Box<dyn YourTrait>，编译器会告诉你是否安全");
}

// 方案 3 的例子：用 clone_box 代替 clone
pub trait DrawClone {
    fn clone_box(&self) -> Box<dyn DrawClone>;
}

impl DrawClone for Point {
    fn clone_box(&self) -> Box<dyn DrawClone> {
        Box::new(Point { x: self.x, y: self.y })
    }
}
// 这样返回 Box<dyn DrawClone> 而不是 Self，保持对象安全
