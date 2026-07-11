# 第 5 章：使用结构体组织数据 (Using Structs)

> 对应 the book 第 5 章
> 学习目标：掌握结构体定义、实例化、方法（impl）、派生 Debug trait

## 📂 示例代码
```bash
cd ch05-structs/structs
cargo run --bin defining      # 5.1 定义结构体
cargo run --bin methods       # 5.2 方法与关联函数
cargo run --bin debug_trait   # 5.3 打印结构体 & Debug trait
```

---

## 5.1 定义结构体并实例化

### 三种结构体

| 类型 | 语法 | 用途 |
|------|------|------|
| **经典结构体** | `struct User { name: String, ... }` | 有命名字段（最常用） |
| **元组结构体** | `struct Color(i32, i32, i32)` | 有名字的元组，字段名不重要 |
| **单元结构体** | `struct AlwaysEqual;` | 无字段，用于在类型上实现 trait |

### 经典结构体

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 实例化：所有字段必须赋值，顺序可打乱
let user1 = User {
    active: true,
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
};
```

### 三个实用语法

**① 字段简写**（变量名和字段名相同时）：
```rust
let username = String::from("bob");
let user = User {
    username,      // ← 简写！等价于 username: username
    ...其他字段
};
```

**② 结构体更新语法**（用另一个实例填充剩余字段）：
```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1        // ← 其余字段从 user1 拿
};
// ⚠️ ..user1 会「移动」user1 的 String 字段（username、email）
//    user1 整体不能再用了，但 bool/u64 等 Copy 字段仍能用
```

**③ 元组结构体访问**：用 `.0 .1 .2`（和元组一样）
```rust
struct Point(i32, i32, i32);
let p = Point(1, 2, 3);
println!("{} {} {}", p.0, p.1, p.2);
// ⚠️ 不同元组结构体即使内部类型相同，也是不同类型
//    struct Color(i32,i32,i32); struct Point(i32,i32,i32);
//    Color 和 Point 不能互相赋值
```

### 结构体字段的所有权（重要！）

```rust
// ✅ 推荐：用 String（拥有所有权）
struct User {
    username: String,
}

// ❌ 编译错误：引用需要生命周期标注（第 10 章详讲）
// struct BadUser {
//     username: &str,
// }
```
> 现阶段：结构体字段用 `String` 这种拥有所有权的类型，避免引用。

---

## 5.2 方法与关联函数（impl 块）

### 方法 vs 关联函数

| | 方法 | 关联函数 |
|---|------|---------|
| 参数 | 有 `self` / `&self` / `&mut self` | 无 `self` 参数 |
| 调用 | 实例.方法名 `rect.area()` | 类型::函数名 `Rectangle::square(3)` |
| 类比 | 类似实例方法 | 类似「静态方法」/构造函数 |

### self 参数的四种形式 ⭐

```rust
impl Rectangle {
    fn area(&self) -> u32 { ... }           // 只读借用（最常用）
    fn scale(&mut self, f: u32) { ... }     // 可变借用（改数据）
    fn consume(self) { ... }                // 获取所有权（消耗实例）
    fn square(size: u32) -> Self { ... }    // 关联函数（无 self）
}
```

| 形式 | 含义 | 能否改 | 何时用 |
|------|------|--------|-------|
| `&self` | 只读借用 | ❌ | 读数据（最常用） |
| `&mut self` | 可变借用 | ✅ | 改数据 |
| `self` | 获取所有权 | ✅（消耗） | 转换/消耗 self |
| （无 self） | 关联函数 | — | 构造函数等 |

### 完整示例：Rectangle

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：只读借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法：接收另一个引用
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 方法：可变借用
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 关联函数：构造函数（用 :: 调用）
    fn square(size: u32) -> Self {     // Self 是 Rectangle 的别名
        Self { width: size, height: size }
    }
}
```

### 自动引用和解引用

```rust
rect1.area();   // Rust 自动加 & → 等价于 Rectangle::area(&rect1)
```
Rust 自动处理 `.` 的引用，你不用手动写 `(&rect1).area()`，只要决定「读」还是「写」即可。

### 可以有多个 impl 块
```rust
impl Rectangle { fn area(&self) {} }
impl Rectangle { fn new_method(&self) {} }   // 合法，但不常见
```

---

## 5.3 打印结构体 & 派生 Debug trait

### 问题：默认无法打印结构体
```rust
let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect);   // ❌ 编译错误！不知道怎么打印
```

### 解决：`#[derive(Debug)]`
```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
```

### 三种打印方式

```rust
// ① {:?} —— 单行调试输出
println!("{:?}", rect);
// Rectangle { width: 30, height: 50 }

// ② {:#?} —— 多行美化输出
println!("{:#?}", rect);
// Rectangle {
//     width: 30,
//     height: 50,
// }

// ③ dbg!() 宏 —— 开发调试利器
let r = Rectangle { width: dbg!(2 * 10), height: 50 };
dbg!(&r);
// [src/main.rs:25] 2 * 10 = 20
// [src/main.rs:26] &r = Rectangle { width: 20, height: 50 }
```

### println! vs dbg!

| 特性 | println! | dbg! |
|------|----------|------|
| 输出流 | stdout（标准输出） | stderr（标准错误） |
| 打印格式 | 自定义 | 固定 Debug 格式 |
| 行号 | ❌ | ✅ 文件名:行号 |
| 返回值 | ❌（返回 `()`） | ✅ 返回表达式的值 |
| 场景 | 给用户看 | 开发调试用 |

### 常见的派生 trait（预览）

```rust
#[derive(Debug, Clone, PartialEq)]   // 可以一次派生多个
struct Rectangle { width: u32, height: u32 }
```

| derive | 作用 | 第几章详讲 |
|--------|------|-----------|
| `Debug` | 支持 `{:?}` 打印 | 本章 |
| `Clone` | 支持 `.clone()` 深拷贝 | 第 10 章 |
| `Copy` | 支持按位拷贝 | 第 10 章 |
| `PartialEq` | 支持 `==` `!=` | 第 10 章 |
| `Hash` | 支持放入 HashMap | 第 10 章 |

---

## 📋 本章概念速查表

| 概念 | 语法 |
|------|------|
| 定义结构体 | `struct Name { field: Type, ... }` |
| 元组结构体 | `struct Name(Type, Type, ...)` |
| 单元结构体 | `struct Name;` |
| 方法 | `impl Name { fn f(&self) {} }` |
| 关联函数 | `impl Name { fn f() -> Self {} }` |
| 字段简写 | `username,`（省略 `:username`） |
| 更新语法 | `..other_instance` |
| 调试打印 | `#[derive(Debug)]` + `{:?}` / `{:#?}` |
| 调试宏 | `dbg!(expr)` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`#[allow(dead_code)]` 是什么意思？

**A：** 是一个「属性标注」(attribute)，告诉编译器「这段代码没被使用我知道了，别再警告」。

#### 什么是 dead_code？
「死代码」= 定义了但从未被使用的代码。Rust 编译器对此发 **warning**（不是 error，能编译通过）：

| 情况 | 例子 |
|------|------|
| 字段从未被读取 | struct 的某字段只写不读 |
| 函数从未被调用 | 定义了 `fn foo()` 但没人调用 |
| 常量从未被使用 | `const MAX: u32 = 100;` 但没用 |
| 导入从未被使用 | `use std::io;` 但没用 `io` |

#### 为什么第 5 章用到？
`defining.rs` 里的 `build_user` 函数是为**演示**构造函数模式写的，`main` 没调用它。不加 `#[allow(dead_code)]` 会警告 `function build_user is never used`，加了就闭嘴。

#### 作用域：`#[...]` vs `#![...]`
```rust
#[allow(dead_code)]            // 一个 # → 只作用于「紧跟的下一项」
fn foo() {}

#![allow(dead_code)]           // #! → 作用于「整个文件/模块」
```

#### 类似的 allow 系列
| 属性 | 抑制什么 |
|------|---------|
| `#[allow(dead_code)]` | 死代码 |
| `#[allow(unused_variables)]` | 未使用的变量 |
| `#[allow(unused_imports)]` | 未使用的 import |
| `#[allow(non_snake_case)]` | 非 snake_case 命名 |
| `#![allow(warnings)]` | 所有警告 |

#### 什么时候用？
| 场景 | 建议 |
|------|------|
| 教学/示例代码 | ✅ 用 |
| 库的公共 API | ✅ 用 |
| 正在开发的代码 | ⚠️ 谨慎用 |
| 正式生产代码 | ❌ 尽量不用（死代码该删掉） |

#### 拓展：Rust 属性系统
`#[...]` 是属性（类似 Java 注解、Python 装饰器），常见属性：
```rust
#[derive(Debug, Clone)]    // 自动生成 trait 实现
#[allow(dead_code)]        // 抑制 lint
#[cfg(test)]               // 只在测试配置下编译
```

---

### Q2：`self`（消耗）的使用场景能举例吗？

**A：** `self`（无引用）会**消耗实例**——调用后原实例失效。主要用于「转换」「释放资源」「Builder 链式调用」。

#### 场景 1：Builder 模式（方法链）⭐ 最经典
```rust
let html = HtmlBuilder::new("div")
    .add_class("header")    // fn add_class(mut self, ...) -> Self
    .add_id("main")         // 消耗 self，返回新 self
    .build();               // 最终消耗 self，返回 String
```
每个方法消耗进来的 self、返回新 self，所有权在链中流转，零拷贝。**不用 `&mut self` 是因为它返回引用无法链式**。

#### 场景 2：转换数据格式（`into_` 惯例）
```rust
let url = config.into_url();   // 消耗 config 转成 String
// config 已失效
```
把结构体消耗转换成另一类型，避免克隆。

#### 场景 3：释放内部资源（零拷贝搬数据）
```rust
let content = msg.into_content();   // 把内部 String 抽出来
```
`&self` 要返回内部数据只能 `.clone()`（有代价），`self` 直接搬走，零成本。

#### 标准库里的常见例子
| 方法 | 作用 |
|------|------|
| `String::into_bytes(self)` | 消耗 String → `Vec<u8>` |
| `Vec::into_iter(self)` | 消耗 Vec → 迭代器 |
| `Option::unwrap(self)` | 消耗 Option → 取出值 |
| `Result::unwrap(self)` | 消耗 Result → 取出值或 panic |

#### ⭐ Rust 命名惯例（一眼判断方法行为）
| 前缀 | self 形式 | 含义 | 例子 |
|------|----------|------|------|
| `as_` | `&self` | 廉价引用转换 | `as_str()` |
| `to_` | `&self` | 克隆并转换（有代价） | `to_string()` |
| `into_` | `self` ⭐ | **消耗** self 转换 | `into_bytes()` |
| `get_` | `&self` | 获取字段 | `get_ref()` |
| `set_` | `&mut self` | 设置字段 | `set_name()` |

**关键记忆**：
- `into_` → 消耗 self，原实例失效
- `to_` → 克隆，原实例仍可用
- `as_` → 廉价引用，不消耗不克隆

#### 三种 self 本质对比
```rust
fn length(&self) -> usize { ... }        // 借用读，实例活着
fn exclaim(&mut self) { ... }            // 借用写，实例活着
fn into_content(self) -> String { ... }  // 消耗，实例死了，数据搬走
```
需要 `self` 是因为「数据搬走比克隆更高效」——零成本转移所有权。

---

### Q3：`#[derive(Debug)]` 的打印规则是什么？是 JSON 吗？

**A：不是 JSON**，是 Rust 自己的 Debug 格式，规则简单固定。

#### Debug 输出规则（实验总结）
| 类型 | 格式 | 例子 |
|------|------|------|
| 字符串 | 双引号，转义特殊字符 | `"hello\nworld"` |
| 元组 | 圆括号 | `(1, "two", 3.0)` |
| 数组/Vec | 方括号 | `[1, 2, 3]` |
| 经典结构体 | `名字 { 字段: 值 }` | `User { name: "alice", age: 30 }` |
| 元组结构体 | `名字(值)` | `Point(10, 20)` |
| 枚举-无数据 | 变体名 | `Quit` |
| 枚举-有关联值 | `变体(值)` 或 `变体 { 字段: 值 }` | `Write("hi")` / `Move { x: 1, y: 2 }` |
| Option/Result | `Some(5)` / `None` / `Ok(42)` / `Err("...")` | |

#### Debug vs JSON 的区别
| 特征 | JSON | Rust Debug |
|------|------|-----------|
| 字段名引号 | `"name":` 必须有 | `name:` **无引号** |
| 类型名 | 无 | **有**（`User { ... }`） |
| 元组/枚举 | 不支持 | 支持 |

#### 单行 `{:?}` vs 多行 `{:#?}`
```
{:?}  → 单行紧凑
{:#?} → 多行美化（# = pretty-print）
```

#### Debug vs Display 两个层次
| trait | 受众 | 格式 | 实现 |
|-------|------|------|------|
| **Debug** | 程序员（调试） | `{:?}` | `#[derive(Debug)]` 自动派生 |
| **Display** | 用户（正式输出） | `{}` | **必须手写**，不能 derive |

> 想要真正的 JSON 输出？用 `serde_json` 库 + `#[derive(Serialize)]`，那是另一个话题。Debug 格式只是给调试用的，不保证跨版本兼容。

---

## ✅ 第 5 章 小结

学完本章你应该掌握：
1. ✅ 定义三种结构体（经典、元组、单元）
2. ✅ 实例化、字段简写、结构体更新语法
3. ✅ 用 `impl` 块定义方法和关联函数
4. ✅ 区分 `&self` / `&mut self` / `self` / 关联函数
5. ✅ 用 `#[derive(Debug)]` 打印结构体，会使用 `dbg!` 宏

---

## 📂 本章练习目录

- `structs/src/defining.rs` —— 5.1 定义结构体
- `structs/src/methods.rs` —— 5.2 方法与关联函数
- `structs/src/debug_trait.rs` —— 5.3 打印结构体 & Debug trait
