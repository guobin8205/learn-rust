# 第 15 章：智能指针 (Smart Pointers)

> 对应 the book 第 15 章
> 学习目标：掌握 Box、Rc/Arc、RefCell，理解它们如何突破所有权限制

## 📂 示例代码
```bash
cd ch15-smart-pointers/smart_pointers
cargo run --bin box_t       # 15.1 Box<T>
cargo run --bin deref_t     # 15.2 Deref trait
cargo run --bin rc_t        # 15.3 Rc<T> / Arc<T>
cargo run --bin refcell_t   # 15.4 RefCell<T>
```

---

## 智能指针 vs 普通引用

| | 普通引用 `&T` | 智能指针（Box/Rc/RefCell） |
|---|-------------|------------------------|
| 所有权 | 无（借用） | 通常「拥有」数据 |
| 数据位置 | 引用别处 | 多在堆上 |
| drop 时 | 无事 | 释放数据 |
| 额外能力 | 无 | 自动释放、引用计数等 |

---

## 15.1 Box<T> —— 堆分配

### 基本用法

```rust
let b = Box::new(5);    // 5 放到堆上，b 是指向堆的指针
println!("{}", b);       // 自动解引用，用起来和普通变量一样
// b 离开作用域时自动释放堆内存
```

### ⭐ 核心用途

**用途 1：递归类型（最重要）**

```rust
// 递归类型必须用 Box，否则大小无限
enum List {
    Cons(i32, Box<List>),   // Box<List> 是固定大小（指针）
    Nil,
}
```

**为什么必须用 Box？**
```
不用 Box：
  enum List { Cons(i32, List), Nil }
  List 的大小 = i32 + List = i32 + (i32 + List) = ... 无限大！

用 Box 后：
  enum List { Cons(i32, Box<List>), Nil }
  List 的大小 = i32 + Box<List> = i32 + 指针（固定）✅
```

**用途 2：trait 对象**（第 17 章详讲）
```rust
let shapes: Vec<Box<dyn Debug>> = vec![Box::new(1), Box::new("hi")];
```

### ⚠️ 注意：Box::new 的栈溢出陷阱

```rust
// ❌ 栈溢出！先在栈上构造再移到堆
let big = Box::new([0u64; 1_000_000]);   // 8MB > 栈大小

// ✅ 大数据用 vec!（直接堆上构造）
let big = vec![0u64; 1_000_000];
```

---

## 15.2 Deref trait —— 自动解引用

### 实现 Deref 让自定义类型支持 `*` 和自动转换

```rust
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

### ⭐ Deref 强制转换（deref coercion）

```rust
let m = MyBox::new(String::from("Hello"));
hello(&m);   // ✅ &MyBox<String> 自动转 &str！

fn hello(name: &str) { ... }
```

**转换链**：
```
&MyBox<String>  ──deref──→  &String  ──deref──→  &str
```

Rust 看到类型不匹配时，会自动链式调用 `deref()` 转换。

### 标准库的应用

| 类型 | Deref Target | 转换 |
|------|-------------|------|
| `String` | `str` | `&String → &str` |
| `Vec<T>` | `[T]` | `&Vec → &[T]` |
| `Box<T>` | `T` | `&Box<T> → &T` |

这就是为什么 `&String` 能传给接收 `&str` 的函数！

---

## 15.3 Rc<T> —— 引用计数（多个所有者）⭐

### 解决的问题

所有权规则要求「只有一个所有者」，但有时需要多个地方共享同一份数据。Rc 允许**多个所有者**。

### 基本用法

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));     // count = 1
let b = Rc::clone(&a);                       // count = 2（不拷贝数据！）
let c = Rc::clone(&a);                       // count = 3
println!("count = {}", Rc::strong_count(&a)); // 3
// c drop → count=2, b drop → count=1, a drop → count=0 → 数据释放
```

**⭐ `Rc::clone` 不是拷贝！** 只是增加引用计数，所有 Rc 指向同一份数据。

### 内存示意
```
堆：┌──────────────┐
    │ "hello"       │ ← 真实数据
    │ count = 3     │ ← 引用计数
    └──────────────┘
       ↑     ↑     ↑
栈：   a     b     c    ← 三个指针
```

### 经典场景：共享节点（图/树）

```rust
enum List {
    Cons(i32, Rc<List>),   // 多个节点共享同一个子节点
    Nil,
}
```

### Rc 的限制：不可变 + 非线程安全

```rust
let data = Rc::new(5);
// *data = 10;   // ❌ 不能修改 Rc 内部值

// let data = Rc::new(5);
// thread::spawn(move || data);   // ❌ Rc 不能跨线程
```

### Rc vs Arc

| | Rc<T> | Arc<T> |
|---|-------|--------|
| 引用计数 | 普通整数 | 原子操作 |
| 线程安全 | ❌ 单线程 | ✅ 多线程 |
| 性能 | 略快 | 略慢 |

```rust
// 多线程共享用 Arc
use std::sync::Arc;
let data = Arc::new(100);
let data_clone = Arc::clone(&data);
thread::spawn(move || println!("{}", data_clone));
```

---

## 15.4 RefCell<T> —— 内部可变性 ⭐

### 解决的问题

有些场景下你有一个「不可变」的值，但内部需要修改。RefCell 把**借用检查从编译期移到运行时**。

### 基本用法

```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() = 10;    // 运行时检查借用
println!("{}", cell.borrow());
```

### ⚠️ 运行时 panic

违反借用规则会**运行时 panic**（不是编译错误）：
```rust
let cell = RefCell::new(5);
let b1 = cell.borrow();       // 不可变借用
let b2 = cell.borrow_mut();   // 💥 运行时 panic: already borrowed
```

### 经典场景：MockMessenger

trait 定义是 `&self`（不可变），但实现需要修改内部状态：

```rust
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,   // 用 RefCell 包裹
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // ⭐ 在「不可变 &self」里修改数据
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}
```

### 编译期 vs 运行时借用检查

| | 编译期（默认） | 运行时（RefCell） |
|---|-------------|----------------|
| 检查时机 | 编译时 | 运行时 |
| 错误 | 编译错误 | panic |
| 性能 | 零成本 | 有少量开销 |
| 灵活性 | 受限 | 更灵活 |

---

## ⭐ 终极组合：Rc<RefCell<T>>

**共享 + 可变 = Rc<RefCell<T>>**（Rust 单线程最强的组合）

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(0));
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);

*a.borrow_mut() = 10;    // 通过 a 修改
*b.borrow_mut() += 5;    // 通过 b 修改（同一份数据）
println!("{}", shared.borrow());   // 15
```

---

## 📋 四大智能指针总结 ⭐

| 指针 | 所有权 | 借用检查 | 可变性 | 何时用 |
|------|--------|---------|--------|--------|
| `Box<T>` | 唯一 | 编译期 | 自由 | 堆分配、递归类型 |
| `Rc<T>` | 共享 | 编译期 | 只读 | 单线程共享只读 |
| `RefCell<T>` | 唯一 | **运行时** | 内部可变 | 不可变接口需要修改内部 |
| `Rc<RefCell<T>>` | 共享 | 运行时 | **共享可变** | 单线程共享可变 |
| `Arc<T>` | 共享（线程安全） | 编译期 | 只读 | 多线程共享只读 |
| `Arc<Mutex<T>>` | 共享（线程安全） | 运行时 | 共享可变 | 多线程共享可变 |

### 两大维度

```
          编译期检查          运行时检查
唯一所有者  Box<T>            RefCell<T>
多个所有者  Rc<T>/Arc<T>      Rc<RefCell<T>>/Arc<Mutex<T>>
```

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：List 用普通指针（引用）不是也可以固定大小吗？为什么必须用 Box？

**A：** 你的直觉对——`Box` 和 `&` 都是指针，大小都固定。问题不在「大小」，而在**所有权**。

#### 核心区别：谁拥有数据？
```
Box<List>   → 拥有数据（自己负责释放）
&List       → 不拥有数据（借用别人的）
```

#### 引用版本的三个问题

**问题 1：被引用的对象在哪？**
```rust
let node3 = Node::Nil;              // 必须先有 node3
let node2 = Node::Cons(3, &node3);  // node2 借用 node3
let node1 = Node::Cons(2, &node2);  // node1 借用 node2
// N 层链表需要 N 个独立 let 变量！
```

**问题 2：不能动态/递归构造**
```rust
// ❌ 引用版：递归构造时局部变量会 drop → 悬垂
fn build_list(n: i32) -> ??? {
    let inner = Node::Nil;
    Node::Cons(n, &inner)   // inner 函数结束 drop，引用悬垂！
}

// ✅ Box 版：能递归构造并返回
fn build_list(n: i32) -> Box<BoxList> {
    Box::new(BoxList::Cons(n, build_list(n - 1)))
}
```

**问题 3：生命周期标注让代码复杂**
```rust
// 引用版：每个函数都要标生命周期
fn push<'a>(list: &'a List<'a>, val: i32) -> List<'a> { ... }
// Box 版：不用标
fn push(list: Box<List>, val: i32) -> Box<List> { ... }
```

#### 完整对比
| 特性 | `&List`（引用） | `Box<List>` |
|------|---------------|-------------|
| 指针大小 | 固定 | 固定 |
| **所有权** | ❌ 不拥有 | ✅ 拥有 |
| 生命周期 | 必须标注 | 不需要 |
| 能递归构造 | ❌ | ✅ |
| 能动态构造 | ❌ | ✅ |
| 能从函数返回 | ❌ 困难 | ✅ |
| 数据何时释放 | 被引用者负责 | Box drop 时 |

#### 关键洞察
> **「指针大小」不是问题，「所有权归属」才是。**

Box 和 `&` 大小都是 8 字节。但 Box「拥有」数据 → 自管理生命周期 → 能自由构造/返回。`&`「借用」数据 → 依赖外部 → 不能自由构造。

**Box 的本质 = 指针 + 所有权 + 自动释放**（所以叫「智能指针」）。

#### 现实类比
- **引用** = 借书（书是图书馆的，到期要还）
- **Box** = 买书（书是你的，扔掉时书也没了）

---

### Q2：Box::new 为什么数据要在栈上构造？不能像 vec! 一样直接堆上构造？

**A：** 不是 Box「不能」，而是 `Box::new` 是**普通函数**，受「函数参数先在调用者栈帧求值」规则限制。`vec!` 是**宏**，能绕过。

#### 根本原因：函数 vs 宏
```rust
// Box::new 是函数：参数先在调用者栈帧求值
Box::new([0u64; 1_000_000])
//     ↑ 先在栈上构造 8MB 数组 → 再传给 Box → 移动到堆 → 栈溢出！

// vec! 是宏：展开后直接堆分配
vec![0u64; 1_000_000]
//  展开成：Vec::with_capacity(1M) 先堆分配 → 再逐个写入堆 → 从不经过栈
```

#### 执行流程对比
```
Box::new(大数组)：
  1. 栈上构造 8MB → 2. 传参 → 3. 移动到堆
                    ↑ 步骤 1 栈溢出

vec![大数组]：
  1. Vec::with_capacity 堆分配 → 2. 直接写入堆
                                 ↑ 从不经过栈
```

#### 三种构造方式对比
| 方式 | 栈使用 | 大数组？ |
|------|--------|---------|
| `Box::new([u64; 1M])` | 8MB（整个数组） | ❌ 溢出 |
| `vec![u64; 1M]` | 24 字节（Vec 元数据） | ✅ |
| `vec![...].into_boxed_slice()` | 24 字节 | ✅ |

#### 实用解决方案
```rust
// ✅ 方法 1：vec! + into_boxed_slice（推荐）
let big: Box<[u64]> = vec![0u64; 1_000_000].into_boxed_slice();

// ✅ 方法 2：Box<Vec<T>>
let big: Box<Vec<u64>> = Box::new(vec![0u64; 1_000_000]);
```

#### 类比理解
- `Box::new(大箱子)` = 先搬到走廊（栈）再搬到仓库（堆），走廊窄卡住
- `vec![大箱子]` = 直接在仓库里组装，不经过走廊

#### 为什么 Box::new 不设计成宏？
Box::new 接受**任意类型 T**，宏需要在编译期生成专门代码。`vec![值; n]` 语义具体（知道要 n 个元素），能用宏优化。Box::new 太通用，只能按函数规则走。

---

### Q3：只有一个所有者和允许多个所有者不冲突吗？

**A：** 不冲突。Rc 没有「违反」所有权规则，而是**扩展**了所有权的实现方式——用引用计数替代单一所有者。数据始终只有**一份**，只是「谁来释放」的逻辑变了。

#### 所有权规则的本质目的
> 保证「数据有明确的生命周期，避免 double free」。

只要数据**只被释放一次**，就不违反规则。Rc 正好保证这一点。

#### 普通 move vs Rc 的区别
```rust
// 普通 move：数据 1 份，所有者 1 个
let s2 = s1;   // s1 失效

// Rc：数据还是 1 份！只是多个指针指向它
let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);   // 不拷贝数据，只 count+1
```

#### 实验验证：三个 Rc 指向同一份数据
```
x 的堆地址: 0x21d103a8930
y 的堆地址: 0x21d103a8930
z 的堆地址: 0x21d103a8930
三个地址相同吗？true
```

#### Rc 的内部结构
```
栈：              堆：
a: ptr ──┐       ┌──────────────────────┐
b: ptr ──┼──→    │ RcBox {              │
c: ptr ──┘       │   value: "hello",    │  ← 数据只有一份
                 │   strong_count: 3,   │  ← 引用计数
                 │ }                    │
                 └──────────────────────┘
```
a/b/c 是「指针」，不是「所有者」。真正的管理者是 RcBox。

#### 为什么不 double free？
```
❌ 真·多独立所有者（假设）：
   a drop → 释放 "hello"
   b drop → 再释放 "hello" → double free！

✅ Rc（共享所有权）：
   a drop → count 2（不释放）
   b drop → count 1（不释放）
   c drop → count 0（才释放）→ 只释放一次！
```

#### 三种模式对比
| 模式 | 数据份数 | 释放时机 | double free？ |
|------|---------|---------|--------------|
| 普通 move | 1 | 所有者离开 | 不会 |
| 真·多所有者（假设） | 多份拷贝 | 各自离开 | ⚠️ 可能 |
| **Rc** | **1 份共享** | **最后一个引用离开** | **不会** |

#### 更准确的说法：共享所有权
不是「多个所有者」，而是「**共享所有权**」——多个 Rc 共有一份数据，通过引用计数协调释放。

#### 类比
- 普通所有权 = 一个人独租（搬走就退房）
- Rc = 三人合租一间（最后一人搬走才退房）

房间始终只有**一间**。

#### Rc 是引用计数式 GC
Rc 其实是垃圾回收的一种形式（引用计数 GC）。和 Java/Go 的追踪式 GC 不同，Rc 是**确定性释放**（count=0 立即释放，无暂停）。

---

### Q4：那 Rc 是不是相当于一个「所有权代理」？

**A：完全正确！** 「所有权代理」这个比喻精确抓住了 Rc 的本质。

#### Rc 的两层代理结构
```
你的代码（a, b, c）
     │
     ↓ 访问/修改请求
┌─────────────────┐
│ Rc（代理层）     │  ← 管理引用计数
│  - clone: +1    │
│  - drop: -1     │
│  - count=0: drop│
└────────┬────────┘
         ↓
┌─────────────────┐
│ 实际数据         │  ← "hello"
└─────────────────┘
```

#### 为什么「代理」特别贴切？

**① 代理隔离了用户和数据**
- 用户不直接操作数据，而是通过代理
- `a/b/c` 持有代理的引用，不是直接拥有数据

**② 代理管理数据的生命周期**
- 何时分配、何时释放，由代理决定
- clone → count+1，drop → count-1，count=0 → 释放

**③ 代理控制访问权限**
- Rc 默认是「只读代理」（不能修改 `*x`）
- 要修改需要组合 RefCell（可变性代理）

**④ 代理可以组合**
- `Rc<RefCell<T>>` = 共享代理 + 可变代理（双重代理）
- `Arc<Mutex<T>>` = 线程安全共享代理 + 互斥可变代理

#### 所有智能指针都是不同类型的「代理」
| 智能指针 | 代理类型 | 职责 |
|---------|---------|------|
| `Box<T>` | 独占代理 | 一个客户，转移所有权 |
| `Rc<T>` | 共享只读代理 | 多客户共享，引用计数 |
| `Arc<T>` | 线程安全共享代理 | 原子引用计数 |
| `RefCell<T>` | 可变性代理 | 运行时借用检查 |
| `Weak<T>` | 弱引用代理 | 不计数，避免循环引用 |

#### 现实类比
- 你（a/b/c） → 物业公司（Rc） → 产权系统 → 房子（数据）
- 你不直接拥有房子，你拥有「产权份额」
- 物业公司管理「什么时候房子归零回收」

#### 核心洞察
> **智能指针 = 指针 + 所有权管理策略（代理）**

普通引用 `&T` 只是「指向数据的指针」，没有管理职责。
智能指针是「带管理策略的指针」——代理了所有权的各个方面（分配、释放、共享、可变性）。

---

## ✅ 第 15 章 小结

学完本章你应该掌握：
1. ✅ 用 Box 把数据放堆上，实现递归类型
2. ✅ 实现 Deref trait，理解自动解引用转换
3. ✅ 用 Rc 实现多个所有者共享数据
4. ✅ 用 RefCell 实现内部可变性
5. ✅ 用 Rc<RefCell<T>> 实现共享可变
6. ✅ 区分编译期检查和运行时检查

---

## 📂 本章练习目录

- `smart_pointers/src/box_t.rs` —— 15.1 Box<T>
- `smart_pointers/src/deref_t.rs` —— 15.2 Deref
- `smart_pointers/src/rc_t.rs` —— 15.3 Rc/Arc
- `smart_pointers/src/refcell_t.rs` —— 15.4 RefCell
