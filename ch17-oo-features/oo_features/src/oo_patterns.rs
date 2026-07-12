// ===== 17.3 面向对象模式 =====
// 用 cargo run --bin oo_patterns 运行

// ========================================================
// Rust 是不是面向对象语言？
// ========================================================
// 答案：取决于你如何定义「面向对象」
//
// 典型 OOP 特性：
//   1. 封装（Encapsulation）     ✅ Rust 有（pub/私有）
//   2. 继承（Inheritance）       ❌ Rust 没有（用 trait + 组合代替）
//   3. 多态（Polymorphism）      ✅ Rust 有（trait + 泛型/dyn）
//   4. 抽象（Abstraction）       ✅ Rust 有（trait）

// ========================================================
// 一、封装：隐藏实现细节
// ========================================================

pub struct AveragedCollection {
    list: Vec<i32>,        // 私有字段（默认）
    average: f64,          // 私有字段
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self { list: Vec::new(), average: 0.0 }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if result.is_some() {
            self.update_average();
        }
        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {   // 私有方法
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// 外部代码不能直接访问 list 和 average，必须通过 pub 方法
// 这就是封装：隐藏内部实现，只暴露接口

// ========================================================
// 二、多态：trait 实现（第 10 章详讲过，这里用 OOP 视角看）
// ========================================================

pub trait Animal {
    fn name(&self) -> String;
    fn make_sound(&self) -> String;
}

pub struct Dog { name: String }
pub struct Cat { name: String }

impl Animal for Dog {
    fn name(&self) -> String { self.name.clone() }
    fn make_sound(&self) -> String { String::from("汪汪") }
}

impl Animal for Cat {
    fn name(&self) -> String { self.name.clone() }
    fn make_sound(&self) -> String { String::from("喵喵") }
}

// ========================================================
// 三、状态模式（OOP 经典设计模式，用 Rust 实现）
// ========================================================

// OOP 版本用继承实现状态模式：
//   abstract class State { virtual State* approve() = 0; }
//   class Draft : public State { State* approve() { return new Review; } }
//
// Rust 版本用 trait + 结构体 enum 实现更安全：
// 每个状态是独立类型，转换通过返回新类型实现

// （简化版状态模式：用 enum + match，第 6 章学过的方式更地道）
pub struct Post {
    state: Option<Box<dyn State>>,   // 当前状态
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // 只有 state 的 request_add 方法会调用 content 修改
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self   // Draft 不能直接 approve，保持原状态
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content   // 只有 Published 返回内容
    }
}

fn main() {
    println!("===== 1. 封装 =====\n");
    let mut c = AveragedCollection::new();
    c.add(10);
    c.add(20);
    c.add(30);
    println!("平均值: {}", c.average());
    // c.list   // ❌ 私有，不能访问
    // c.average = 100.0;   // ❌ 私有
    println!("（list 和 average 字段都是私有的）");

    println!("\n===== 2. 多态 =====\n");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("旺财") }),
        Box::new(Cat { name: String::from("咪咪") }),
    ];
    for a in &animals {
        println!("{}: {}", a.name(), a.make_sound());
    }

    println!("\n===== 3. 状态模式（OOP 经典设计模式）=====\n");

    let mut post = Post::new();
    post.add_text("我吃了一顿沙拉");
    println!("Draft 时 content: '{}'", post.content());   // 空

    post.request_review();
    println!("Review 时 content: '{}'", post.content());   // 还是空

    post.approve();
    println!("Published 时 content: '{}'", post.content()); // 现在有了

    println!("\n===== 4. Rust 的 OOP 特性总结 =====\n");

    println!("| OOP 特性   | Rust 支持？ | 实现方式               |");
    println!("|-----------|----------|-----------------------|");
    println!("| 封装       | ✅       | pub / 私有字段方法     |");
    println!("| 继承       | ❌       | 用 trait + 组合代替    |");
    println!("| 多态       | ✅       | trait + 泛型/dyn       |");
    println!("| 抽象       | ✅       | trait 定义接口          |");

    println!("\n===== 5. Rust 没有继承，用什么代替？=====\n");

    println!("OOP 用继承解决的问题，Rust 用其他方式：");
    println!();
    println!("① 代码复用：OOP 用继承，Rust 用「组合 + trait 默认方法」");
    println!("② 多态：OOP 用虚函数，Rust 用 trait + 泛型/dyn");
    println!("③ 类型系统：OOP 用子类型，Rust 用 trait bound");

    println!("\n===== 6. 为什么 Rust 选择不继承？=====\n");
    println!("继承的问题：");
    println!("  - 耦合重（子类依赖父类实现）");
    println!("  - 单继承局限（多继承复杂如 C++）");
    println!("  - 违背「组合优于继承」原则");
    println!();
    println!("Rust 的方案：");
    println!("  - trait 定义接口（解耦）");
    println!("  - 组合代替继承（灵活）");
    println!("  - 泛型 + dyn 实现多态（零成本或动态）");
}
