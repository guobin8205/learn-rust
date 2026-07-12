// ===== 15.4 RefCell<T> 内部可变性 =====
// 用 cargo run --bin refcell_t 运行

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("===== 1. 什么是内部可变性？=====\n");

    // 问题描述：
    //   有些场景下，你有一个「不可变」的值，但内部某些数据需要修改
    //   比如一个「注册到全局的回调函数」需要修改自己的状态
    //
    // RefCell 解决：把借用检查从「编译期」移到「运行时」

    println!("===== 2. RefCell 基础 =====\n");

    let cell = RefCell::new(5);
    println!("初始: {:?}", cell);

    // 借用检查在运行时
    {
        let mut borrowed = cell.borrow_mut();   // 可变借用（运行时检查）
        *borrowed = 10;
    }   // ← 必须在这里释放借用
    println!("修改后: {:?}", cell);

    // 多个不可变借用（OK）
    let b1 = cell.borrow();
    let b2 = cell.borrow();
    println!("两个不可变借用: {} {}", b1, b2);
    drop(b1); drop(b2);

    println!("\n===== 3. ⭐ 运行时借用检查的代价 =====\n");

    // ⚠️ 如果违反借用规则，运行时 panic（不是编译错误）
    // 下面两行取消注释会 panic：
    // let cell = RefCell::new(5);
    // let b1 = cell.borrow();       // 不可变借用
    // let b2 = cell.borrow_mut();   // ❌ 运行时 panic: already borrowed
    // println!("{}", b2);
    println!("  （取消上面注释会看到运行时 panic）");

    // 对比：普通引用的借用检查在编译期，会编译错误
    // 编译期检查更好（提前发现问题），但有些场景必须用运行时（如下面的 Messenger）

    println!("\n===== 4. 经典场景：MockMessenger（不可变接口，内部可变）=====\n");

    let mock = MockMessenger::new();
    let mut tracker = LimitTracker::new(&mock, 100);
    tracker.set_value(95);   // 应该触发 Warning
    tracker.set_value(105);  // 应该触发 Error

    println!("MockMessenger 记录的消息: {:?}", mock.sent_messages.borrow());

    println!("\n===== 5. Rc<RefCell<T>>：共享 + 可变 =====\n");

    use std::cell::RefCell;
    use std::rc::Rc;

    // 多个所有者 + 内部可变（Rust 单线程最强的组合）
    let shared_value = Rc::new(RefCell::new(0));
    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    *a.borrow_mut() = 10;    // 通过 a 修改
    *b.borrow_mut() += 5;    // 通过 b 修改（看到的是同一个数据）

    println!("共享可变值: {}", shared_value.borrow());   // 15

    println!("\n===== 6. 智能指针对比总结 =====\n");
    println!("| 指针            | 所有权 | 借用检查 | 可变性   |");
    println!("|----------------|--------|---------|----------|");
    println!("| Box<T>         | 唯一   | 编译期  | 自由     |");
    println!("| Rc<T>          | 共享   | 编译期  | 只读     |");
    println!("| RefCell<T>     | 唯一   | 运行时  | 内部可变 |");
    println!("| Rc<RefCell<T>> | 共享   | 运行时  | 共享可变 |");
}

// ========================================================
// 注册模式的经典例子（不可变接口，内部可变）
// ========================================================

pub trait Messenger {
    fn send(&self, msg: &str);   // ⚠️ 注意是 &self（不可变）
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: 超过最大值！");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Warning: 已达 90%");
        }
    }
}

// 用 RefCell 实现 Messenger（因为 trait 定义是 &self，但需要修改内部状态）
#[derive(Debug)]
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // ⭐ 用 RefCell 在「不可变 &self」里修改数据
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

// ========================================================
// ⭐ Rc<RefCell<T>>：共享 + 可变（最强大的组合）
// ========================================================

#[derive(Debug)]
enum RcfList {
    Cons(Rc<RefCell<i32>>, Rc<RcfList>),   // 共享 + 可变
    Nil,
}
use RcfList::*;

// ========================================================
// ⭐ 三大智能指针对比
// ========================================================
// | 指针        | 所有权     | 何时借用检查 | 可变性 |
// |------------|-----------|------------|--------|
// | Box<T>     | 唯一      | 编译期      | 自由   |
// | Rc<T>      | 共享       | 编译期      | 只读   |
// | RefCell<T> | 唯一      | 运行时      | 内部可变 |
// | Rc<RefCell<T>> | 共享   | 运行时      | 共享可变 |
// | Arc<T>     | 共享（线程安全）| 编译期 | 只读   |
// | Arc<Mutex<T>> | 共享（线程安全）| 运行时 | 共享可变 |
