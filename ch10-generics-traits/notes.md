# 第 10 章：泛型、Trait 和生命周期 ⭐ 全书最长最难

> 对应 the book 第 10 章
> 学习目标：掌握泛型（类型参数化）、Trait（定义共享行为）、生命周期（引用有效性）

## 📂 示例代码
```bash
cd ch10-generics-traits/generics
cargo run --bin generics_t   # 10.1 泛型
cargo run --bin trait_t      # 10.2 Trait
cargo run --bin lifetime_t   # 10.3 生命周期
```

---

## 10.1 泛型 (Generics)

### 提取泛型函数

```rust
// 泛型函数：<T> 表示类型参数，T: PartialOrd 是 trait 约束
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
// 一个函数搞定所有「能比较大小」的类型！
```

### 泛型结构体

```rust
struct Point<T> { x: T, y: T }           // x、y 同类型
struct Point2<T, U> { x: T, y: U }       // x、y 可不同类型
```

### 泛型方法

```rust
impl<T> Point<T> {                        // 为所有 T 的 Point 实现
    fn x(&self) -> &T { &self.x }
}

impl Point<f64> {                         // 只为 f64 的 Point 实现（特定类型）
    fn distance_from_origin(&self) -> f64 { ... }
}
```

### ⭐ 零成本抽象（单态化）

泛型在编译时**单态化 (monomorphization)**：为每个具体类型生成专用代码
```
largest::<i32>  → 生成 largest_i32 版本
largest::<char> → 生成 largest_char 版本
```
**运行时和手写特定类型一样快，没有性能损失！**

### 你已经在用泛型

`Option<T>`、`Result<T,E>`、`Vec<T>`、`HashMap<K,V>` 都是泛型枚举/结构体。

---

## 10.2 Trait：定义共享行为

### 定义与实现

```rust
// 定义 trait（类似接口）
trait Summary {
    fn summarize(&self) -> String;                    // 必须实现
    fn preview(&self) -> String {                     // 默认实现（可不重写）
        format!("预览: {}...", &self.summarize()[..10])
    }
}

// 为不同类型实现同一个 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String { format!("{}", self.title) }
}
impl Summary for Tweet {
    fn summarize(&self) -> String { format!("@{}", self.username) }
}
```

### Trait 作为参数（三种写法）

```rust
// 写法 1：impl Trait（最简单，语法糖）
fn notify(item: &impl Summary) { ... }

// 写法 2：Trait bound（完整形式）
fn notify<T: Summary>(item: &T) { ... }

// 要求两个参数同类型（只能用 bound 形式）
fn notify_same<T: Summary>(a: &T, b: &T) { ... }
```

### 多约束：`+`

```rust
fn notify(a: &(impl Summary + Display)) { ... }    // impl Trait 形式
fn notify<T: Summary + Display>(a: &T) { ... }     // bound 形式
```

### where 子句（约束多时更清晰）

```rust
// 约束多时函数签名很长，用 where 更易读
fn some_fn<T, U>(a: &T, b: &U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{ ... }
```

### 返回实现了 Trait 的类型

```rust
fn create_summary() -> impl Summary {
    Tweet { ... }
    // ⚠️ 只能返回一种具体类型，不能根据条件返回不同类型
}
```

### Trait Object（dyn Trait）预览（第 17 章详讲）

```rust
// 把不同类型都当作「实现了 Summary 的东西」放进同一个集合
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Tweet { ... }),
    Box::new(NewsArticle { ... }),
];
```

### Trait vs 其他语言的接口

| 语言 | 概念 | Rust 的区别 |
|------|------|-----------|
| Java | interface | Rust trait 可有默认实现、可组合 |
| Go | interface | Rust 要显式 `impl Trait for Type` |
| TypeScript | interface | Rust 不是鸭子类型 |
| Rust | **trait** | 可默认实现、可组合、可做约束、trait object |

---

## 10.3 生命周期 (Lifetimes) ⭐ 最难

### 解决什么问题？

引用必须始终有效（不能悬垂）。编译器需要知道每个引用「活多久」。大部分时候能自动推断，有时需要手动标注。

### 生命周期标注语法：`'a`

```rust
// 'a 表示：x、y、返回值三个引用至少活一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**理解 `'a`**：它是一个「约束」，不是「修改」。表示「返回值至少和 x、y 中较短的那个活得一样久」。

### 生命周期标注的实际效果

```rust
let result;
{
    let s2 = String::from("world!");
    result = longest(s1.as_str(), s2.as_str());
    println!("{}", result);   // ✅ s2 还活着
}
// println!("{}", result);   // ❌ s2 已 drop，result 可能指向 s2，不能用
```

### 结构体中的生命周期

```rust
// 结构体持有引用时，必须标注生命周期
struct Excerpt<'a> {
    part: &'a str,    // part 引用的数据必须比 excerpt 活得久
}
```

### 静态生命周期 `'static`

```rust
let s: &'static str = "字面量";   // 整个程序运行期间都有效
```
字符串字面量存在二进制文件里，永远有效。

### 生命周期消除规则（编译器自动做的）

1. 每个引用参数获得独立的生命周期
2. 只有一个输入生命周期时，赋给所有输出
3. 方法（有 `&self`）时，self 的生命周期赋给输出

```rust
fn foo(x: &str) -> &str       // 自动变成 fn foo<'a>(x: &'a str) -> &'a str
```

### 生命周期使用场景

| 场景 | 要标注吗 |
|------|---------|
| 函数参数和返回都是引用 | 通常要 |
| 结构体字段是引用 | 必须要 |
| 只有参数引用无返回 | 通常不用 |
| 方法里用 `&self` | 通常不用（规则 3） |

> 💡 **经验**：先不写生命周期标注，让编译器告诉你哪里需要！

### 三合一：泛型 + Trait + 生命周期（终极形态）

```rust
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{ ... }
```

---

## 📋 三大特性速查

| 特性 | 语法 | 作用 |
|------|------|------|
| 泛型 | `<T>` | 类型参数化（一份代码多种类型） |
| Trait | `trait` / `impl Trait for T` | 定义共享行为（接口） |
| 生命周期 | `'a` | 标注引用的有效期限 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：生命周期为什么一定要标注？编译器不能自己推断吗？

**A：** 根本原因是——**调用者只看函数签名，不看函数体**。如果签名不标明返回引用和哪个参数绑定，调用者就无法判断安全性。

#### 编译器的真实报错（这就是答案）
```
fn longest(x: &str, y: &str) -> &str { ... }

error[E0106]: missing lifetime specifier
help: this function's return type contains a borrowed value,
      but the signature does not say whether it is borrowed from `x` or `y`
```
编译器直说：**「签名没说返回值引用了 x 还是 y」**。

#### 为什么编译器不能自己推断？
关键：调用者只看签名，不看函数体。

```rust
// 调用者看到签名：fn longest(x: &str, y: &str) -> &str
// 返回的 &str 到底是 x 的一部分还是 y 的一部分？不知道！
```

考虑两种场景：
```rust
// 场景 A：返回 x（x 长）→ r 指向 x，安全
// 场景 B：返回 y（y 长）→ 但 y 可能先 drop，r 悬垂！
```
编译器无法保证安全，所以要求你明确标注。

#### 'a 标注的作用：给编译器一个「契约」
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//              ↑           ↑           ↑
//          表示「三者至少活一样久」
//          返回值 = x 和 y 中较短的那个的生命周期
```
有了契约，编译器就能检查每次调用：
- x 和 y 都活到返回值用完 → ✅
- 某个参数提前 drop → ❌ 编译错误（阻止 use-after-free）

#### 类比
借两本书给朋友，朋友还一本：
- 不标注：不知道还的是哪本，万一另一本被烧了？
- 标注 'a：签协议「还的书在两本都在的期间有效」

#### 验证：编译器的保护
```rust
// ✅ 安全
let r = longest(s1.as_str(), s2.as_str());
println!("{}", r);

// ❌ 编译错误（s2 does not live long enough）
let r;
{
    let s2 = String::from("inner");
    r = longest(s1.as_str(), s2.as_str());
}
println!("{}", r);
```

#### 什么时候不用标注？（大部分时候！）
| 情况 | 要标注吗 |
|------|---------|
| 多个引用参数 + 返回引用 | **要** ⭐ |
| 结构体字段是引用 | **要** ⭐ |
| 只有一个引用参数 | 不用（自动推断） |
| 方法里有 &self | 不用（规则 3） |
| 不返回引用 | 不用 |

> 💡 **经验**：先不写生命周期标注，写完编译，编译器的 `help:` 会直接告诉你怎么加！

---

## ✅ 第 10 章 小结

学完本章你应该掌握：
1. ✅ 写泛型函数/结构体/方法，理解单态化
2. ✅ 定义 trait、实现 trait、用 trait 做参数约束
3. ✅ 用 `+` 组合多个 trait，用 `where` 写复杂约束
4. ✅ 理解生命周期标注 `'a` 的含义和作用
5. ✅ 知道何时需要标注生命周期，何时编译器自动推断

---

## 📂 本章练习目录

- `generics/src/generics_t.rs` —— 10.1 泛型
- `generics/src/trait_t.rs` —— 10.2 Trait
- `generics/src/lifetime_t.rs` —— 10.3 生命周期
