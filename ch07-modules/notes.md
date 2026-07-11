# 第 7 章：包、Crate 和模块 (Packages, Crates, Modules)

> 对应 the book 第 7 章
> 学习目标：掌握用模块组织代码——mod、pub、路径、use

## 📂 示例代码
```bash
cd ch07-modules/restaurant
cargo run                    # 主示例：餐厅模块系统
cargo run --bin use_advanced # use 的高级用法
```

### 项目结构（⭐ Rust 模块的核心）
```
restaurant/
├── Cargo.toml
└── src/
    ├── main.rs             ← crate 根（crate root）
    ├── front_of_house.rs   ← 子模块（mod front_of_house; 引入）
    ├── back_of_house.rs    ← 子模块
    └── use_advanced.rs     ← use 用法演示（独立 bin）
```

---

## 7.1 核心概念：包 / Crate / 模块 / 路径

### 四个层次的概念（容易混淆！）

| 概念 | 含义 | 类比 |
|------|------|------|
| **Package（包）** | 一个 Cargo 项目（含一个或多个 crate） | 一个 GitHub 仓库 |
| **Crate** | 一个编译单元（二进制或库） | 一个可执行文件/库 |
| **Module（模块）** | crate 内的代码组织单元 | 文件夹 |
| **Path（路径）** | 定位某项的方式 | 文件路径 |

### Crate 的两种类型

| 类型 | 入口文件 | 产物 |
|------|---------|------|
| **Binary crate**（二进制） | `src/main.rs` | 可执行文件 |
| **Library crate**（库） | `src/lib.rs` | 库（给别人用） |

> 💡 一个 package 可以同时有 main.rs 和 lib.rs，即既有可执行又有库。

### 模块树（restaurant 示例）
```
crate (main.rs)
├── front_of_house
│   ├── hosting (pub)
│   │   ├── add_to_waitlist()  [pub]
│   │   └── seating_at_table() [私有]
│   └── serving [私有]
│       ├── take_order()       [私有]
│       └── serve_order()      [私有]
└── back_of_house
    ├── Appetizer (enum)      [pub]
    ├── Breakfast (struct)    [pub，部分字段私有]
    └── fix_incorrect_order() [pub]
```

---

## 7.2 mod：声明模块

### 模块定义的两种方式

**方式 1：内联定义**（小模块）
```rust
mod hosting {
    fn add_to_waitlist() { ... }
}
```

**方式 2：文件分离**（大模块，推荐）⭐
```rust
// main.rs 里
mod front_of_house;   // 声明：去找 src/front_of_house.rs
```
Rust 会自动找这两个位置之一：
- `src/front_of_house.rs`（新版风格，推荐）
- `src/front_of_house/mod.rs`（旧版风格）

### 模块的嵌套
```rust
// src/front_of_house.rs
pub mod hosting {           // 子模块
    pub fn add_to_waitlist() { ... }
}
```
嵌套模块对应文件系统：
```
src/front_of_house.rs              ← front_of_house 模块
src/front_of_house/hosting.rs      ← hosting 子模块（新风格）
```

---

## 7.3 pub：控制可见性 ⭐

### 核心规则：默认私有

**Rust 默认所有东西都是私有的！** 要公开必须加 `pub`。

### pub 对不同项的行为

| 项 | 默认 | 加 pub 后 |
|------|------|---------|
| `fn` 函数 | 私有 | 公开 |
| `mod` 模块 | 私有 | 公开 |
| `const` 常量 | 私有 | 公开 |
| **`struct`** | 私有 | **类型公开，但字段仍私有！** ⚠️ |
| **`enum`** | 私有 | **类型公开，所有变体也公开** |

### ⭐ struct vs enum 的 pub 行为不同（重点！）

```rust
// struct：类型公开，但字段默认私有，要逐个加 pub
pub struct Breakfast {
    pub toast: String,           // ✅ 公开字段
    seasonal_fruit: String,      // ❌ 私有！外部不能直接访问
}

// enum：类型公开，所有变体自动公开
pub enum Appetizer {
    Soup,    // 自动公开
    Salad,   // 自动公开
}
```

> 💡 **实验验证**：main.rs 里写 `meal.seasonal_fruit` 会编译错误（`field is private`），因为 `seasonal_fruit` 没有 pub。

### 可见性规则总结
1. 如果项是 `pub`，可以被「能看到该模块的任何地方」访问
2. 如果项不是 `pub`，只能在其所在模块（及子模块）内访问
3. 子模块可以访问父模块的私有项（家族内部）

---

## 7.4 路径：引用模块中的项

### 三种路径

```rust
// 绝对路径：从 crate 根开始
crate::front_of_house::hosting::add_to_waitlist();

// 相对路径：从当前模块开始
front_of_house::hosting::add_to_waitlist();

// use 引入后：短路径
use crate::front_of_house::hosting;
hosting::add_to_waitlist();   // 简短！
```

### super 关键字：引用父模块

```rust
// 在 back_of_house 模块里
super::front_of_house::hosting::add_to_waitlist();
// super 回到父模块（crate 根），再下钻
```
类似文件系统的 `..`。

---

## 7.5 use：引入路径 ⭐

### 基本用法
```rust
use std::collections::HashMap;     // 引入类型，后续直接用 HashMap
use std::io::stdin;                // 引入函数

let mut map = HashMap::new();      // 不用写全路径
```

### 最佳实践：use 引入什么

| 引入什么 | 惯例 | 原因 |
|---------|------|------|
| 函数 | 引入**父模块** `use std::io` → `io::stdin()` | 清楚函数归属 |
| 结构体/枚举 | 引入**类型本身** `use std::collections::HashMap` | 直接用名字 |
| Trait | 引入**trait 本身** `use rand::RngExt` | 才能调用方法 |

### 嵌套路径（一次引入多个）
```rust
// 繁琐：
use std::cmp::Ordering;
use std::collections::HashMap;

// 简洁（用 {} 嵌套）：
use std::{cmp::Ordering, collections::HashMap};

// glob 全引入（少用）：
use std::collections::*;   // 引入 collections 下所有项
```

### as：起别名（解决命名冲突）
```rust
use std::fmt::Result;
use std::io::Result as IoResult;   // 别名避免和上面冲突
```

### pub use：重导出
```rust
// 内部用 use 引入，同时 pub 让外部也能通过我的路径访问
pub use kitchen::Chef;
// 外部：restaurant::Chef（即使 Chef 实际在 kitchen 模块）
```
> 用于「隐藏内部结构」，对外提供简洁的 API。

---

## 📋 本章速查表

| 语法 | 作用 |
|------|------|
| `mod xxx;` | 声明子模块（内容在 src/xxx.rs） |
| `mod xxx { }` | 内联定义模块 |
| `pub` | 公开（默认私有） |
| `pub(crate)` | 只在当前 crate 内公开 |
| `crate::` | 绝对路径（从 crate 根） |
| `self::` | 当前模块 |
| `super::` | 父模块 |
| `use` | 引入路径（简写） |
| `use x as y` | 起别名 |
| `pub use` | 重导出 |
| `use std::{a, b};` | 嵌套引入 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`pub use` 是什么意思？什么时候用？

**A：** `pub use` = **重导出**——把深层路径的东西提升到浅层路径，让外部访问更简单。本质是「改变别人访问你代码的路径」。

#### 类比理解
- **普通 `use`** = 你自己从仓库拿东西到办公桌上用（只有自己能用）
- **`pub use`** = 把商品从仓库深处摆到店面入口货架（顾客一进门就能看到）

#### 普通 use vs pub use
```rust
mod network {
    pub mod tcp {
        pub fn connect(addr: &str) { ... }   // 藏得很深
    }
}

use network::tcp::connect;          // 只给自己用
pub use network::tcp::connect;      // 自己用 + 外部也能通过短路径访问
// 外部现在可以直接写 my_lib::connect（不用 network::tcp::connect）
```

#### 三个典型使用场景

**场景 1：简化长路径（最常用）**
```rust
pub use network::tcp::connect;
// 之前：my_lib::network::tcp::connect("127.0.0.1")
// 之后：my_lib::connect("127.0.0.1")   ← 短！
```

**场景 2：隐藏内部结构（封装/解耦）**
```rust
// 内部重构 network → transport，只要 pub use 路径不变
// 使用者代码完全不用改
```

**场景 3：组合多个 crate 的功能**
```rust
pub use serde::{Serialize, Deserialize};   // 使用者只依赖我的库就能用 serde
pub use chrono::DateTime;
```

#### 真实例子：标准库的 prelude
你写 Rust 能直接用 `Vec`、`String`、`Option` 而不用 `use`，就是因为标准库的 prelude 里有一堆 `pub use`：
```rust
// std::prelude 内部（简化）
pub use std::vec::Vec;
pub use std::string::String;
pub use std::option::Option::{self, Some, None};
```

#### 核心记忆
- `use` = **私有引入**（只给自己）
- `pub use` = **公开引入**（给自己 + 重导出给别人）

---

## ✅ 第 7 章 小结

学完本章你应该掌握：
1. ✅ 理解 Package / Crate / Module / Path 的层次关系
2. ✅ 用 `mod` 声明模块（内联 / 文件分离）
3. ✅ 用 `pub` 控制可见性，理解 struct 和 enum 的 pub 差异
4. ✅ 用三种路径（绝对/相对/use）引用模块中的项
5. ✅ 用 `use` 引入路径，掌握 `as` / `pub use` / 嵌套语法

---

## 📂 本章练习目录

- `restaurant/src/main.rs` —— 模块系统主示例
- `restaurant/src/front_of_house.rs` —— 子模块演示 pub/私有
- `restaurant/src/back_of_house.rs` —— struct/enum 的 pub 行为
- `restaurant/src/use_advanced.rs` —— use 高级用法
