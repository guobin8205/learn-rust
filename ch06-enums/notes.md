# 第 6 章：枚举与模式匹配 (Enums and Pattern Matching)

> 对应 the book 第 6 章
> 学习目标：掌握 enum（携带数据的枚举）、Option<T>（消灭 null）、match 模式匹配

## 📂 示例代码
```bash
cd ch06-enums/enums
cargo run --bin defining       # 6.1 枚举定义
cargo run --bin option_enum    # 6.2 Option<T>
cargo run --bin match_control  # 6.3 match 控制流
```

---

## 6.1 枚举 (Enum)

### ⭐ Rust enum 和 C/Java enum 的本质区别

Rust 的 enum 变体**可以携带数据**，而且每个变体可以携带**不同类型、不同数量**的数据：

```rust
enum Message {
    Quit,                        // 无数据
    Write(String),               // 一个 String
    Move { x: i32, y: i32 },     // 命名字段（像 struct）
    ChangeColor(i32, i32, i32),  // 三个 i32（像 tuple）
}
```

这是 Rust enum 最强大的特性——一个枚举值就把「类型标签」和「关联数据」打包在一起。

### 枚举 vs 结构体：什么时候用哪个？

| 问题 | 用结构体 | 用枚举 |
|------|---------|--------|
| 「和」关系（字段同时存在） | ✅ User{name, email, age} | ❌ |
| 「或」关系（几种情况之一） | ❌ | ✅ Message 是 Quit/Write/Move 之一 |

> 💡 经验：**「和」用 struct，「或」用 enum**。一个用户有名字+邮箱+年龄（同时有）→ struct；一条消息是 Quit 或 Write 或 Move 之一（多选一）→ enum。

### 枚举也能有方法（impl 块）

```rust
impl Message {
    fn call(&self) { ... }   // 和 struct 的方法一样
}
```

### 枚举的经典例子：IP 地址

```rust
// 用枚举把「类型」和「数据」打包（比 struct + enum 更简洁）
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
```

---

## 6.2 Option\<T\>：消灭 null ⭐

### Option 的定义

```rust
enum Option<T> {
    Some(T),   // 有值
    None,      // 无值
}
```

**Rust 没有 null！** 用 `Option<T>` 表达「可能有值，可能没有」。

### 为什么不用 null？

Tony Hoare（null 的发明者）称之为「十亿美元错误」：
- null 让「可能为空」这件事**隐藏在类型之外**
- 程序员容易忘记检查 null → 运行时空指针崩溃

Rust 的做法：**把「可能为空」写进类型系统**。`Option<T>` 和 `T` 是不同的类型，你**不能直接用** `Option<i32>` 当 `i32`，必须先处理 `None` 的情况。

```rust
let x: i32 = 5;
let y: Option<i32> = Some(5);
// let sum = x + y;          // ❌ 编译错误！类型不同
let sum = x + y.unwrap_or(0); // ✅ 必须明确处理 None
```

### 从 Option 取值的方式

| 方法 | 行为 | 何时用 |
|------|------|--------|
| `.unwrap()` | Some→取值，None→**panic** | 你确定一定是 Some |
| `.expect("msg")` | 同上，但自定义错误信息 | 同上，带提示 |
| `.unwrap_or(默认值)` | None 时用默认值 | **安全，推荐** ✅ |
| `.unwrap_or_else(\|\| 计算())` | None 时用闭包计算 | 默认值昂贵时 |
| `.is_some()` / `.is_none()` | 返回 bool | 只想判断 |
| `.map(\|x\| ...)` | 有值则变换 | 函数式风格 |
| `match` | 穷尽匹配 | **最 Rust 的方式** ✅ |

### 返回 Option 的常见方法

| 方法 | 返回 | 说明 |
|------|------|------|
| `vec.get(i)` | `Option<&T>` | 越界返回 None（不 panic） |
| `map.get(&k)` | `Option<&V>` | 键不存在返回 None |
| `str.find(p)` | `Option<usize>` | 没找到返回 None |

> 💡 `vec[i]` 越界会 panic，`vec.get(i)` 越界返回 None。后者更安全。

---

## 6.3 match 模式匹配 ⭐

### match 的基本语法

```rust
match 值 {
    模式1 => 结果1,
    模式2 => 结果2,
    _ => 默认结果,      // _ 是通配符，匹配所有其他
}
```

### match 绑定变体内部的值

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {     // ← 绑定 state
            println!("来自 {:?}", state);
            25
        }
    }
}
```

### ⭐ match 必须穷尽所有可能 (exhaustive)

漏掉任何一个分支 → **编译错误**！这是 Rust 安全性的核心保证：

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // ❌ 如果漏掉 None → non-exhaustive patterns 编译错误
        None => None,
    }
}
```

> 💡 这就是为什么 Rust 的 enum + match 比 switch 安全：编译器**强制**你处理所有情况。

### 通配符 `_` 和占位符

```rust
match dice {
    3 => println!("3"),
    6 => println!("6"),
    _ => println!("其他"),     // _ 匹配所有其他，不绑定值
}

match dice {
    3 => println!("3"),
    n => println!("是 {}", n), // n 绑定值，可以使用
}
```

### match 是表达式，有返回值

```rust
let text = match boolean {
    true => "是",
    false => "否",
};
// ⚠️ 所有分支返回类型必须相同
```

### if let：只关心一种情况的简写

```rust
// match 写法（繁琐）：
match config_max {
    Some(max) => println!("{}", max),
    _ => (),
}

// if let 写法（简洁）：
if let Some(max) = config_max {
    println!("{}", max);
}

// 配合 else 处理其他情况：
if let Coin::Quarter(state) = coin {
    println!("{:?}", state);
} else {
    count += 1;
}
```

### 模式匹配的能力（预览，第 18 章详讲）

```rust
// 解构结构体
let Point { x, y } = p;

// 解构枚举
match msg {
    Message::Write(text) => ...,
    Message::Move { x, y } => ...,
}

// 匹配范围
match age {
    0..=12 => "儿童",
    13..=19 => "青少年",
    _ => "成年",
}
```

### match vs if let 怎么选

| 情况 | 用什么 |
|------|--------|
| 处理多个分支 | match |
| 只关心一种情况，其他忽略 | if let |
| 想要穷尽检查（编译器兜底） | match |
| 想要简洁 | if let |

> 💡 `if let` 是 `match` 的语法糖，不要求穷尽，代价是失去穷尽检查。

---

## 📋 本章概念速查表

| 概念 | 语法 |
|------|------|
| 定义枚举 | `enum E { A, B(T), C{x,y} }` |
| 枚举方法 | `impl E { fn f(&self) {} }` |
| Option 有值 | `Some(value)` |
| Option 无值 | `None` |
| match 穷尽匹配 | `match x { Pat => result, _ => ... }` |
| 绑定内部值 | `Coin::Quarter(state) => ...` |
| 通配符 | `_`（匹配所有，不绑定） |
| if let 简写 | `if let Pat = x { ... }` |
| 范围匹配 | `0..=12` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`IpAddrKind::V4` / `V6` 没指定类型，默认是什么类型？

**A：它们不是「类型」，是 `IpAddrKind` 类型的「值」。** 这种不携带数据的变体叫**单元变体 (unit variant)**。

#### 核心认知：变体是「值」不是「类型」
```rust
let four = IpAddrKind::V4;
//     └─ four 的类型是 IpAddrKind
//         four 的值是 V4
```

类比理解：
| 类型 | 这个类型的值 |
|------|------------|
| `bool` | `true`, `false` |
| `i32` | `0`, `1`, `42`, ... |
| `IpAddrKind` | `V4`, `V6` |

就像 `true` 是 `bool` 的一个值，`V4` 是 `IpAddrKind` 的一个值。V4 不需要"指定类型"，因为它本身就**不是**类型。

#### 实验验证
```
IpAddrKind 占 1 字节   ← 只存「标签」（discriminant 判别式）
bool 占 1 字节          ← 大小相同
IpAddr2 占 32 字节      ← String(24字节) + 判别式 + 对齐
```

#### 三种变体形式对比
```rust
enum MyEnum {
    TagOnly,                    // 单元变体：只有标签，无数据
    WithData(i32),              // 数据变体：标签 + i32
    WithFields { x: i32, y: i32 },  // 字段变体：标签 + 字段
}
```

#### 所有变体属于同一个类型
```rust
let addrs: Vec<IpAddrKind> = vec![IpAddrKind::V4, IpAddrKind::V6];  // ✅ 同一类型
```

#### 类比其他语言
- C：`enum { V4, V6 };`（整数常量 0, 1）
- Java：`enum IpAddrKind { V4, V6 }`（枚举实例）
- TypeScript：`type T = "V4" | "V6"`（字符串字面量联合）

> 💡 关键：**V4 不是「一个类型」，而是「IpAddrKind 类型的一个值」**。单元变体在内存里只占 1 字节左右（存一个判别式标记）。

---

### Q2：感觉 enum 像其他语言的多态和泛型的结合体？

**A：直觉完全正确！** Rust 的 enum 在类型理论中叫**和类型 (Sum Type)**，实现了类似多态 + 数据封装的功能，但机制和 OOP 完全不同。

#### 「多态」体现
```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
impl Shape {
    fn area(&self) -> f64 {
        match self { ... }   // 不同形状不同行为 = 多态
    }
}
```
不同形状都是 `Shape` 类型，统一调用 `area()`——这就是多态，等价于 Java 的继承。

#### 「泛型」体现（变体携带不同类型）
```rust
enum JsonValue {
    Number(f64),      // 持有 f64
    String(String),   // 持有 String
    Bool(bool),       // 持有 bool
}
```
一个容器装多种类型——这就是 TypeScript 的 discriminated union、Haskell 的 ADT。

#### 四种「多态」方式对比
| 方式 | 语言 | 机制 |
|------|------|------|
| 继承 | Java/C++ | 子类继承父类 |
| 接口 | Java/Go | 接口约定 |
| trait 对象 | Rust | `Box<dyn Trait>` 动态分发 |
| **enum + match** ⭐ | Rust/Haskell/TS | 和类型，编译期确定 |

#### enum 多态 vs trait 多态（Rust 两种方式）
| | enum + match | trait 对象 |
|---|-------------|-----------|
| 何时确定行为 | 编译期 | 运行期 |
| 添加新变体 | 要改 enum | 不用改（开放） |
| 添加新操作 | 加 match 分支 | 要加 trait 方法 |
| 性能 | 更快 | 稍慢（动态分发） |
| 穷尽检查 | ✅ | ❌ |

#### 选型直觉
| 场景 | 推荐 |
|------|------|
| 几种固定的可能（Shape = Circle/Rect/Triangle） | **enum** ✅ |
| 可扩展的多种实现（Plugin 系统） | **trait** ✅ |
| 数据格式（JSON = Number/String/Array） | **enum** ✅ |

**核心原则**：
- 「几种固定的可能」→ enum（编译器强制处理所有情况）
- 「可扩展的多种实现」→ trait（开放关闭原则）

#### 术语
- **和类型 (Sum Type)**：A 或 B 或 C → enum
- **积类型 (Product Type)**：A 和 B 和 C → struct
- **代数数据类型 (ADT)**：和类型 + 积类型

Rust 的 `enum` 是和类型，`struct` 是积类型。Haskell/OCaml 有完整 ADT，TypeScript 的 discriminated union 也是，Rust 把它发扬到主流语言。

---

### Q3：None 为什么一定要有类型？

**A：因为 None 本身不携带任何信息，编译器无法推断它是「什么的 None」。** 这是消灭 null bug 的关键防线。

#### 核心原因：编译器没有线索
```rust
enum Option<T> {
    Some(T),   // ← T 在这里出现
    None,      // ← T 在哪里？没有信息！
}
```
| 字面量 | 编译器能看到什么 | 能否推断 T |
|--------|----------------|-----------|
| `Some(5)` | 看到 5 → i32 | ✅ |
| `None` | 什么都没有 | ❌ |

**类比**：`Option<T>` 是「可能装东西的盒子」
- `Some(5)` = 装了 5 的盒子 → 知道是装 i32 的盒子
- `None` = **空盒子** → 是装什么的盒子？必须告诉它

#### 同一个 None 可以是任意 Option<T>
```rust
let a: Option<i32> = None;     // None 是 Option<i32>
let b: Option<&str> = None;    // None 是 Option<&str> ← 不同类型！
```
None 本身是泛型的，它的完整类型是 `Option<T>`，T 待定。

#### 三种让编译器知道 T 的方式
```rust
// 方式 1：标注变量类型
let a: Option<i32> = None;
// 方式 2：turbofish
let a = None::<i32>;
// 方式 3：上下文推断（最常见）
fn find() -> Option<i32> { None }   // 返回类型已经说明
```

#### 为什么这么设计？（核心价值）
**强制 None 有类型 = 消灭 null bug 的关键**

```rust
// ❌ Java：null 无类型，任何引用都能是 null
String name = "Alice";
name = null;              // ✅ 允许！
name.length();            // 💥 NullPointerException

// ✅ Rust：None 必须是 Option<T>，不能赋值给普通类型
let name: String = String::from("Alice");
// name = None;           // ❌ 编译错误！None 不是 String
```

| 语言 | 「空」 | 有类型吗 | 后果 |
|------|-------|---------|------|
| Java/C#/JS | null | ❌ 无类型 | 任何地方都可能 null → 运行时 NPE |
| Rust | None | ✅ 必须是 `Option<T>` | 不能渗透到其他类型 |

**设计哲学**：None 不是「虚无」，而是「`Option<T>` 类型的一个值」。一个 `String` 永远是 String，绝不会变成 null。

#### 对比：None 和数字字面量很像
```rust
let x = 5;            // ❓ i32 还是 i64？要推断
let x: u8 = 5;        // ✅ 明确

let n = None;         // ❓ Option<什么>？要推断
let n: Option<u8> = None;  // ✅ 明确
```
数字 `5` 和 `None` 都是「类型待定的字面量」，都需要上下文确定。只是数字有默认（i32），None 没有。

---

### Q4：Option 相关代码里为什么有的用 `|`，有的用 `||`？

**A：它们是完全不同的东西**，只是长得像。Option 场景里的 `|` 几乎都是**闭包**。

#### 五种用法对照（核心！）
| 写法 | 含义 | 出现场景 | 例子 |
|------|------|---------|------|
| `\|x\|` | **闭包参数** | unwrap_or_else / map | `\|n\| n * 2` |
| `\|\|` | **无参闭包** | unwrap_or_else | `\|\| 999` |
| `Pat\|Pat` | 模式「或」 | match / if let | `Some(1)\|Some(2)` |
| `a \| b` | 位运算按位或 | 算术运算 | `0b1010 \| 0b0110` |
| `a\|\|b` | 逻辑或（短路） | 布尔表达式 | `true \|\| false` |

#### 你在 Option 场景看到的两种

**① `||` 无参闭包**
```rust
let v = none.unwrap_or_else(|| 999);
//                              ↑↑ 这是闭包，不是逻辑或！
```
为什么传闭包不传值？因为 `unwrap_or_else` 只在 None 时才执行闭包（懒计算）：
```rust
// unwrap_or 总是计算默认值
let v = none.unwrap_or(expensive());        // expensive() 总会执行！
// unwrap_or_else 只在 None 时计算
let v = none.unwrap_or_else(|| expensive()); // None 时才执行！
```

**② `|n|` 有参闭包**
```rust
let v = some.map(|n| n * 2);   // Some(5) → Some(10)
//                ↑  参数 n，函数体 n*2
```

#### 怎么区分？看上下文
| 看到 | 判断 |
|------|------|
| `方法(\|\| ...)` | 闭包（方法调用里） |
| `方法(\|n\| ...)` | 闭包（带参数） |
| `Some(1) \| Some(2)` | 模式或（在 match 里） |
| `整数 \| 整数` | 位或 |
| `bool \|\| bool` | 逻辑或 |

**记忆口诀**：
- **紧挨方法调用的 `|`/`||`** → 闭包
- **match 分支里的 `|`** → 模式或
- **整数间的 `|`** → 位或
- **布尔间的 `||`** → 逻辑或

#### 闭包 = 匿名函数（语法）
```rust
|参数| 函数体              // 最简
|a, b| a + b               // 多参数
|| println!("hi")          // 无参数
|n: i32| -> i32 { n * 2 }  // 完整标注（通常省略）
```
闭包第 13 章详讲，现在只需知道：**`||` 或 `|n|` 就是定义匿名小函数**。

---

### Q5：`if let` 这个语句的含义没看懂

**A：`if let` 是 `match` 的简写**，用于「只关心一种情况，其他忽略」。

#### 一句话理解
```rust
if let Some(max) = config_max { ... }
// 「如果 config_max 是 Some(...) 的样子，把里面的值赋给 max，执行花括号」
```

#### 语法结构
```
if let Some(max) = config_max { ... }
       └──┬──┘   └───┬───┘
        模式       要检查的值
```
对比普通 `if`：
- `if 条件 { ... }` → 条件为 true 才执行
- `if let 模式 = 值 { ... }` → 值匹配模式才执行

#### if let vs match 完全等价
```rust
// match（繁琐，要写 _ => ()）
match config_max {
    Some(max) => println!("{}", max),
    _ => (),
}
// if let（简洁，省掉废话）
if let Some(max) = config_max {
    println!("{}", max);
}
```

#### 典型用法
```rust
if let Some(name) = maybe_name { ... }   // 只处理 Some
if let None = empty { ... }              // 只处理 None
if let Some(v) = result { ... } else { ... }  // 配合 else
```

#### 何时用 if let，何时用 match？
| 场景 | 用什么 |
|------|--------|
| 只关心一种情况 | **if let** ✅（简洁） |
| 需要处理多种情况 | **match** ✅ |
| 想要编译器强制穷尽 | **match** ✅ |

**关键权衡**：
- `if let` = 灵活简洁，但**不要求穷尽**（可能漏掉情况）
- `match` = 啰嗦但**强制穷尽**（编译器兜底）

---

### Q6：「n 绑定了值可以使用」是什么意思？`_` 不是也包含了这个情况吗？

**A：** `_` 和 `n` 都能匹配所有情况，区别在于**分支体里能不能用那个值**。

| 模式 | 匹配范围 | 分支体里能用？ | 说明 |
|------|---------|--------------|------|
| `_` | 所有 | ❌ 不能 | 丢弃值，不命名 |
| `n` | 所有 | ✅ 能（用 `n`） | 绑定值，命名为 `n` |

#### 什么是「绑定」？
**绑定 (bind)** = 把匹配到的值起个变量名，之后能用这个名字。

```rust
let dice = 5;
match dice {
    3 => println!("是 3"),
    n => println!("不是 3，是 {}", n),   // n "接住"了 5，所以能用
    //                 ↑ 输出「不是 3，是 5」
}

match dice {
    3 => println!("是 3"),
    _ => println!("不是 3"),            // _ 丢弃了值，用不了
    //          ↑ 输出「不是 3」（没法说具体是几）
}
```

**类比**：想象接球
- `n` → 用手接住球（拿到手里，能看能玩）
- `_` → 让球掉地上（直接忽略，没接住）

#### 什么时候用哪个？
**用 `_`**：完全不关心那个值，只要兜底
```rust
match dice {
    3 => println!("3"),
    6 => println!("6"),
    _ => println!("其他"),   // 不关心具体是几
}
```

**用变量名**：想在分支体里用到那个值
```rust
match dice {
    3 => println!("3"),
    n => println!("掷出了 {}", n),   // 想打印，得用变量接住
}
```

#### 解构时最常用（展示两者用途）
```rust
let point = (10, 20);
match point {
    (x, _) => println!("x = {}", x),   // 只关心 x，y 用 _ 忽略
    (x, y) => println!("{} {}", x, y), // 都绑定
}
```
> `User { name, .. }` 里的 `..` 也是「忽略其他字段」，和 `_` 同理。

---

## ✅ 第 6 章 小结

学完本章你应该掌握：
1. ✅ 定义枚举，让变体携带不同类型的数据
2. ✅ 区分「和」用 struct、「或」用 enum
3. ✅ 理解 Option<T> 为什么消灭了 null 问题
4. ✅ 用 match / if let 处理 Option 和枚举
5. ✅ 理解 match 的穷尽性（编译器强制你处理所有情况）

---

## 📂 本章练习目录

- `enums/src/defining.rs` —— 6.1 枚举定义
- `enums/src/option_enum.rs` —— 6.2 Option<T>
- `enums/src/match_control.rs` —— 6.3 match 控制流
