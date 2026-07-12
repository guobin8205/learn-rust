// ===== 16.4 Send 和 Sync trait =====
// 用 cargo run --bin send_sync 运行

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("===== 1. Send：可以安全地在线程间「转移所有权」=====\n");

    // Send trait：表示类型 T 的所有权可以转移到另一个线程
    // 几乎所有类型都是 Send，除了：
    //   - Rc<T>（非原子引用计数）
    //   - 裸指针 *const T / *mut T
    //   - 等

    let s = String::from("hello");
    thread::spawn(move || {   // ✅ String 是 Send
        println!("线程内: {}", s);
    }).join().unwrap();

    // ⚠️ Rc 不是 Send
    let rc = Rc::new(5);
    // thread::spawn(move || {  // ❌ 编译错误：Rc 不是 Send
    //     println!("{}", rc);
    // });
    println!("Rc 不能跨线程（不是 Send）");

    println!("\n===== 2. Sync：可以安全地在线程间「共享引用」=====\n");

    // Sync trait：表示 &T 可以安全地在多个线程间共享
    // 如果 T 是 Sync，&T 就可以跨线程使用

    // Mutex<T> 是 Sync（内部有锁，保证安全）
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {   // ✅ Mutex 是 Sync
        *data_clone.lock().unwrap() = 42;
    }).join().unwrap();
    println!("Mutex 跨线程共享: {}", data.lock().unwrap());

    // ⚠️ RefCell 不是 Sync
    let cell = RefCell::new(5);
    // thread::spawn(move || {  // ❌ RefCell 不是 Sync
    //     *cell.borrow_mut() = 10;
    // });
    println!("RefCell 不能跨线程（不是 Sync）");

    println!("\n===== 3. ⭐ Send 和 Sync 的关键区别 =====\n");

    println!("| trait | 含义                       | 何时需要               |");
    println!("|-------|---------------------------|------------------------|");
    println!("| Send  | 可以「转移所有权」到另一个线程 | thread::spawn 的闭包需要 |");
    println!("| Sync  | 可以「共享 &T」到多个线程    | Arc<T> 需要 T: Sync     |");

    println!("\n===== 4. 自动推导 =====\n");

    // Send 和 Sync 都是「标记 trait」（marker trait），没有方法
    // 它们是编译器的「标签」，根据组成自动推导：
    //
    // 如果一个 struct 的所有字段都是 Send，那 struct 也是 Send
    // 如果一个 struct 的所有字段都是 Sync，那 struct 也是 Sync

    println!("struct MyData {{ a: i32, b: String }}    → 自动 Send + Sync");
    println!("struct MyData {{ rc: Rc<i32> }}         → 自动不是 Send/Sync");
    println!("struct MyData {{ mutex: Mutex<i32> }}   → 自动 Send + Sync");

    println!("\n===== 5. unsafe 实现不安全的 Send/Sync =====\n");

    // 编译器自动推导，但你可以用 unsafe 强制实现（很危险！）
    // 通常只有库作者需要，普通开发者不用碰
    //
    // unsafe impl Send for MyType {}
    // unsafe impl Sync for MyType {}
    println!("（普通开发者不用手动实现 Send/Sync）");

    println!("\n===== 6. 常见类型的 Send/Sync 表 =====\n");

    println!("| 类型           | Send | Sync | 说明                  |");
    println!("|---------------|------|------|----------------------|");
    println!("| i32, String   | ✅   | ✅   | 基础类型             |");
    println!("| Vec<T>        | ✅   | ✅   |                       |");
    println!("| Box<T>        | ✅*  | ✅*  | 取决于 T             |");
    println!("| Rc<T>         | ❌   | ❌   | 非原子引用计数        |");
    println!("| Arc<T>        | ✅   | ✅*  | 原子引用计数          |");
    println!("| RefCell<T>    | ✅   | ❌   | 运行时借用检查        |");
    println!("| Mutex<T>      | ✅   | ✅   | 加锁保护              |");
    println!("| RwLock<T>     | ✅   | ✅   | 读写锁                |");
    println!();
    println!("* 取决于内部类型 T 的属性");

    println!("\n===== 7. 完整对比：单线程 vs 多线程的智能指针选择 =====\n");

    println!("| 场景           | 单线程           | 多线程              |");
    println!("|---------------|-----------------|--------------------|");
    println!("| 唯一所有权     | Box<T>          | Box<T>             |");
    println!("| 共享只读       | Rc<T>           | Arc<T>             |");
    println!("| 内部可变       | RefCell<T>      | Mutex<T>/RwLock<T>|");
    println!("| 共享可变       | Rc<RefCell<T>>  | Arc<Mutex<T>>      |");

    // ========================================================
    // ⭐ 无畏并发的核心
    // ========================================================
    // Rust 的并发安全在「编译期」就检查了：
    //   1. thread::spawn 要求闭包是 Send
    //   2. Arc<T> 要求 T 是 Send + Sync
    //   3. 所有权的 move 语义防止数据竞争
    //
    // 你不可能写出「数据竞争」的代码，因为编译器会阻止！
    // 这就是「无畏并发（Fearless Concurrency）」的含义
}
