# 第 4 章：所有权 (Ownership) ⭐ 全书最重要

> 对应 the book 第 4 章
> 学习目标：理解所有权规则、移动、借用、切片——Rust 内存管理的核心机制

## 📂 示例代码
```bash
cd ch04-ownership/ownership
cargo run --bin ownership_rules   # 4.1 所有权规则与移动
cargo run --bin borrowing         # 4.2 引用与借用
cargo run --bin slices            # 4.3 切片
```

---

## 🔑 前置概念：栈 (Stack) vs 堆 (Heap)

| | 栈 Stack | 堆 Heap |
|---|---------|---------|
| 存什么 | 编译期已知大小的数据 | 运行时才知道大小的数据 |
| 速度 | 快（直接操作栈顶） | 慢（要找空闲块） |
| 管理 | 自动 push/pop | 手动/自动分配释放 |
| 例子 | 整数、浮点、布尔、char | String、Vec、Box |

**String 在内存中的样子**（理解所有权的关键）：
```
栈上（3 个字段）：            堆上：
┌──────────────┐            ┌───┬───┬───┬───┬───┐
│ ptr ───────────────→      │ h │ e │ l │ l │ o │
│ len = 5      │            └───┴───┴───┴───┴───┘
│ capacity = 5 │
└──────────────┘
```
- 栈上存：指针 ptr（指向堆）、长度 len、容量 capacity
- 堆上存：实际的字符数据

---

## 4.1 所有权规则与移动

### 🔑 所有权三大规则（必须背下来！）

1. 每个值都有一个**所有者** (owner) 变量
2. 同一时刻，值只能有**一个**所有者
3. 当所有者离开作用域，值被自动 **drop**（释放）

### 一、拷贝 (Copy) vs 移动 (Move)

#### 栈数据：Copy（拷贝）
```rust
let x = 5;
let y = x;          // x 的值被拷贝给 y，两者独立
println!("{x}, {y}");  // 两个都能用
```
**Copy 的类型**：整数、浮点、布尔、char、`&T` 不可变引用、(全 Copy 元素的)元组

#### 堆数据：Move（移动）⭐ 核心
```rust
let s1 = String::from("hello");
let s2 = s1;        // ⚠️ 不是拷贝！s1 的所有权移动给 s2
// println!("{s1}");  // ❌ 编译错误！s1 已失效
println!("{s2}");   // ✅ 只有 s2 能用
```

**移动过程图示**：
```
赋值 let s2 = s1; 后：
  栈：s1 → [已失效]              ← 不再拥有数据
       s2 → [ptr,len,cap] → 堆:[hello]  ← 接管了堆数据
```

**为什么不拷贝？**
1. 堆数据拷贝代价大
2. 如果拷贝，s1 和 s2 会指向同一块堆内存，两者离开作用域都会 drop → **double free（双重释放）崩溃**
3. 移动保证「同一时刻只有一个所有者」，从根本上避免 double free

### 二、函数调用也会移动（易踩坑！）
```rust
let s = String::from("hello");
takes_ownership(s);    // ⚠️ s 移动进函数
// println!("{s}");    // ❌ s 已失效

let num = 5;
makes_copy(num);       // num 是 Copy
println!("{num}");     // ✅ 仍可用

let s3 = gives_ownership();          // 返回值移动给 s3
let s5 = takes_and_gives_back(s4);   // s4 移入，又移出给 s5
```

> 💡 **痛点**：每次用 String 调函数都要移进移出，太麻烦 → 引出 4.2 节「引用」

### 三、克隆 Clone（真的需要深拷贝时）
```rust
let s1 = String::from("hello");
let s2 = s1.clone();   // 显式深拷贝，s1 仍有效
println!("{s1}, {s2}");  // 两个都能用
```
> 看到 `.clone()` 就知道「这里有堆拷贝开销」

### Copy vs Move 判定表

| 类型 | 行为 | 原因 |
|------|------|------|
| 整数/浮点/布尔/char | Copy | 栈上，大小固定 |
| `&T` 不可变引用 | Copy | 只是借用 |
| (Copy, Copy) 元组 | Copy | 所有元素都 Copy |
| String | Move | 有堆数据 |
| Vec\<T\> | Move | 有堆数据 |
| (String, i32) 元组 | Move | 含非 Copy 元素 |

---

## 4.2 引用与借用

### 一、不可变引用 `&T` —— 只读借用
```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);   // 传 &s1，不移动所有权
println!("'{s1}' 长度 {len}");      // ✅ s1 仍可用

fn calculate_length(s: &String) -> usize {
    s.len()
}   // s 不拥有数据，离开作用域什么也不发生
```

**借用 (Borrowing)**：拿引用去用数据，但不拿走所有权。就像借书——你能看，但书还是图书馆的。

### 二、可变引用 `&mut T` —— 可写借用
```rust
let mut s = String::from("hello");
change(&mut s);              // 传可变引用，函数能修改
println!("{s}");             // hello, world

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
> ⚠️ 变量本身必须是 `mut`，才能借用 `&mut`

### 三、⭐ 可变引用的两大规则（极易踩坑！）

#### 规则 1：同一时刻只能有一个可变引用
```rust
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s;   // ❌ 编译错误！两个可变引用
```
**目的**：防止**数据竞争 (data race)**

#### 规则 2：可变引用与不可变引用不能共存
```rust
let mut s = String::from("hello");
let r1 = &s;          // 不可变引用
let r2 = &s;          // 多个不可变引用 OK
// let r3 = &mut s;   // ❌ 已有不可变引用，不能再有可变
println!("{r1} {r2}");
let r3 = &mut s;      // ✅ r1 r2 已失效（NLL），现在可以
println!("{r3}");
```
**目的**：防止「有人正在读，你却把它改了」

### 四、NLL：非词法生命周期 (Non-Lexical Lifetimes)

引用的作用域不是到 `{}` 结束，而是到**最后一次使用**：
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}");   // ← r1 r2 最后一次使用，此后失效
let r3 = &mut s;         // ✅ 因为 r1 r2 已失效，不冲突
println!("{r3}");
```

### 五、悬垂引用 (Dangling Reference)
```rust
// fn dangle() -> &String {     // ❌ 编译错误
//     let s = String::from("hi");
//     &s    // s 会被 drop，返回指向它的引用 = 悬垂！
// }
```
Rust 编译器在**编译期**就阻止悬垂引用。

### ⭐ 借用规则总结
1. 任意时刻，要么有 **1 个可变引用**，要么有 **任意多个不可变引用**（二者互斥）
2. 引用必须始终有效（不能悬垂）

---

## 4.3 切片 (Slices)

切片：引用集合中**连续的一部分**，不拥有所有权。

### 一、字符串切片 `&str`
```rust
let s = String::from("hello world");
let hello = &s[0..5];    // "hello"
let world = &s[6..11];   // "world"

// 简写
let a = &s[..5];     // 从头
let b = &s[6..];     // 到尾
let c = &s[..];      // 整个
```

**切片内存结构**：
```
s → [ptr,len,cap] → 堆:[h][e][l][l][o][ ][w][o][r][l][d]
                          ↑              ↑
切片 &s[0..5]: [ptr,len=5]─┘              │
切片 &s[6..11]:[ptr,len=5]───────────────┘
```
切片只存「起始指针 + 长度」，不拥有堆数据。

### 二、字符串字面量就是切片
```rust
let s: &str = "hello world";   // 字面量类型是 &str
```
字面量存在二进制里，`&str` 是指向它的切片。

### 三、String vs &str 对比

| 类型 | 所有权 | 可变性 | 内存 |
|------|--------|--------|------|
| String | 有 | 可变 | 堆，可增长 |
| &str | 无 | 只读 | 指向某处字符串 |

### 四、函数参数用 &str 更通用 ⭐ 最佳实践
```rust
fn print_str(s: &str) { ... }

print_str(&String::from("hi"));  // ✅ &String 自动转 &str
print_str("hi");                 // ✅ 本来就是 &str
```
> 💡 函数参数优先用 `&str` 而非 `&String`，因为 `&String` 能自动转 `&str`

### 五、其他类型的切片
```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
```

### 六、用切片解决「悬挂引用」问题
```rust
let mut s = String::from("hello world");
let word = first_word(&s);   // word: &str
// s.clear();   // ❌ 编译错误！违反借用规则
println!("{word}");   // ✅ 安全
```
切片和原数据的生命周期被编译器绑定，保证不会悬垂。

---

## 📋 所有权 / 借用 / 切片 三者总结

| 概念 | 语法 | 所有权 | 用途 |
|------|------|--------|------|
| 移动 | `s` | 有 | 拥有数据 |
| 借用 | `&s` / `&mut s` | 无 | 读/写但不拥有 |
| 切片 | `&s[..]` / `&[T]` | 无 | 引用数据的一部分 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：一个可变引用后面可以多线程同时修改吗？

**A：不行。** Rust 会在**编译期**就阻止你，这是「无畏并发」的核心。

#### 三道防线

**防线 1：借用检查器**
```rust
let mut x = 0;
let r1 = &mut x;
let r2 = &mut x;   // ❌ 编译错误！同一时刻两个可变引用
```
第 4 章学的借用规则在多线程同样生效，根本创建不出两个 `&mut`。

**防线 2：`Send` trait（跨线程传递限制）**
`thread::spawn` 要求闭包是 `Send + 'static`：
- `Send` 表示类型能安全地跨线程转移所有权
- 非线程安全的类型（如 `Rc<T>`）不是 `Send`，禁止跨线程传

**防线 3：`Arc` 不允许直接修改**
```rust
let x = Arc::new(0);
*x = 1;   // ❌ cannot assign to data in an `Arc`
```
`Arc` 只提供共享只读访问，想修改必须配合 `Mutex`。

#### 正确做法：Arc + Mutex
```rust
let counter = Arc::new(Mutex::new(0));   // 共享 + 加锁
for _ in 0..5 {
    let counter = Arc::clone(&counter);
    thread::spawn(move || {
        let mut num = counter.lock().unwrap();  // 🔒 上锁
        *num += 1;                              // 一次只一个线程能到这
    });                                          // ← 离开作用域自动解锁
}
// 结果：稳定得到 5
```

#### 对比表

| 企图 | 结果 | 原因 |
|------|------|------|
| 单线程两个 `&mut` | ❌ 编译错 | 借用规则 |
| 多线程共享 `&mut` | ❌ 编译错 | 借用规则 + Send 约束 |
| 多线程 `Arc` 直接改 | ❌ 编译错 | Arc 不提供可变访问 |
| 多线程 `Arc<Mutex<T>>` | ✅ 正确 | 锁保证互斥访问 |

#### 设计哲学：无畏并发 (Fearless Concurrency)
- 其他语言：多线程 bug 在**运行时**随机出现，极难调试
- Rust：多线程 bug 在**编译时**就被挡住，根本无法编译

借用规则（第 4 章）不只是管内存安全，还顺带解决了数据竞争——这是 Rust 最天才的设计之一。第 16 章会专门讲并发。

---

### Q2：引用感觉本身就是借用操作？它们是一回事吗？

**A：直觉是对的。** 创建引用 = 借用，两者描述同一件事，只是视角不同。

| | 引用 (Reference) | 借用 (Borrowing) |
|---|-----------------|------------------|
| 词性 | 名词 | 动词 |
| 本质 | 一种类型/值（`&i32`） | 一种动作/关系 |
| 视角 | 关注机制（这个指针本身） | 关注与所有权的关系（没拿走所有权） |

```rust
let r: &String = &s;
//     └ 类型 ┘   └ 创建引用 = 借用 s
```

**关键结论**：在 Rust 里，创建引用的过程就是借用。两个词经常互换使用。

**借用这个词的真正价值**——强调与「所有权转移」的对比：
```rust
fn takes_ownership(s: String)  {}  // 拿走所有权（move）
fn borrows(s: &String)         {}  // 只是借用，不拿走
```
Rust 里让别的代码访问数据只有两种方式：
1. **move**——给它所有权，你就不能用了
2. **borrow**（给引用）——你还有所有权，让它借用看看/改改

**借用 = 给引用 = 不转移所有权**，三个说法等价。

不同语境用不同词更准确：
- 讨论类型时说「引用」：「参数是 `&String` 引用类型」
- 讨论所有权时说「借用」：「这里只是借用，不是拥有」
- 讨论约束时说「借用规则」（因为强调的是借用行为的约束）

---

### Q3：切片部分说「违反借用规则」，具体违反哪条？

**A：** 违反的是——**可变借用（`&mut`）和不可变借用（`&`）不能同时存在**。编译器错误码 E0502。

#### 问题代码
```rust
let mut s = String::from("hello world");
let word = first_word(&s);   // ① &s 不可变借用，word 指向 s 的数据
s.clear();                    // ② s.clear() 是 &mut self，可变借用
println!("{word}");           // ③ 还在用 word，说明 ① 的借用还活着
```

#### 编译器报错（E0502）
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> immutable borrow occurs here         （first_word(&s)）
 --> mutable borrow occurs here          （s.clear()）
 --> immutable borrow later used here    （println!("{word}")）
```

#### 时间线分析
```
  ① first_word(&s)   ─────word 的不可变借用────→ ③ println!("{word}")
                                              │
                              ② s.clear()    │ &mut 和 & 同时存在 → 冲突！
                              (可变借用)       │
```
第 ② 行时 word 还活着（因为第 ③ 行要用），此时 clear() 又产生可变借用，两者同时存在 → 违反规则。

#### 为什么阻止？
```
first_word(&s) → word 指向 "hello world" 的 "hello"
s.clear()      → s 内容清空，堆内存释放
println!(word) → word 指向已释放内存 → 悬垂引用（use-after-free）！
```

#### 关键洞察：切片本质就是借用
切片 `&str` / `&[T]` 是一种不可变引用，受借用规则约束，和 `&s` 完全一样：
```rust
let word: &str = &s[0..5];   // word 是不可变借用 s 的一部分
// word 活着时，s 不能被可变借用
```

#### 修改顺序就能编译（NLL 的作用）
```rust
let word = first_word(&s);
println!("{word}");          // ← word 最后一次使用，借用到此结束
s.clear();                   // ✅ 现在 s 可以可变借用了
```

---

### Q4：切片是只读的，那切片的内容怎么修改？

**A：** 切片是「视图」不是「数据本身」。要修改，用**可变切片** `&mut [T]`，或修改**原数据本身**。

#### 切片的本质
切片只是「指针 + 长度」，不拥有数据，是指向别人数据的窗口：
- `&[T]`（不可变切片）= 只读窗口
- `&mut [T]`（可变切片）= **可读写窗口** ⭐ 修改用这个

#### 方式 1：可变切片 `&mut [T]`（最常用）
```rust
let mut arr = [1, 2, 3, 4, 5];
let slice: &mut [i32] = &mut arr[..];   // 可变切片
slice[0] = 100;                          // ✅ 改的就是底层数据 arr
println!("{arr:?}");                      // [100, 2, 3, 4, 5]
```
> 切片和 arr 共享同一块内存，改切片 = 改 arr。

#### 方式 2：修改原 String（切片只是临时读视图）
```rust
let mut s = String::from("hello world");
{
    let word = first_word(&s);       // 临时只读切片
    println!("{word}");
}                                    // ← 借用结束（NLL）
s.push_str("!");                     // ✅ 现在可以改 s 了
```
「先用切片读，用完还掉，再改原数据」。

#### 方式 3：改字符串字节（要小心 UTF-8）
```rust
let mut s = String::from("hello");
let bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
bytes[0] = b'H';   // Hello
```
> ⚠️ unsafe 是因为改字节可能破坏 UTF-8 编码。更安全的方式：用 `chars()` 收集成 `Vec<char>` 改完再组装。

#### 四种切片类型对比

| 类型 | 可读 | 可写 | 用途 |
|------|------|------|------|
| `&[T]` / `&str` | ✅ | ❌ | 只读视图 |
| `&mut [T]` | ✅ | ✅ | **可读写视图**（修改用这个） |
| `&mut str` | ✅ | ⚠️ 受限 | 很少用（UTF-8 限制） |

#### 关键原则
> 切片是数据的「视图」不是「副本」。修改切片本质上是修改它指向的**底层数据**。

修改三步骤：
1. 底层数据声明为 `mut`
2. 用 `&mut [T]` 取可变切片
3. 通过可变切片修改（改的就是底层数据）

借用规则保证：同一时刻要么一个 `&mut [T]`，要么多个 `&[T]`，绝不会边改边读。

---

### Q5：可变切片也只能存在一个？

**A：是的。** 可变切片就是可变借用，受借用规则约束，同一时刻只能一个。但 Rust 提供安全方法可以同时拿到**不重叠的**多个可变切片。

#### 两个可变切片同时存在 → 编译错误
```rust
let mut arr = [1, 2, 3, 4, 5];
let r1: &mut [i32] = &mut arr[..];
let r2: &mut [i32] = &mut arr[..];   // ❌ E0499
```
原因：两个切片指向**重叠**内存，同时修改会数据竞争。

#### 特例：split_at_mut 拆分不重叠的可变切片
```rust
let mut arr = [1, 2, 3, 4, 5];
let (left, right) = arr.split_at_mut(2);
//   left  = &mut [1, 2]      ← 前段
//   right = &mut [3, 4, 5]   ← 后段
left[0] = 100;      // ✅
right[0] = 999;     // ✅ 两个可变切片同时存在！
```

#### chunks_mut 按块迭代
```rust
let mut arr = [1, 2, 3, 4, 5, 6];
for chunk in arr.chunks_mut(2) {
    chunk[0] = 0;   // 每块第一元素清零
}
// 结果：[0, 2, 0, 4, 0, 6]
```

#### 为什么 split_at_mut 能编译？
它内部用了 `unsafe`，把一个切片拆成两个**不重叠**的内存区域：
```rust
pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    let ptr = self.as_mut_ptr();
    unsafe { /* 保证两段指针绝不重叠 */ }
}
```
这是 Rust 重要模式：**用 unsafe 实现底层，对外暴露安全 API**。

#### 借用规则的本质
> 借用规则防止的是**内存重叠**。两个可变引用指向同一块内存 → 危险 → 禁止。但若证明指向不同内存（如 split_at_mut），就允许。

| 情况 | 能否编译 | 原因 |
|------|---------|------|
| 两个 `&mut arr[..]`（重叠） | ❌ E0499 | 两个可变借用 |
| `arr.split_at_mut(2)` | ✅ | 标准库保证不重叠 |
| `arr.chunks_mut(2)` | ✅ | 标准库保证不重叠 |

---

## ✅ 第 4 章 小结

学完本章你应该掌握：
1. ✅ **所有权三大规则**（一个所有者，离开作用域自动 drop）
2. ✅ 区分 **Copy（栈数据）vs Move（堆数据）**
3. ✅ 理解函数调用会移动所有权
4. ✅ 用 `&T` / `&mut T` **借用**数据，不拿走所有权
5. ✅ 记住**借用规则**（可变/不可变引用互斥）
6. ✅ 用切片 `&str` / `&[T]` 引用数据的一部分

---

## 📂 本章练习目录

- `ownership/src/ownership_rules.rs` —— 4.1 所有权规则与移动
- `ownership/src/borrowing.rs` —— 4.2 引用与借用
- `ownership/src/slices.rs` —— 4.3 切片
