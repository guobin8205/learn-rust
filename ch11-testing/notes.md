# 第 11 章：编写自动化测试 (Writing Automated Tests)

> 对应 the book 第 11 章
> 学习目标：掌握 Rust 的单元测试、集成测试，会写 assert 和 should_panic

## 📂 项目结构（⭐ 测试项目的标准组织）

```
testing/
├── Cargo.toml
├── src/
│   ├── lib.rs              ← 库代码 + 单元测试（#[cfg(test)]）
│   └── main.rs             ← 可执行入口（调用 lib）
└── tests/
    └── integration_test.rs ← 集成测试（完全外部测试）
```

## 运行测试
```bash
cd ch11-testing/testing
cargo test                              # 运行所有测试
cargo test test_add                     # 只运行名字匹配的测试
cargo test -- --nocapture               # 显示 println 输出
cargo test -- --ignored                 # 只运行被 #[ignore] 标注的测试
```

---

## 11.1 测试的三要素

一个测试函数做三件事：
1. **准备数据**（设置测试场景）
2. **执行代码**（调用要测试的函数）
3. **断言结果**（验证是否符合预期）

```rust
#[test]
fn test_add() {
    let result = add(2, 3);         // ① ② 准备+执行
    assert_eq!(result, 5);          // ③ 断言
}
```

---

## 11.2 单元测试（和代码放在一起）

### ⭐ 核心约定

```rust
#[cfg(test)]        // 只在 cargo test 时编译，正常构建不包含
mod tests {
    use super::*;   // 引入父模块的所有项（包括私有项）

    #[test]         // 标记这是测试函数
    fn test_something() {
        assert!(true);
    }
}
```

- `#[cfg(test)]`：条件编译，只在测试时包含这个模块
- `mod tests`：测试模块（子模块，所以用 `use super::*` 引入父模块）
- `#[test]`：标记测试函数

### 断言宏

| 宏 | 用途 | 失败行为 |
|------|------|---------|
| `assert!(cond)` | 断言条件为 true | panic |
| `assert_eq!(a, b)` | 断言相等 | panic，打印两边值 |
| `assert_ne!(a, b)` | 断言不等 | panic |
| `assert!(cond, "msg {}", x)` | 带自定义消息 | panic 时显示消息 |

```rust
#[test]
fn test() {
    assert!(is_even(4));              // 断言为 true
    assert_eq!(add(2, 3), 5);         // 断言相等
    assert_ne!(add(1, 1), 3);         // 断言不等
    assert_eq!(result, 5, "应该是 5，但得到 {}", result);  // 带消息
}
```

### 测试 Result（更灵活）

```rust
// 测试函数可以返回 Result<(), E>，用 ? 代替 assert
#[test]
fn test_parse() -> Result<(), String> {
    let n = guess_number("42")?;   // 失败时返回 Err，测试失败
    assert_eq!(n, 42);
    Ok(())
}
```

### should_panic：测试应该 panic 的情况

```rust
#[test]
#[should_panic]                    // 预期会 panic，没 panic 反而失败
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[99];                 // 越界会 panic
}

#[test]
#[should_panic(expected = "必须是正数")]   // 指定 panic 消息
fn test_negative() {
    guess_number("-5").unwrap();
}
```

### 单元测试的优势：能测私有函数

```rust
fn internal_helper(x: i32) -> i32 { x * 2 }   // 私有函数

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_internal() {
        assert_eq!(internal_helper(5), 10);   // ✅ 能测私有函数
    }
}
```

---

## 11.3 集成测试（完全外部测试）

### 位置约定

集成测试放在 **`tests/` 目录**，每个文件都是独立的测试 crate：

```
tests/
├── integration_test.rs    ← 一个集成测试 crate
└── another_test.rs        ← 另一个独立的（不会共享代码）
```

### 集成测试的特点

```rust
// tests/integration_test.rs
use testing::{add, Greeting};   // 引入库（像普通用户一样）

#[test]
fn integration_test_add() {
    assert_eq!(add(10, 20), 30);
}
```

| 特点 | 单元测试 | 集成测试 |
|------|---------|---------|
| 位置 | `src/` 内（`#[cfg(test)]`） | `tests/` 目录 |
| 测什么 | 内部细节（含私有） | 只测 pub 接口 |
| 视角 | 开发者视角 | 使用者视角 |
| 访问 | 能访问私有函数 | 只能访问 pub 项 |

### 集成测试不能测私有函数

```rust
// tests/integration_test.rs
use testing::*;

// internal_helper 是私有的，这里访问不到 ❌
// assert_eq!(internal_helper(5), 10);  // 编译错误
```

---

## 11.4 常用测试命令

```bash
cargo test                        # 运行所有测试
cargo test test_add               # 只运行名字包含 "test_add" 的测试
cargo test -- --nocapture         # 显示测试中的 println! 输出
cargo test -- --ignored           # 只运行被 #[ignore] 标注的测试
cargo test -- --test-threads=1    # 单线程运行（按顺序，调试用）
```

### `#[ignore]`：暂时跳过的测试

```rust
#[test]
#[ignore]    // 默认不运行（可能很慢或还没写完）
fn expensive_test() {
    // ...
}
```

---

## 11.5 项目组织：lib + binary

测试项目的标准结构是同时有 lib 和 binary：

```
src/
├── lib.rs     ← 库代码（可以被测试、被外部用）
└── main.rs    ← 可执行入口（调用 lib）
```

`main.rs` 里用 `use testing::xxx` 引入 lib（testing 是 package name），这样所有逻辑在 lib 里，可以被单元测试和集成测试访问。

---

## 📋 测试速查表

| 需求 | 写法 |
|------|------|
| 标记测试函数 | `#[test]` |
| 只测试时编译 | `#[cfg(test)]` |
| 断言相等 | `assert_eq!(a, b)` |
| 断言为真 | `assert!(cond)` |
| 预期 panic | `#[should_panic]` |
| 指定 panic 消息 | `#[should_panic(expected = "msg")]` |
| 跳过测试 | `#[ignore]` |
| 引入父模块 | `use super::*` |
| 单元测试位置 | `src/xxx.rs` 内 `#[cfg(test)] mod tests` |
| 集成测试位置 | `tests/xxx.rs` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 11 章 小结

学完本章你应该掌握：
1. ✅ 写单元测试（`#[test]` + `#[cfg(test)]`）
2. ✅ 用 assert! / assert_eq! / should_panic 断言
3. ✅ 写集成测试（`tests/` 目录）
4. ✅ 区分单元测试和集成测试的用途
5. ✅ 用各种 cargo test 选项控制测试运行

---

## 📂 本章练习目录

- `testing/src/lib.rs` —— 库代码 + 9 个单元测试
- `testing/src/main.rs` —— 可执行入口
- `testing/tests/integration_test.rs` —— 3 个集成测试
