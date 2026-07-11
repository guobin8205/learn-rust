# 第 2 章：猜数游戏 (Programming a Guessing Game)

> 对应 the book 第 2 章
> 学习目标：通过一个完整小程序，感性认识 Rust 的核心语法（不要求全部掌握，第 3-6 章会详细讲）

---

## 一、程序效果

一个猜数游戏：
1. 程序生成一个 1-100 的随机数
2. 让用户反复输入猜测
3. 根据输入提示「太小了」「太大了」或「猜对了」
4. 输入非数字时提示重新输入

### 运行方式
```bash
cd ch02-guessing-game/guessing_game
cargo run
```

---

## 二、完整代码

```rust
use rand::RngExt; // rand 0.10 中，random_range 方法在 RngExt 这个 trait 里
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！");

    // 生成一个 1..=100 的随机数
    let secret_number = rand::rng().random_range(1..=100);

    // 反复让用户猜，直到猜对
    loop {
        println!("请猜一个 1 到 100 之间的数字：");

        // 存放用户输入的字符串
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // 把用户输入的字符串转换成数字
        // 用 shadowing（隐藏）复用 guess 这个名字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字！");
                continue; // 输入非法，跳过本次循环，重新让用户猜
            }
        };

        println!("你猜的数字是：{guess}");

        // 比较猜的数字和目标数字
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),    // 猜的 < 目标
            Ordering::Greater => println!("太大了！"), // 猜的 > 目标
            Ordering::Equal => {
                println!("🎉 猜对了！");
                break; // 猜对了，退出循环
            }
        }
    }
}
```

---

## 三、逐段讲解（核心知识点）

### 3.1 使用外部库（crate）

```toml
# Cargo.toml
[dependencies]
rand = "0.10.2"    # 用 cargo add rand 添加，或手动写入
```

- **crate** = Rust 的「包/库」。标准库之外的功能要用外部 crate。
- `cargo add rand` 会自动：① 改 `Cargo.toml` ② 下载到本地 ③ 写入 `Cargo.lock`
- 第一次 `cargo build` 时，cargo 会下载并编译所有依赖。

> 💡 **版本号约定**：`"0.10.2"` 表示「兼容 0.10.x 的最新版」。在 0.x.y 阶段，0.x 是兼容边界。

### 3.2 导入与 `use`

```rust
use rand::RngExt;       // 引入 trait，才能用 random_range 方法
use std::cmp::Ordering; // 引入枚举，比较结果用
use std::io;            // 引入模块，用于读取输入
```

- `use` 类似 Python 的 `from ... import`，把路径引入作用域，免去每次写全路径。
- `std` 是标准库，`rand` 是外部库。
- **重要规则**：Rust 里「方法属于 trait」时，必须 `use` 该 trait 才能调用方法。这就是为什么 `random_range` 不可用时要 `use rand::RngExt;`。

### 3.3 生成随机数

```rust
let secret_number = rand::rng().random_range(1..=100);
```

| 部分 | 含义 |
|------|------|
| `let` | 声明变量的关键字 |
| `secret_number` | 变量名（默认不可变） |
| `rand::rng()` | 获取一个线程局部随机数生成器 |
| `.random_range(1..=100)` | 生成 [1, 100] 范围内的随机整数 |
| `1..=100` | Rust 的**包含范围**语法，包含 1 和 100 |
| `1..100` | （对比）**半开范围**，包含 1 不含 100 |

> ⚠️ **rand 版本差异**（the book 与实际环境的区别）：
> - the book 用 rand 0.8：`rand::thread_rng().gen_range(1..=100)` + `use rand::Rng;`
> - 实际环境用 rand 0.10：`rand::rng().random_range(1..=100)` + `use rand::RngExt;`
> - 这正好是一次**读编译器报错并修复**的实战（见 Q&A）。

### 3.4 循环 `loop`

```rust
loop {
    // 反复执行这里的代码
    // 直到遇到 break 才退出
}
```

- `loop` 是**无限循环**，必须靠 `break` 退出。
- 这里用 `loop` 是因为我们要让用户反复猜，直到猜对。

### 3.5 可变变量 `let mut`

```rust
let mut guess = String::new();
```

| 写法 | 含义 |
|------|------|
| `let guess = ...` | 不可变变量（默认！不能修改） |
| `let mut guess = ...` | 可变变量（可以修改） |
| `String::new()` | 创建一个空字符串（`::` 调用关联函数/静态方法） |

> 💡 **Rust 的核心原则**：变量默认不可变。要修改必须显式写 `mut`。这是 Rust 安全性的基础之一（第 3 章详讲）。

### 3.6 读取用户输入 + `Result` 错误处理

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("读取输入失败");
```

| 部分 | 含义 |
|------|------|
| `io::stdin()` | 获取标准输入的句柄 |
| `read_line(&mut guess)` | 把用户输入读入 `guess`（必须传可变引用 `&mut`） |
| `&mut guess` | 「guess 的可变引用」——函数内部能修改它 |
| `.expect("...")` | `read_line` 返回 `Result`，`expect` 失败时 panic 并打印消息 |

> 💡 **`Result` 是 Rust 处理错误的核心类型**（第 9 章详讲）：
> - `Result` 有两个变体：`Ok(值)`（成功）和 `Err(错误)`（失败）
> - `read_line` 可能失败（比如读不到输入），所以返回 `Result`
> - `.expect()` = 「如果是 Err 就 panic，如果是 Ok 就取值」

### 3.7 Shadowing（变量隐藏）⭐ 重要

```rust
let guess = String::new();      // guess 是 String
// ...
let guess: u32 = match ...;     // guess 重新绑定为 u32，覆盖了原来的名字
```

- **Shadowing（隐藏）**：用 `let` 再次声明同名变量，新变量**覆盖**旧变量。
- 这里我们把字符串类型的 `guess` 重新绑定为数字类型 `u32`。
- 优点：不用造 `guess_str` / `guess_num` 两个名字，复用 `guess` 这个有意义的名字。
- 与 `mut` 不同：shadowing 是创建新变量，不是修改原变量。

### 3.8 `match` 表达式 + `parse()` 类型转换

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,           // 成功：取出数字
    Err(_) => {               // 失败：_ 表示「不关心错误类型」
        println!("请输入有效的数字！");
        continue;             // 跳过本次循环
    }
};
```

| 部分 | 含义 |
|------|------|
| `guess.trim()` | 去掉字符串首尾空白（包括用户回车产生的 `\n`） |
| `.parse()` | 把字符串解析为数字，返回 `Result`（可能失败） |
| `let guess: u32` | 手动标注类型 `u32`，告诉 `parse()` 要转成什么类型 |
| `match` | 对 `Result` 进行模式匹配，分别处理成功和失败 |
| `Ok(num) => num` | 成功分支：把 Ok 里的值绑定到 num 并返回 |
| `Err(_)` | 失败分支：`_` 是通配符，「匹配任何值但不用」 |
| `continue` | 跳过本次循环剩余部分，直接进入下一轮 |

> 💡 **`match` 是 Rust 最强大的控制结构之一**（第 6 章详讲），它要求你**穷尽所有可能**，这是安全性的保证。

### 3.9 比较数字：`cmp` 与 `Ordering`

```rust
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("太小了！"),
    Ordering::Greater => println!("太大了！"),
    Ordering::Equal   => { ... break; }
}
```

| 部分 | 含义 |
|------|------|
| `guess.cmp(&secret_number)` | 比较两个值，返回 `Ordering` 枚举 |
| `&secret_number` | 传引用（cmp 不需要拿走所有权） |
| `Ordering::Less` | guess < secret |
| `Ordering::Greater` | guess > secret |
| `Ordering::Equal` | guess == secret |
| `break` | 退出 `loop` 循环 |

---

## 四、本章涉及的语法一览（后续会深入学习）

| 语法 | 含义 | 详讲章节 |
|------|------|---------|
| `let` / `let mut` | 不可变 / 可变变量 | 第 3 章 |
| `&` / `&mut` | 引用 / 可变引用 | 第 4 章 |
| `String` / 字符串 | 堆上的可增长字符串 | 第 8 章 |
| `loop` / `break` / `continue` | 循环控制 | 第 3 章 |
| `match` | 模式匹配 | 第 6 章 |
| `enum`（Ordering） | 枚举类型 | 第 6 章 |
| `Result` / `Ok` / `Err` | 错误处理 | 第 9 章 |
| `use` / 路径 | 模块导入 | 第 7 章 |
| crate / `Cargo.toml` | 包管理 | 第 7、14 章 |
| shadowing | 变量隐藏 | 第 3 章 |
| `::`（关联函数/trait 方法） | 静态调用 | 第 5 章 |

> 📌 **本章不要求全部掌握**！the book 第 2 章的目的是让你「先有个整体感觉」。后面章节会逐一深入。

---

## 五、实战提示：如何读懂 Rust 编译器报错

本章实战中遇到了一个真实编译错误，过程很有教育意义：

**错误信息**：
```
error[E0599]: no method named `random_range` found for struct `ThreadRng`
help: trait `RngExt` which provides `random_range` is implemented but not in scope;
      perhaps you want to import it
    1 + use rand::RngExt;
```

**解读步骤**：
1. `error[E0599]`：错误码，可 `rustc --explain E0599` 查看详细说明
2. `no method named random_range found`：找不到这个方法
3. `help:` 后面是修复建议——这是 Rust 编译器最友好的地方，**直接告诉你怎么改**
4. 修复：把 `use rand::Rng;` 改成 `use rand::RngExt;`

> 💡 **养成习惯**：遇到 Rust 编译错误，认真读到底部的 `help:` 部分，经常直接就是答案。

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：为什么 `printf "..." | cargo run` 会卡住？

**A：** 这是一个**交互式程序**（使用 `io::stdin().read_line()` 等待键盘输入）。

通过管道 `printf "50\n" | cargo run` 注入输入时，在 Windows 的 Git Bash 环境下，管道的行为不可靠——程序可能读到 EOF 后 `read_line` 仍在等待，或者 cargo 父进程没有正确退出，导致看起来「卡住」。

**正确做法**：交互式程序应该在你**自己的终端**里手动运行：
```bash
cd ch02-guessing-game/guessing_game
cargo run
# 然后亲手输入数字
```

如果要自动化测试这类程序，应该用专门的测试方式（如管道重定向到文件，或用 `expect` 工具），而不是直接 `printf | cargo run`。

---

### Q2：为什么叫 shadowing？原变量释放了吗？

**A：** "Shadowing"（遮蔽/隐藏）——新变量在**名字层面**遮住了旧变量，让旧变量无法再通过这个名字访问。

**关键澄清：原变量并没有立即释放！**

```rust
let mut guess = String::new();    // ① guess 是 String
let guess: u32 = match ... { ... };  // ② shadowing，guess 变成 u32
```

内存视角：
```
执行第 ① 行后：
  变量表：guess ──→ String ""（在堆上）

执行第 ② 行后：
                     String ""（仍在内存里！只是名字访问不到了）
  变量表：guess ──→ u32 数字（栈上）  ← 新变量，遮住了旧变量
```

| 问题 | 答案 |
|------|------|
| 为什么叫 shadowing？ | 新变量"遮蔽"了旧变量的名字 |
| 创建了新变量吗？ | **是的**，shadowing 是用 `let` 创建了一个全新变量 |
| 原变量立即释放了吗？ | **没有**。原变量继续存在，直到**它所在的作用域结束**时才释放 |
| 原变量能访问吗？ | **不能**。名字被遮蔽后就访问不到了 |

**与 `mut` 的区别**：
- `mut` 是允许修改**同一个**变量的值
- shadowing 是创建一个**新的**变量（可以改类型！），只是复用了名字

```rust
let x = 5;
let x = x + 1;     // shadowing：用旧值算新值
let x = x * 2;     // 再次 shadowing

let guess = String::new();   // String 类型
let guess: u32 = 42;         // shadowing 改类型为 u32（mut 做不到！）
```

---

### Q3：match 是流操作吗？怎么区分 trim Err 还是 parse Err？

**A：** 这里有个重要的理解偏差需要澄清。

**关键点：`trim()` 根本不会产生 Err！**

```rust
let guess: u32 = match guess.trim().parse() {
//                           │     │
//                           │     └── parse() 返回 Result，可能失败
//                           └──────── trim() 返回 &str，不会失败
    Ok(num) => num,
    Err(_) => continue,
};
```

执行流程：
1. `guess.trim()` → 返回 `&str`（去掉首尾空白），**不会失败**
2. `"...".parse()` → 返回 `Result<u32, _>`，可能失败
3. `match` 匹配的是 `parse()` 的返回值（一个 `Result`）

| 疑问 | 解答 |
|------|------|
| match 是流操作吗？ | 不是。match 是一个**表达式**，对**单个已求值的值**做模式匹配 |
| 里面函数失败就进 Err？ | 不是"所有函数"。match 匹配的是 `parse()` 这一个函数的返回值 |
| 怎么知道是 trim 还是 parse 的错误？ | **trim 不会失败**！这里只有 `parse()` 可能失败 |

**关于"流"的澄清**：
- `guess.trim().parse()` 这种**方法链**才是类似"流"（前一个的输出是后一个的输入）
- `match` 是对这条链**最终的结果值**进行分支判断

**如果想分别处理多个操作的错误**，需要分开写：
```rust
match 操作A() {
    Ok(v) => match 操作B() {
        Ok(result) => { /* 都成功 */ }
        Err(e) => { /* B 的错误 */ }
    },
    Err(e) => { /* A 的错误 */ }
}
```
（第 9 章会学到更优雅的 `?` 运算符）

---

### Q4：Ordering::Less 是怎么比较的？

**A：** `.cmp()` 方法执行实际比较，返回一个 `Ordering` 枚举值告诉你是哪种大小关系。

```rust
match guess.cmp(&secret_number) {
//   │      │   │
//   │      │   └── 被比较的对象（传引用）
//   │      └────── cmp 方法：执行比较
//   └────────────── 发起比较的对象
    Ordering::Less    => ...,  // guess < secret
    Ordering::Greater => ...,  // guess > secret
    Ordering::Equal   => ...,  // guess == secret
}
```

**`Ordering` 是标准库定义的枚举**，只有三个变体：
```rust
pub enum Ordering {
    Less,    // 调用者 < 参数
    Equal,   // 调用者 == 参数
    Greater, // 调用者 > 参数
}
```

**`cmp` 方法来自 `Ord` trait**（第 10 章详讲）：
```rust
pub trait Ord {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

任何实现了 `Ord` 的类型（数字、字符、字符串等）都能用 `.cmp()`。

**为什么不用 `>` `<` `==`？**

也可以这样写，但不那么 Rust：
```rust
if guess < secret_number {
    println!("太小了！");
} else if guess > secret_number {
    println!("太大了！");
} else { ... }
```

`cmp` + `match` 的优势：
1. **一次调用**得到三种结果（`if/else` 要比较两次）
2. `match` **强制处理所有三种情况**，不会漏掉相等的情况
3. 这就是 Rust 的安全性思维——通过类型系统强制你考虑全面

---

## ✅ 第 2 章 小结

学完本章你应该：
1. ✅ 知道如何用 `cargo add` 添加外部库
2. ✅ 大致看懂猜数游戏程序的每一行代码
3. ✅ 初步认识 `let`/`let mut`、`loop`、`match`、`Result`、`&mut` 等核心语法
4. ✅ 学会读 Rust 编译器报错，特别是 `help:` 部分
5. ✅ 理解 shadowing（变量隐藏）的概念

> ⚠️ 本章很多语法只是「混个脸熟」，不用现在全搞懂。从第 3 章开始会系统学习。

---

## 📂 本章练习目录

- `guessing_game/` —— 完整的猜数游戏程序
