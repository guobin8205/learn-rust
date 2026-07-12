# 第 13 章：闭包与迭代器 (Functional Features)

> 对应 the book 第 13 章
> 学习目标：掌握闭包（捕获环境的匿名函数）和迭代器（零成本抽象）

## 📂 示例代码
```bash
cd ch13-closures-iterators/functional
cargo run --bin closures     # 13.1 闭包
cargo run --bin iterators    # 13.2 迭代器
cargo run --bin refactor     # 13.3 用迭代器重构 minigrep
```

---

## 13.1 闭包 (Closures)

### 闭包 = 能捕获环境的匿名函数

```rust
// 普通函数（必须标注类型，不能访问环境）
fn add(a: i32, b: i32) -> i32 { a + b }

// 闭包（类型可推断，能捕获环境）
let add = |a, b| a + b;              // 最简形式
let add = |a: i32, b: i32| -> i32 { a + b };  // 带类型（通常省略）
let say_hi = || println!("Hi!");     // 无参数
```

### ⭐ 核心能力：捕获环境

闭包能「看到」并使用它定义时周围的变量——这是普通函数做不到的：

```rust
let x = 10;
let add_x = |y| x + y;     // 闭包「捕获」了 x
add_x(5);                   // 15（用了外部的 x=10）
```

### 捕获的三种方式：Fn / FnMut / FnOnce ⭐

| Trait | 能做什么 | 调用次数 | 例子 |
|-------|---------|---------|------|
| `Fn` | 不可变借用（只读） | 多次 | `\|\| println!("{:?}", list)` |
| `FnMut` | 可变借用（能修改） | 多次 | `\|\| { counter += 1; }` |
| `FnOnce` | 获取所有权（消耗） | 只能 1 次 | `move \|\| { consume(data); }` |

层级关系：**Fn ⊂ FnMut ⊂ FnOnce**（实现 Fn 必然也实现 FnMut 和 FnOnce）

### move 关键字：强制获取所有权

```rust
let s = String::from("hi");
let closure = move || { println!("{}", s); };   // move 拿走 s
// println!("{}", s);   // ❌ s 已被移动
```

> ⭐ **多线程必须用 move**：把数据所有权转移给新线程
> ```rust
> let data = vec![1, 2, 3];
> thread::spawn(move || { println!("{:?}", data); });
> ```

### 闭包作为函数参数

```rust
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }

apply(|n| n * 2, 5);    // 10
```

---

## 13.2 迭代器 (Iterators)

### ⭐ 迭代器是惰性的（lazy）

创建迭代器时**什么都不做**，只有被消费时才求值：

```rust
let iter = v.iter();         // 创建（还没遍历）
for val in iter { ... }      // 这里才真正执行
```

### Iterator trait（核心）

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;   // 唯一必须实现的方法
    // 其他方法都有默认实现
}
```

### 三种迭代方式

```rust
for x in v.iter() { }        // &T（借用，v 仍可用）
for x in v.iter_mut() { }    // &mut T（可变借用，能修改）
for x in v.into_iter() { }   // T（获取所有权，消耗 v）
```

### 消费适配器（consuming adaptors）—— 消耗迭代器产出值

| 方法 | 作用 |
|------|------|
| `.sum()` | 求和 |
| `.count()` | 计数 |
| `.collect()` | 收集成集合 |
| `.any(\|x\| ...)` | 是否存在满足条件的 |
| `.all(\|x\| ...)` | 是否全部满足 |
| `.max()` / `.min()` | 最大/最小 |

### ⭐ 迭代器适配器（iterator adaptors）—— 产出新迭代器

| 方法 | 作用 |
|------|------|
| `.map(\|x\| ...)` | 变换每个元素 |
| `.filter(\|x\| ...)` | 过滤 |
| `.enumerate()` | 加索引 |
| `.zip(other)` | 拉链配对 |
| `.flat_map(\|x\| x)` | 展平嵌套 |
| `.take(n)` | 取前 n 个 |
| `.skip(n)` | 跳过前 n 个 |
| `.copied()` | `&T` 转 `T`（Copy 类型解引用） |

### ⭐ 链式调用（函数式风格）

```rust
// 找出偶数并翻倍
let result: Vec<i32> = vec![1,2,3,4,5].iter()
    .filter(|x| *x % 2 == 0)      // [2, 4]
    .map(|x| x * 2)                // [4, 8]
    .collect();
```

### 自定义迭代器（实现 Iterator trait）

```rust
struct Counter { count: u32, max: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else { None }
    }
}

// 自定义迭代器也能用所有方法！
Counter::new(5).zip(...).map(...).filter(...).sum()
```

---

## 13.3 用迭代器重构 minigrep ⭐

### 对比：循环版本 vs 迭代器版本

```rust
// 循环版本（7 行）
fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) { results.push(line); }
    }
    results
}

// 迭代器版本（1 行核心逻辑）⭐
fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
```

两个版本**结果完全相同**，但迭代器版本更简洁、更函数式。

### ⭐ 零成本抽象（zero-cost abstraction）

这是本章的精髓：

> 迭代器版本和循环版本**编译后等价**，运行时性能相同！

因为：
1. 迭代器适配器（filter、map）是内联优化的
2. 没有中间集合分配（filter 返回惰性迭代器，不是 Vec）
3. 编译器能做更激进的优化（甚至可能比手写循环更快）

这就是 Rust「高级抽象 + 零运行时成本」的精髓——**你可以用优雅的函数式风格，不损失任何性能**。

---

## 📋 闭包与迭代器速查

### 闭包
| 语法 | 含义 |
|------|------|
| `\|x\| x + 1` | 闭包 |
| `move \|\| ...` | 强制获取所有权 |
| `impl Fn(i32) -> i32` | 闭包作为参数 |

### 迭代器链式常用组合
```rust
vec.iter()                    // 创建
    .filter(|x| cond)         // 过滤
    .map(|x| transform)       // 变换
    .collect()                // 收集
```

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：模拟生成器闭包的状态值保存在哪里？不是违反生命周期吗？

**A：** 闭包本质是「匿名结构体」，捕获的变量成了结构体字段。`move` 关键字把变量所有权搬进闭包，所以不会违反生命周期。

#### 闭包的本质：匿名结构体
```rust
// 你写的代码
let mut count = start;
move || { let r = count; count += 1; r }

// 编译器实际生成的
struct Closure { count: u32 }            // ← count 成了结构体字段！
impl FnMut<()> for Closure {
    fn call_mut(&mut self) -> u32 {
        let r = self.count;
        self.count += 1;
        r
    }
}
```

#### 为什么不违反生命周期？move 转移所有权
```rust
fn generate_counter(start: u32) -> Box<dyn FnMut() -> u32> {
    let mut count = start;
    Box::new(move || { ... })
    //  ↑ move 把 count 的所有权搬进闭包
    //    函数返回时，count 跟着闭包（被 Box 包裹）一起返回，不会悬垂
}
```

#### 内存布局
```
栈：counter → Box 指针 ─┐
                       ↓
堆：闭包结构体 { count: u32 }   ← count 存在这里
```

#### 对比两种情况
```
❌ 不用 move（借用）：违反生命周期
   fn bad() -> impl FnMut() -> i32 {
       let count = 0;
       || { count += 1 }   // 闭包引用 count，函数结束 count drop → 悬垂！
   }

✅ 用 move（转移所有权）：合法
   fn good() -> Box<dyn FnMut() -> u32> {
       let count = 0;
       Box::new(move || { count += 1 })
       // count 的所有权转移进闭包，跟着返回，不悬垂
   }
```

#### 验证：两个闭包有独立状态
```
c1() = 100   ← c1 的 count
c1() = 101   ← c1 自增
c2() = 200   ← c2 有独立的 count
```

#### 为什么需要 Box？
- `Box` 把闭包放在堆上，函数才能返回它
- `dyn FnMut` 因为每个闭包是不同类型，编译期大小未知，用 trait 对象统一

#### 核心认知
| 疑问 | 答案 |
|------|------|
| count 存哪里？ | 闭包结构体的字段里（Box 放堆上） |
| 违反生命周期吗？ | 不，move 转移了所有权 |
| 函数返回后还在吗？ | 在，跟着闭包一起返回 |

#### 串联第 4 章
`move` 就是「强制把闭包用到的所有变量所有权转移进闭包」，和函数参数的 move 是同一机制。

---

## ✅ 第 13 章 小结

学完本章你应该掌握：
1. ✅ 写闭包，理解捕获环境的机制
2. ✅ 区分 Fn / FnMut / FnOnce，会用 move
3. ✅ 用迭代器方法链处理数据（map/filter/collect）
4. ✅ 实现自定义迭代器
5. ✅ 理解「零成本抽象」——迭代器和循环一样快

---

## 📂 本章练习目录

- `functional/src/closures.rs` —— 13.1 闭包
- `functional/src/iterators.rs` —— 13.2 迭代器
- `functional/src/refactor.rs` —— 13.3 用迭代器重构 minigrep
