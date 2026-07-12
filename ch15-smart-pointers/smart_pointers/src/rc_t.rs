// ===== 15.3 Rc<T> 引用计数（多个所有者）=====
// 用 cargo run --bin rc_t 运行

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("===== 1. 为什么需要 Rc？=====\n");

    // 问题：所有权规则要求「只有一个所有者」
    // 但有时候多个地方需要「共享同一份数据」
    // Rc（Reference Counting）解决这个问题：允许多个所有者

    println!("===== 2. Rc 基础：共享数据 =====\n");

    // Rc::new 创建一个引用计数 = 1 的值
    let a = Rc::new(String::from("hello"));
    println!("count a = {}", Rc::strong_count(&a));   // 1

    // Rc::clone 增加引用计数（不是深拷贝！）
    let b = Rc::clone(&a);
    println!("count after b = {}", Rc::strong_count(&a));   // 2

    {
        let c = Rc::clone(&a);
        println!("count after c = {}", Rc::strong_count(&a));   // 3
        println!("b = {}, c = {}", b, c);   // 都能访问同一个数据
    }   // c 离开作用域，引用计数 -1
    println!("count after c drop = {}", Rc::strong_count(&a));   // 2

    // ⭐ Rc::clone 不是拷贝数据！只是增加引用计数
    //    所有 Rc 指向堆上同一份数据
    //    最后一个 Rc drop 时，数据才被释放

    println!("\n===== 3. 经典场景：共享节点（图/树结构）=====\n");

    // 两个 List 共享同一个 tail
    let shared_tail = Rc::new(List::Cons(5, Rc::new(List::Nil)));
    println!("共享节点创建后 count = {}", Rc::strong_count(&shared_tail));

    let list_a = Cons(3, Rc::clone(&shared_tail));   // 共享 tail
    let list_b = Cons(4, Rc::clone(&shared_tail));   // 也共享 tail
    println!("两个 list 共享后 count = {}", Rc::strong_count(&shared_tail));

    println!("list_a: {:?}", list_a);
    println!("list_b: {:?}", list_b);

    println!("\n===== 4. Rc 的限制：不可变 =====\n");

    // ⚠️ Rc 只提供共享只读访问
    // 我们不能通过 Rc 修改内部数据
    let data = Rc::new(5);
    // *data = 10;   // ❌ 不能修改 Rc 内部的值

    // 如果需要「共享 + 可变」怎么办？
    // → Rc<RefCell<T>>（下一节讲 RefCell）

    println!("\n===== 5. Rc 不是线程安全的 =====\n");

    // ⚠️ Rc 不能跨线程使用（引用计数不是原子操作）
    // let data = Rc::new(5);
    // thread::spawn(move || { println!("{}", data); });   // ❌ Rc 不实现 Send

    // 多线程用 Arc（Atomic Rc，原子引用计数）
    let data = Arc::new(100);
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("线程中: {}", data_clone);
    });
    handle.join().unwrap();
    println!("主线程: {}", data);

    println!("\n===== 6. Rc vs Arc 对比 =====\n");

    // | 特性       | Rc<T>          | Arc<T>           |
    // |-----------|----------------|------------------|
    // | 引用计数   | 普通整数         | 原子操作           |
    // | 线程安全   | ❌ 单线程        | ✅ 多线程          |
    // | 性能       | 略快            | 略慢（原子开销）    |
    // | 何时用     | 单线程共享        | 多线程共享          |

    println!("Rc: 单线程共享（快）");
    println!("Arc: 多线程共享（安全）");
}

// ========================================================
// 共享链表（用 Rc 让多个节点共享子节点）
// ========================================================
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),    // 用 Rc 让多个 Cons 共享同一个 tail
    Nil,
}
use List::*;

// ========================================================
// ⭐ 引用计数的本质
// ========================================================
// 堆：  ┌──────────────┐
//      │ "hello"       │ ← 真实数据
//      │ count = 3     │ ← 引用计数
//      └──────────────┘
//         ↑     ↑     ↑
// 栈：    a     b     c    ← 三个 Rc 指针都指向这里
//
// Rc::clone(&a) → 只是把 count 加 1（不拷贝数据）
// c 离开作用域 → count 减 1
// 最后一个 Rc 离开 → count 归 0 → 数据释放
//
// ⚠️ 这其实就是「垃圾回收」的一种形式（引用计数式 GC）
//    但和 Java/Go 的「追踪式 GC」不同，Rust 的引用计数是确定性的
