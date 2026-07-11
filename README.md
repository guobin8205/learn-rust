# Rust 学习课程（基于《Rust 权威指南》）

> 工具链：rustc 1.96.0 / cargo 1.96.0
> 教材：The Rust Programming Language (the book)
> 每章包含：知识点笔记 + 配套练习代码

---

## 阶段一：入门基础（第 1–3 章）
目标：跑通开发环境，掌握 Rust 基本语法。

### 第 1 章 入门 (Getting Started)
- 安装 rustup / rustc / cargo，配置 IDE（rust-analyzer）
- `cargo new` / `cargo build` / `cargo run` / `cargo check`
- Hello World，理解 Cargo 项目结构
- 📁 练习目录：`ch01-getting-started/`

### 第 2 章 猜数游戏 (Programming a Guessing Game)
- 第一个完整小程序，感性认识 Rust
- 标准库导入、`mut`、`::` 语法、`Result` 处理、外部 crate (`rand`)
- 📁 练习目录：`ch02-guessing-game/`

### 第 3 章 常见编程概念 (Common Programming Concepts)
- 变量与可变性（`let` / `let mut` / 常量 `const` / 隐藏 shadowing）
- 数据类型（标量 / 复合类型 / 数组 / 元组）
- 函数、控制流（`if` / `loop` / `while` / `for`）
- 📁 练习目录：`ch03-concepts/`

---

## 阶段二：核心机制（第 4–6 章）
目标：攻克 Rust 的灵魂——所有权系统。

### 第 4 章 所有权 (Ownership) ⭐ 最关键
- 所有权三大规则、移动语义 (Move)、借用 (Borrow)、引用
- 切片 (slice) `&str` / `&[T]`
- 📁 练习目录：`ch04-ownership/`

### 第 5 章 结构体 (Using Structs)
- 定义结构体、元组结构体、单元结构体
- 方法与关联函数 `impl` 块
- 📁 练习目录：`ch05-structs/`

### 第 6 章 枚举与模式匹配 (Enums and Pattern Matching)
- 枚举 `enum`、`Option<T>`（消灭空指针）
- `match` / `if let` / `while let`
- 📁 练习目录：`ch06-enums/`

---

## 阶段三：组织与工具（第 7–9 章）
目标：能组织中型项目，掌握常用数据结构与错误处理。

### 第 7 章 包、Crate 和模块 (Packages, Crates, Modules)
- crate / module / path / `pub` / `use` / 重导出
- 📁 练习目录：`ch07-modules/`

### 第 8 章 常见集合 (Common Collections)
- `Vec<T>`、`String`、`HashMap<K,V>`
- 📁 练习目录：`ch08-collections/`

### 第 9 章 错误处理 (Error Handling)
- `panic!` 与不可恢复错误
- `Result<T, E>` 与可恢复错误、`?` 运算符
- 📁 练习目录：`ch09-error-handling/`

---

## 阶段四：进阶特性（第 10–13 章）
目标：写出地道的 Rust 代码，掌握测试驱动开发。

### 第 10 章 泛型、Trait 和生命周期 ⭐ 重点
- 泛型函数/结构体/枚举
- Trait 定义与实现、Trait bound、默认实现
- 生命周期 `'a`、消除规则
- 📁 练习目录：`ch10-generics-traits/`

### 第 11 章 编写自动化测试 (Writing Automated Tests)
- 单元测试、集成测试、`#[test]` / `#[cfg(test)]`
- 📁 练习目录：`ch11-testing/`

### 第 12 章 I/O 项目：minigrep ⭐ 实战
- 综合运用前面知识，实现一个 mini grep 命令行工具
- 📁 练习目录：`ch12-minigrep/`

### 第 13 章 闭包与迭代器 (Functional Features)
- 闭包语法、捕获方式、`Fn`/`FnMut`/`FnOnce`
- 迭代器 `Iterator` trait、消费器/适配器、零成本抽象
- 用迭代器重构 minigrep
- 📁 练习目录：`ch13-closures-iterators/`

---

## 阶段五：高级实战（第 14–20 章）
目标：掌握高级特性，完成综合项目。

### 第 14 章 Cargo 与 Crates.io
- release profile、工作空间 workspace、发布发布到 crates.io
- 📁 练习目录：`ch14-cargo/`

### 第 15 章 智能指针 (Smart Pointers)
- `Box<T>`、`Rc<T>` / `Arc<T>`、`RefCell<T>` 与内部可变性
- 📁 练习目录：`ch15-smart-pointers/`

### 第 16 章 无畏并发 (Fearless Concurrency)
- 线程 `thread::spawn`、消息传递 `mpsc`、共享状态 `Mutex`
- `Send` / `Sync` trait
- 📁 练习目录：`ch16-concurrency/`

### 第 17 章 Rust 的面向对象特性
- 封装、继承的替代方案（Trait 对象 `dyn Trait`）
- 📁 练习目录：`ch17-oo-features/`

### 第 18 章 模式与匹配 (Patterns and Matching)
- 模式语法全解、refutable / irrefutable 模式
- 📁 练习目录：`ch18-patterns/`

### 第 19 章 高级特性 (Advanced Features)
- 不安全 Rust、关联类型、宏 macro、类型别名
- 📁 练习目录：`ch19-advanced/`

### 第 20 章 多线程 Web 服务器（终章）⭐ 综合实战
- 从零构建一个多线程 Web 服务器
- 📁 练习目录：`ch20-web-server/`

---

## 学习建议
1. **按顺序学**：章节之间环环相扣，尤其第 4 章所有权是后续所有内容的基础。
2. **每章必动手**：每个练习目录都自己敲一遍代码，不要只看。
3. **遇到编译器报错别怕**：Rust 编译器是最好的老师，认真读错误信息。
4. **推荐练习平台**：学完基础后可去 [Rustlings](https://github.com/rust-lang/rustlings) 刷小练习。
5. ⭐ 标记的章节是重中之重，建议多花时间。
