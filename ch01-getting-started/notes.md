# 第 1 章：入门 (Getting Started)

> 对应 the book 第 1 章
> 学习目标：掌握 Cargo 项目结构和常用命令，会创建、编译、运行 Rust 项目

---

## 一、Cargo 项目结构（必须记牢）

```
hello_cargo/              ← 项目根目录
├── Cargo.toml            ← 📋 清单文件（项目配置 + 依赖），手动维护
├── Cargo.lock            ← 🔒 依赖版本锁定，自动生成，不要手动改
├── src/                  ← 所有源代码都放这里
│   └── main.rs           ← 程序入口（二进制 crate 的根）
└── target/               ← 编译产物（可执行文件），被 .gitignore 忽略
    └── debug/
        └── hello_cargo.exe   ← 编译出的可执行文件
```

> 💡 **关键规则**：源代码必须放在 `src/` 目录下，配置文件 `Cargo.toml` 放在根目录。

### 创建项目的命令
```bash
cargo new hello_cargo
```
会自动生成上述结构，并初始化一个 git 仓库。

---

## 二、`Cargo.toml` —— 清单文件

```toml
[package]            # ← "表"(table) 声明，方括号开头
name = "hello_cargo" # 包名（也是生成可执行文件的名字）
version = "0.1.0"    # 语义化版本号
edition = "2024"     # Rust 版本（每 3 年一次，决定语言特性）

[dependencies]       # 第三方依赖写在这里（现在为空）
```

- `edition`：当前为 `2024`，决定可用的语言特性，每 3 年更新一次。
- 后面第 2 章用到 `rand` 库时，会在 `[dependencies]` 下添加 `rand = "0.8"`。

---

## 三、`src/main.rs` —— 入口代码

```rust
fn main() {                      // fn = 声明函数，main 是程序入口
    println!("Hello, world!");   // println! 是「宏」不是函数，注意感叹号!
}
```

三个新手必知细节：
- `fn` 是声明函数的关键字
- `main` 函数是程序入口，无参数无返回值
- **`println!` 带感叹号是「宏 (macro)」**，不是普通函数（后面会讲区别，现在记住带 `!` 即可）

---

## 四、四个最常用的 Cargo 命令（核心！）

| 命令 | 作用 | 什么时候用 |
|------|------|-----------|
| `cargo new <名字>` | 创建新项目 | 开始新项目 |
| `cargo build` | 编译（生成可执行文件） | 只想编译不运行 |
| `cargo run` | **编译 + 运行**（最常用✅） | 日常开发，写完就跑 |
| `cargo check` | **只检查能否编译，不生成文件**（最快） | 写代码过程中频繁检查 |
| `cargo build --release` | 优化编译 | 发布/性能测试时 |

> 💡 **重要**：日常开发用 `cargo check` 比 `cargo build` 快很多，因为它跳过了生成机器码的步骤。养成「写一段 → `cargo check` 一下」的习惯。

### 命令演示（带实际输出）
```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.30s

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\hello_cargo.exe`
Hello, world!
```

---

## 五、Debug vs Release

| 模式 | 命令 | 产物路径 | 特点 |
|------|------|---------|------|
| Debug（默认） | `cargo build` | `target/debug/` | 未优化 + 带调试信息，**编译快** |
| Release | `cargo build --release` | `target/release/` | 优化后，**编译慢，运行快** |

- 日常开发用 Debug
- 性能测试或发布用 Release

---

## 六、本章涉及的核心概念速查

| 概念 | 说明 |
|------|------|
| `rustc` | Rust 编译器（平时很少直接用，都用 cargo） |
| `cargo` | Rust 的构建工具 + 包管理器 |
| `crate` | Rust 的编译单元/包（后面第 7 章细讲） |
| `Cargo.toml` | 项目配置清单（手动维护） |
| `Cargo.lock` | 依赖版本锁定（自动生成，勿手动改） |
| `fn` | 声明函数的关键字 |
| `main` | 程序入口函数 |
| `!`（如 `println!`） | 宏的标志符号 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

---

### Q1：如何在 VSCode 中使用 Cargo？

**A：** 分为「装插件」→「打开项目」→「日常使用」三步。

#### 1. 安装插件

| 插件 | 插件 ID | 是否必须 |
|------|---------|---------|
| **rust-analyzer** | `rust-lang.rust-analyzer` | ⭐ 必须 |
| **Even Better Toml** | `tamasfe.even-better-toml` | 推荐（Cargo.toml 语法高亮） |
| **CodeLLDB** | `vadimcn.vscode-lldb` | 推荐（断点调试） |
| **Error Lens** | `usernamehw.errorlens` | 推荐（错误信息显示在行尾） |

> ⚠️ 不要装旧的 `rust` 插件（已废弃），只装 `rust-analyzer` 即可。

#### 2. 打开项目（关键！）

必须打开**包含 `Cargo.toml` 的文件夹**，rust-analyzer 才会激活：

- 方式 1（命令行）：`code hello_cargo`
- 方式 2（VSCode）：`文件` → `打开文件夹` → 选含 `Cargo.toml` 的目录

#### 3. 日常操作

| 操作 | 方法 |
|------|------|
| 运行项目 | 内置终端（`` Ctrl+` ``）里敲 `cargo run` |
| 断点调试 | 点行号设断点 → 按 `F5` → 选 LLDB 环境 |
| 格式化代码 | `Shift + Alt + F`（基于 rustfmt） |
| 跳转定义 | `F12` |
| 查看引用 | `Shift + F12` |
| 快速修复 | `Ctrl + .` |
| 重命名 | `F2` |

#### 4. 推荐 settings.json 配置

```jsonc
{
    // 输入时实时用 clippy 检查
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.completion.autoimport.enable": true,

    // 保存时自动格式化
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

#### 5. 前置条件检查

环境应包含以下组件（用 `rustup component list --installed` 查看）：
- `rust-analyzer` —— 语言服务器（补全/跳转/检查）
- `clippy` —— 代码 lint
- `rustfmt` —— 代码格式化
- `rust-src` —— 标准库源码（看源码用）

本机当前已全部安装 ✅

#### 6. 常见问题

| 问题 | 解决方案 |
|------|---------|
| rust-analyzer 一直转圈不结束 | 首次加载会构建索引，等几分钟；或重启 VSCode |
| 没有代码补全 | 检查是否打开了**含 Cargo.toml 的文件夹** |
| 提示找不到 toolchain | 确认 `rustc --version` 能正常运行 |
| 调试按 F5 没反应 | 确认装了 CodeLLDB 插件，并选了 LLDB 环境 |

---

## ✅ 第 1 章 小结

学完本章你应该掌握：
1. ✅ 会用 `cargo new` 创建项目
2. ✅ 看懂 `Cargo.toml` 和 `src/main.rs` 的关系
3. ✅ 熟练使用 `cargo run` / `cargo build` / `cargo check`
4. ✅ 理解 Rust 项目的基本结构，知道 Debug 和 Release 的区别

---

## 📂 本章练习目录

- `hello_cargo/` —— Cargo 自动生成的 Hello World 项目
