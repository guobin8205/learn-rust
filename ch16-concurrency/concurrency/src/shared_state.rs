// ===== 16.3 共享状态（Mutex + Arc）=====
// 用 cargo run --bin shared_state 运行
// ⭐ 这就是第 4 章问答里提到的「多线程共享可变数据」的正解

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("===== 1. Mutex：互斥锁 =====\n");

    // Mutex = Mutual Exclusion（互斥）
    // 保证同一时刻只有一个线程能访问内部数据
    let m = Mutex::new(5);

    {
        // lock() 返回 LockResult<MutexGuard>
        // MutexGuard 在 drop 时自动解锁
        let mut num = m.lock().unwrap();   // 🔒 上锁
        *num = 6;                          // 修改
    }                                       // ← MutexGuard drop → 自动解锁 🔓

    println!("m = {:?}", m);

    println!("\n===== 2. Mutex 的核心规则 =====\n");

    // 必须先 lock 才能访问数据
    // let n = *m;   // ❌ 不能直接访问！必须先 lock
    let n = *m.lock().unwrap();    // ✅
    println!("读: {}", n);

    // ========================================================
    // 为什么需要 Arc？（回顾第 15 章）
    // ========================================================
    // Mutex 不能跨线程共享（它不是 Sync）
    // 要多线程共享，需要 Arc（原子引用计数）
    // Arc<Mutex<T>> = 多线程共享 + 互斥访问

    println!("\n===== 3. ⭐ Arc<Mutex<T>>：多线程共享可变数据 =====\n");

    // 创建共享计数器
    let counter = Arc::new(Mutex::new(0));
    println!("初始值: {}", *counter.lock().unwrap());

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);   // 引用计数 +1（原子操作）
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();   // 🔒 上锁
            *num += 1;                                 // 安全修改
            // ← MutexGuard drop → 解锁 🔓
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("10 个线程各 +1 后: {}", *counter.lock().unwrap());   // 10（安全！）

    println!("\n===== 4. 对比：如果没有 Mutex 会怎样？=====\n");

    // 第 4 章问答里讲过：
    // ❌ Rc 不能跨线程
    // ❌ Arc 单独用不能修改数据
    // ✅ Arc<Mutex<T>> 才是正解

    println!("| 方式                 | 能多线程修改共享数据？ |");
    println!("|---------------------|---------------------|");
    println!("| Rc<T>               | ❌ 不能跨线程         |");
    println!("| Arc<T>              | ❌ 不能修改（只读）    |");
    println!("| Rc<RefCell<T>>      | ❌ 不能跨线程         |");
    println!("| Arc<Mutex<T>>       | ✅ 正解！             |");

    println!("\n===== 5. Mutex 的死锁陷阱 =====\n");

    // ⚠️ 死锁：两个线程互相等待对方的锁
    // 锁的顺序不一致会导致死锁

    // 例：两个锁，线程 1 先 A 后 B，线程 2 先 B 后 A → 死锁
    // let a = Arc::new(Mutex::new(0));
    // let b = Arc::new(Mutex::new(0));
    // 线程 1：lock(a) → lock(b)
    // 线程 2：lock(b) → lock(a)
    // 互相等待对方释放 → 死锁

    println!("死锁原因：");
    println!("  线程 1 持有 A 的锁，等 B");
    println!("  线程 2 持有 B 的锁，等 A");
    println!("  → 互相等待，永远卡住");
    println!();
    println!("预防：所有线程按相同顺序加锁");

    println!("\n===== 6. MutexGuard 的自动解锁（RAII）=====\n");

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // ⭐ Rust Mutex 的优雅之处：用作用域自动管理锁
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap();   // 🔒
        locked.push(4);                                 // 修改
        // locked 在这里 drop → 自动解锁 🔓
        // 即使中间 panic 也会自动解锁（MutexGuard 的 Drop 实现）
    });
    handle.join().unwrap();

    println!("结果: {:?}", data.lock().unwrap());   // [1, 2, 3, 4]

    // ========================================================
    // ⭐ Arc<Mutex<T>> 的代理模型（回顾第 15 章 Q4）
    // ========================================================
    // Arc = 线程安全的「共享代理」（原子引用计数）
    // Mutex = 「互斥代理」（加锁/解锁管理）
    // 组合 = 「线程安全共享 + 互斥可变」的双重代理
    //
    // 你的代码（10 个线程）
    //      │
    //      ↓ clone（增加引用计数）
    //   Arc（共享代理）──原子计数──→ 协调「何时释放」
    //      │
    //      ↓ lock
    //   Mutex（互斥代理）──加锁──→ 协调「谁能访问」
    //      │
    //      ↓
    //   实际数据（0 → 10）
}
