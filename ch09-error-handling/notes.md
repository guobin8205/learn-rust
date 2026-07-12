# 第 9 章：错误处理 (Error Handling)

> 对应 the book 第 9 章
> 学习目标：掌握 Rust 的两种错误处理——panic 和 Result，以及 ? 运算符

## 📂 示例代码
```bash
cd ch09-error-handling/errors
cargo run --bin panic_t        # 9.1 panic!
cargo run --bin result_t       # 9.2 Result<T, E>
cargo run --bin question_mark  # 9.3 ? 运算符
```

---

## Rust 错误处理的两大流派 ⭐

Rust 把错误分成两类，用两种机制处理：

| 错误类型 | 机制 | 场景 | 例子 |
|---------|------|------|------|
| **不可恢复** | `panic!` | 程序处于错误状态，继续不安全 | 数组越界、不该走的分支 |
| **可恢复** | `Result<T, E>` | 预期可能失败的操作，可以处理 | 文件不存在、网络失败、解析失败 |

> 💡 对比其他语言：Java 用异常(Exception)统一处理所有错误，Rust 分成两类。

---

## 9.1 panic! —— 不可恢复错误

### 什么是 panic？
程序**直接崩溃退出**：打印错误信息 → 清理栈（unwinding）→ 退出进程。

### 两种触发方式

```rust
// 方式 1：代码行为触发（如越界、unwrap 遇到 None）
let v = vec![1, 2, 3];
let x = v[99];       // 💥 panic: index out of bounds

// 方式 2：手动调用 panic! 宏
panic!("出错了！");   // 💥
```

### 什么时候该 panic？

| 场景 | 该 panic 吗 | 理由 |
|------|-----------|------|
| 数组越界 | ✅（标准库会） | 程序逻辑错误 |
| 进入不可能的分支 | ✅ | 程序逻辑错误 |
| 配置文件格式错误 | ⚠️ 看情况 | 启动时可 panic |
| 用户输入错误 | ❌ | 应该用 Result |
| 网络请求失败 | ❌ | 应该用 Result |
| 文件不存在 | ❌ | 应该用 Result |

**核心原则**：
- 「可以恢复的错误」→ Result
- 「不该发生的情况」→ panic

### 查看 panic 调用栈
```bash
RUST_BACKTRACE=1 cargo run --bin panic_t
```

---

## 9.2 Result<T, E> —— 可恢复错误

### Result 的定义（和 Option 很像）

```rust
enum Result<T, E> {
    Ok(T),     // 成功，携带值 T
    Err(E),    // 失败，携带错误 E
}
```

对比 Option：
- `Option<T>` = 有值 / 无值（不关心原因）
- `Result<T, E>` = 成功值 / 错误（带错误信息）

### 处理 Result 的几种方式

```rust
// 方式 1：match（最清晰）
match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => println!("失败: {e}"),
}

// 方式 2：unwrap（成功取值，失败 panic）—— 演示/确定成功时用
let f = File::open("x").unwrap();

// 方式 3：expect（带自定义消息的 unwrap）—— 比 unwrap 好
let f = File::open("x").expect("无法打开 x");

// 方式 4：unwrap_or / unwrap_or_else（安全 ✅）
let f = File::open("x").unwrap_or_else(|e| { /* 处理 */ });
```

### 错误传播：把错误返回给调用者 ⭐

很多时候你不知道怎么处理错误，应该「向上传递」给调用者：

```rust
// 传统写法（繁琐）
fn read_username() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),   // ← 把错误传播给调用者
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### Option vs Result 选择

| 情况 | 用什么 | 例子 |
|------|--------|------|
| 有值/无值（无错误信息） | Option | `vec.get(i)` |
| 成功/失败（带错误信息） | Result | `File::open()` |

---

## 9.3 ? 运算符 —— 错误传播的语法糖 ⭐⭐

### ? 是什么？

`?` 是「错误传播」的**语法糖**——一个字符代替 `match + return`：

```rust
// 不用 ?（繁琐）：
let mut f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};

// 用 ?（简洁）：
let mut f = File::open("hello.txt")?;
```

### ? 的工作逻辑

表达式后面加 `?`：
1. 如果是 `Ok(v)` → 取出 v，**继续执行**
2. 如果是 `Err(e)` → **立即 return Err(e)**，把错误传播给调用者

### ? 用于 Option（不只是 Result）

```rust
fn first_char(s: String) -> Option<char> {
    let c = s.chars().next()?;   // Some → 取值；None → 立即 return None
    Some(c.to_ascii_uppercase())
}
```

### 链式调用（非常优雅 ⭐）

```rust
// 三步操作，任何一步失败都自动返回错误
fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

对比不用 ? 的版本（11 行 vs 3 行）：

```rust
fn read_username_verbose() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### ? 的使用条件

**函数返回类型必须和 ? 处理的类型匹配**：

```rust
fn foo() -> Result<String, io::Error> {
    let f = File::open("x")?;   // ✅ io::Error 匹配
    Ok(String::new())
}

fn bar() -> Option<i32> {
    let v = some_option?;       // ✅ Option 匹配
    Some(v + 1)
}
```

### main 函数也能返回 Result

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;   // ✅ 在 main 里也能用 ?
    Ok(())
}
```

> `Box<dyn Error>` 是「任意错误类型」的写法，第 17 章详讲。

---

## 📋 错误处理速查表

| 场景 | 用什么 |
|------|--------|
| 不该发生的情况 | `panic!()` |
| 快速原型/演示 | `.unwrap()` / `.expect("msg")` |
| 可能失败的操作 | `Result<T, E>` + `match` |
| 传播错误 | `?` 运算符 ⭐ |
| 提供默认值 | `.unwrap_or(v)` / `.unwrap_or_else(\|\| ...)` |
| 有/无值（无错误信息） | `Option<T>` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`?` 自动传播错误怎么区分不同错误呢？

**A：** `?` 靠「函数返回类型」+ `From` trait 自动转换。核心是理解「错误从哪来，往哪去」。

#### `?` 展开后的真实逻辑
```rust
expr?
// 等价于：
match expr {
    Ok(v) => v,                          // 成功：取值继续
    Err(e) => return Err(From::from(e)), // 失败：用 From 转换后返回
    //                            ↑ 关键！
}
```
`?` 会调用 `From::from(e)` 把错误转成函数返回类型声明的错误。

#### 三种情况（由简到难）

**情况 1：单一错误类型（最简单）**
```rust
fn read_file(path: &str) -> Result<String, io::Error> {
    File::open(path)?.read_to_string(&mut s)?;   // 都是 io::Error
    Ok(s)
}
// 函数声明 io::Error，? 直接传播，无需区分
```

**情况 2：多种错误 → Box<dyn Error>（简单但不精确）**
```rust
fn read_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    File::open(path)?.read_to_string(&mut s)?;   // io::Error
    let n: i32 = s.trim().parse()?;               // ParseIntError
    Ok(n)   // 两种错误都被 ? 转成 Box<dyn Error>
}
// 调用处区分：e.downcast_ref::<io::Error>()  ← 笨重
```

**情况 3：自定义错误枚举（最佳实践 ⭐）**
```rust
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
}

// 实现 From 让 ? 自动转换
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn read_and_parse(path: &str) -> Result<i32, AppError> {
    File::open(path)?.read_to_string(&mut s)?;  // 自动转 AppError::Io
    let n: i32 = s.trim().parse()?;              // 自动转 AppError::Parse
    Ok(n)
}

// 调用处清晰区分：
match result {
    Err(AppError::Io(e))    => ...,   // IO 错误
    Err(AppError::Parse(e)) => ...,   // 解析错误
}
```

#### ? 转换流程图
```
File::open()? 失败 → io::Error → From → AppError::Io → return Err
s.parse()?    失败 → ParseIntError → From → AppError::Parse → return Err
```

#### 调用者区分错误的方式
| 方式 | 适用 | 写法 |
|------|------|------|
| `match` | 自定义枚举（最佳） | `Err(AppError::Io(e)) => ...` |
| `downcast_ref` | `Box<dyn Error>` | `e.downcast_ref::<io::Error>()` |
| `.kind()` | 特定错误内部 | `e.kind() == NotFound` |

#### 现实简化：thiserror / anyhow 库
```rust
// anyhow：快速原型，不区分错误类型
fn foo() -> anyhow::Result<()> { ... }

// thiserror：库开发，优雅定义（#[from] 自动实现 From）
#[derive(Error, Debug)]
enum AppError {
    #[error("IO: {0}")]
    Io(#[from] io::Error),
    #[error("Parse: {0}")]
    Parse(#[from] ParseIntError),
}
```

#### 核心记忆
> `?` 不需要自己区分错误。它把「原始错误」通过 `From` trait 转换成「函数声明的错误类型」，调用者再通过 `match` 区分不同的错误变体。

---

## ✅ 第 9 章 小结

学完本章你应该掌握：
1. ✅ 区分 panic（不可恢复）和 Result（可恢复）
2. ✅ 用 match / unwrap / expect 处理 Result
3. ✅ 理解错误传播的概念
4. ✅ 熟练使用 `?` 运算符简化错误传播
5. ✅ 知道 Option 和 Result 的选择

---

## 📂 本章练习目录

- `errors/src/panic_t.rs` —— 9.1 panic!
- `errors/src/result_t.rs` —— 9.2 Result<T, E>
- `errors/src/question_mark.rs` —— 9.3 ? 运算符
