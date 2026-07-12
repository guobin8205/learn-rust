# 第 16 章：无畏并发 (Fearless Concurrency)

> 对应 the book 第 16 章
> 学习目标：掌握线程、消息传递、共享状态、Send/Sync trait

## 📂 示例代码
```bash
cd ch16-concurrency/concurrency
cargo run --bin threads       # 16.1 线程
cargo run --bin channels      # 16.2 消息传递
cargo run --bin shared_state  # 16.3 共享状态
cargo run --bin send_sync     # 16.4 Send/Sync
```

---

## ⭐ 什么是「无畏并发」？

> 其他语言：多线程 bug 在**运行时**随机出现，极难调试
> Rust：多线程 bug 在**编译时**就被挡住，根本无法编译

Rust 的所有权系统 + Send/Sync trait，让数据竞争在编译期就无所遁形——这就是「无畏并发」。

---

## 16.1 使用线程

### 创建线程：`thread::spawn`

```rust
use std::thread;

thread::spawn(|| {
    println!("子线程");
});
// ⚠️ 主线程结束时，子线程被强制终止（不管有没有跑完）
```

### 等待子线程：`join`

```rust
let handle = thread::spawn(|| {
    42
});
let result = handle.join().unwrap();   // 阻塞等待子线程完成
```

### ⭐ 跨线程传数据：`move` 闭包

```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {   // ⭐ move 把 data 移进线程
    println!("{:?}", data);
});
// println!("{:?}", data);   // ❌ data 已被 move
```

> ⚠️ 跨线程必须用 `move`，因为线程生命周期可能比创建它的函数长

---

## 16.2 消息传递（channel/mpsc）

### 基本用法

```rust
use std::sync::mpsc;   // mpsc = Multiple Producer, Single Consumer

let (tx, rx) = mpsc::channel();   // 创建管道：发送端 + 接收端

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();   // 发送（所有权转移）
});

let received = rx.recv().unwrap();   // 接收（阻塞等待）
```

### 用迭代器接收多条消息

```rust
for received in rx {   // rx 实现了 Iterator
    println!("{}", received);
}
// 所有发送端 drop 后，迭代结束
```

### 多生产者（mpsc 的 m）

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();    // 克隆发送端
let tx2 = tx.clone();

// 两个线程各自发送
thread::spawn(move || { tx1.send("A").unwrap(); });
thread::spawn(move || { tx2.send("B").unwrap(); });

drop(tx);   // ⚠️ drop 原始 tx，否则 channel 不关闭
for msg in rx { println!("{}", msg); }
```

### 同步 vs 异步 channel

| | 异步（默认） | 同步（sync_channel） |
|---|------------|-------------------|
| 缓冲 | 无限 | 固定大小 |
| 发送方阻塞 | 不阻塞 | 缓冲满时阻塞 |

```rust
let (tx, rx) = mpsc::sync_channel(2);   // 缓冲区大小 2
```

---

## 16.3 共享状态（Mutex + Arc）⭐

### Mutex：互斥锁

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();   // 🔒 上锁，返回 MutexGuard
    *num = 6;                          // 修改
}                                       // ← MutexGuard drop → 自动解锁 🔓
```

**核心**：`lock()` 返回的 `MutexGuard` 在离开作用域时**自动解锁**（RAII 模式）。

### ⭐ Arc<Mutex<T>>：多线程共享可变数据

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));   // 共享 + 互斥
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);  // 引用计数 +1
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();   // 🔒
        *num += 1;
    });                                           // 🔓
    handles.push(handle);
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());   // 10（安全！）
```

### 为什么必须 Arc<Mutex<T>>？

| 方式 | 多线程修改共享数据？ | 原因 |
|------|-------------------|------|
| `Rc<T>` | ❌ | 不能跨线程（不是 Send） |
| `Arc<T>` | ❌ | 不能修改（只读） |
| `Rc<RefCell<T>>` | ❌ | 不能跨线程 |
| **`Arc<Mutex<T>>`** | ✅ | **正解！** |

### 死锁陷阱

```rust
// ⚠️ 死锁：两个线程按不同顺序加锁
// 线程 1：lock(A) → lock(B)
// 线程 2：lock(B) → lock(A)
// 互相等待 → 永远卡住

// 预防：所有线程按相同顺序加锁
```

---

## 16.4 Send 和 Sync trait ⭐

### 两个核心 trait

| trait | 含义 | 何时需要 |
|-------|------|---------|
| `Send` | 可以**转移所有权**到另一线程 | `thread::spawn` 的闭包 |
| `Sync` | 可以**共享 &T** 到多线程 | `Arc<T>` 要求 T: Sync |

### 关键：不是所有类型都 Send/Sync

```rust
// Rc 不是 Send → 不能跨线程
let rc = Rc::new(5);
// thread::spawn(move || { rc });   // ❌ 编译错误

// RefCell 不是 Sync → 不能多线程共享 &RefCell
// Arc<RefCell<T>> 也是非法的
```

### 常见类型的 Send/Sync

| 类型 | Send | Sync |
|------|------|------|
| i32, String, Vec | ✅ | ✅ |
| Rc<T> | ❌ | ❌ |
| Arc<T> | ✅ | ✅* |
| RefCell<T> | ✅ | ❌ |
| Mutex<T> | ✅ | ✅ |

（`*` 取决于内部类型 T）

### 自动推导

Send/Sync 是「标记 trait」，编译器自动推导：
- struct 的所有字段都 Send → struct 也 Send
- struct 的所有字段都 Sync → struct 也 Sync

---

## ⭐ 单线程 vs 多线程的智能指针选择

| 场景 | 单线程 | 多线程 |
|------|--------|--------|
| 唯一所有权 | `Box<T>` | `Box<T>` |
| 共享只读 | `Rc<T>` | `Arc<T>` |
| 内部可变 | `RefCell<T>` | `Mutex<T>` / `RwLock<T>` |
| **共享可变** | `Rc<RefCell<T>>` | **`Arc<Mutex<T>>`** |

---

## 16.5 代理模型视角（回顾第 15 章 Q4）

```
你的代码（10 个线程）
     │ clone（原子引用计数）
     ↓
  Arc（共享代理）──原子计数──→ 协调「何时释放」
     │ lock
     ↓
  Mutex（互斥代理）──加锁──→ 协调「谁能访问」
     │
     ↓
  实际数据（0 → 10）
```

`Arc<Mutex<T>>` = **线程安全共享代理 + 互斥可变代理**的组合。

---

## 📋 并发速查表

| 需求 | 工具 |
|------|------|
| 创建线程 | `thread::spawn` |
| 等待线程 | `handle.join()` |
| 跨线程传数据 | `move` 闭包 |
| 线程间通信 | `mpsc::channel` |
| 多线程共享只读 | `Arc<T>` |
| 多线程共享可变 | `Arc<Mutex<T>>` |
| 自动加锁/解锁 | MutexGuard 的 RAII |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：MutexGuard 的自动解锁能解决死锁吗？

**A：不能。** RAII 解决「锁的释放」（资源管理），死锁是「锁的获取」（算法设计），两者是不同层面。

#### RAII 能解决 vs 不能解决
| 问题 | RAII 解决？ | 说明 |
|------|-----------|------|
| 忘记 unlock | ✅ | guard drop 自动解锁 |
| panic 时没解锁 | ✅ | unwind 也会 drop |
| 提前 return 没解锁 | ✅ | 离开作用域 drop |
| **死锁** | **❌** | **根本拿不到锁，谈不上解锁** |

#### 为什么解决不了死锁？
```
线程1: lock(A) ✅ → 等待 lock(B)...  ← 卡死
                       （永远执行不到 guard drop）

线程2: lock(B) ✅ → 等待 lock(A)...  ← 卡死
                       （永远执行不到 guard drop）
```
RAII 的前提是「guard 能离开作用域」。死锁时线程卡在 `lock()` 上，根本进不了作用域，RAII 无从发挥。

#### 本质区别
| | RAII | 死锁 |
|---|------|------|
| 解决什么 | 「拿到了锁，怎么还回去」 | 「根本拿不到锁」 |
| 发生时机 | 离开作用域时 | `lock()` 调用时阻塞 |
| Rust 自动解决？ | ✅ | ❌ |
| 归根结底 | 资源管理问题 | **算法/设计问题** |

#### 怎么解决死锁？

**方法 1：固定加锁顺序（最常用 ⭐）**
```rust
// ✅ 所有线程按相同顺序加锁
// 线程1: lock(A) → lock(B)
// 线程2: lock(A) → lock(B)
```

**方法 2：用 channel 代替多锁**
```rust
let (tx, rx) = mpsc::channel();   // 消息传递，不用多锁
```

**方法 3：尽量只用一把锁**
```rust
struct State { counter: i32, cache: HashMap<...> }
let state = Arc::new(Mutex::new(State { ... }));   // 只一把锁
```

**方法 4：try_lock（非阻塞尝试）**
```rust
match mutex.try_lock() {
    Ok(guard) => { /* 拿到 */ }
    Err(_) => { /* 没拿到，做别的 */ }
}
```

#### 防护措施对比
| 措施 | 防止什么 | 代价 |
|------|---------|------|
| RAII | 忘记解锁 | 无 |
| 固定加锁顺序 | 死锁 | 程序员纪律 |
| channel | 共享状态并发 | 拷贝开销 |
| 只用一把锁 | 死锁 | 锁粒度粗 |
| try_lock | 死锁 | 需要重试逻辑 |

#### 核心洞察
> RAII 解决「锁的释放」（资源管理），死锁是「锁的获取」（算法设计）。Rust 帮你管内存安全（含 RAII），但并发算法的正确性要你自己负责。

#### 为什么 Rust 不自动检测死锁？
1. 运行时检测有性能开销
2. 死锁检测算法复杂
3. 大部分死锁可以通过「固定加锁顺序」避免
4. Rust 的哲学是「零成本抽象」

所以 Rust 选择：**编译期防数据竞争，运行时不管死锁**。

---

## ✅ 第 16 章 小结

学完本章你应该掌握：
1. ✅ 创建线程、等待线程、跨线程传数据（move）
2. ✅ 用 channel（mpsc）做线程间通信
3. ✅ 用 `Arc<Mutex<T>>` 实现多线程共享可变数据
4. ✅ 理解 Send 和 Sync trait 的作用
5. ✅ 区分单线程和多线程的智能指针选择

---

## 📂 本章练习目录

- `concurrency/src/threads.rs` —— 16.1 线程
- `concurrency/src/channels.rs` —— 16.2 消息传递
- `concurrency/src/shared_state.rs` —— 16.3 共享状态
- `concurrency/src/send_sync.rs` —— 16.4 Send/Sync
