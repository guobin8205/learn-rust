// ===== 10.2 Trait：定义共享行为 =====
// 用 cargo run --bin trait_t 运行

use std::fmt;

// ========================================================
// 一、Trait 是什么？
// ========================================================
// Trait 类似其他语言的「接口（Interface）」
// 定义一组方法签名，让不同类型实现这些方法

// ---------- 定义 Trait ----------
trait Summary {
    // ① 必须实现的方法（只有签名）
    fn summarize(&self) -> String;

    // ② 默认实现（可以选择重写，也可以直接用）
    fn preview(&self) -> String {
        format!("预览: {}...", &self.summarize()[..self.summarize().len().min(10)])
    }
}

// ---------- 为不同类型实现同一个 Trait ----------
struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.title, self.author, self.content)
    }
}

impl Summary for Tweet {
    // 重写 summarize
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
    // 不重写 preview，直接用默认实现
}

// ========================================================
// 二、Trait 作为参数（三种写法）
// ========================================================

// 写法 1：impl Trait 语法（最简单）
fn notify(item: &impl Summary) {
    println!("速报: {}", item.summarize());
}

// 写法 2：Trait bound 语法（完整形式，impl Trait 是它的语法糖）
fn notify_bound<T: Summary>(item: &T) {
    println!("速报(bound): {}", item.summarize());
}

// 多参数 + 多约束
fn notify_both(a: &impl Summary, b: &impl Summary) {
    println!("{} | {}", a.summarize(), b.summarize());
}

// 要求两个参数是同一类型（用 Trait bound）
fn notify_same<T: Summary>(a: &T, b: &T) {
    println!("{} | {}", a.summarize(), b.summarize());
}

// ========================================================
// 三、用 + 指定多个 Trait 约束
// ========================================================
fn notify_display(a: &(impl Summary + fmt::Display)) {
    println!("{}", a);
}

// Trait bound + 多约束
fn notify_all<T: Summary + fmt::Display>(a: &T) {
    println!("{}", a);
}

// ========================================================
// 四、where 子句（约束多时更清晰）
// ========================================================
// 约束多时函数签名很长：
// fn some_fn<T: Summary + fmt::Display, U: Clone + fmt::Debug>(a: &T, b: &U) -> String
//
// 用 where 更易读：
#[allow(dead_code)]
fn some_fn<T, U>(a: &T, b: &U) -> String
where
    T: Summary + fmt::Display,
    U: Clone + fmt::Debug,
{
    format!("{} {:?}", a, b)
}

// ========================================================
// 五、返回实现了 Trait 的类型
// ========================================================
fn create_summary() -> impl Summary {
    // 注意：只能返回「一种」具体类型，不能根据条件返回不同类型
    Tweet {
        username: String::from("horse_ebooks"),
        text: String::from("of course, as you probably already know"),
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("ebooks"),
        text: String::from("hello world"),
    };
    let news = NewsArticle {
        title: String::from("标题"),
        author: String::from("作者"),
        content: String::from("内容"),
    };

    notify(&tweet);
    notify(&news);

    println!("预览: {}", tweet.preview());   // 默认实现

    let s = create_summary();
    println!("创建的: {}", s.summarize());

    // ========================================================
    // 六、Trait Object（dyn Trait）预览（第 17 章详讲）
    // ========================================================
    // 把不同类型都当作「实现了 Summary 的东西」放进同一个 Vec
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Tweet { username: String::from("a"), text: String::from("b") }),
        Box::new(NewsArticle { title: String::from("t"), author: String::from("x"), content: String::from("c") }),
    ];
    for item in &items {
        println!("集合元素: {}", item.summarize());
    }

    println!("\n===== Trait 速查 =====");
    println!("trait Name {{ fn method(&self); }}           // 定义");
    println!("impl Trait for Type {{ ... }}                // 实现");
    println!("fn f(x: &impl Trait)                         // 参数");
    println!("fn f<T: Trait>(x: &T)                        // Trait bound");
    println!("fn f<T: A + B>(x: &T)                        // 多约束");
    println!("where T: A + B                               // where 子句");
    println!("fn f() -> impl Trait                         // 返回");
}

// ========================================================
// ⭐ Trait vs 其他语言的接口
// ========================================================
// | 语言       | 类似概念      | 区别                       |
// |-----------|--------------|---------------------------|
// | Java      | interface    | 不能有默认实现（8 以前）     |
// | Go        | interface    | 隐式实现（不用声明）         |
// | Rust      | trait        | 可默认实现、可组合、可做约束  |
// | TypeScript| interface    | 结构化类型（鸭子类型）       |
//
// Rust trait 特点：
//   1. 可以有默认实现
//   2. 可以用 + 组合多个 trait
//   3. 可以作为泛型约束（trait bound）
//   4. 实现 trait 必须显式写 impl Trait for Type
//   5. trait object（Box<dyn Trait>）实现动态分发
