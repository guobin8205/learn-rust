# 第 14 章：Cargo 与 Crates.io

> 对应 the book 第 14 章
> 学习目标：掌握 Cargo 的高级用法——release profile、workspace、依赖管理

## 📂 示例项目
```bash
cd ch14-cargo
cargo run -p adder          # 运行 workspace 中的 adder
cargo test                  # 运行所有测试
cargo build --release       # 发布构建
```

---

## 14.1 Release Profile（编译配置）

Profile 控制编译器的优化级别，放在 **workspace 根的 Cargo.toml**：

```toml
[profile.dev]
opt-level = 0        # 开发时：不优化，编译快（默认）

[profile.release]
opt-level = 3        # 发布时：最大优化，运行快（默认）
```

### 常用配置项

| 选项 | 含义 | 取值 |
|------|------|------|
| `opt-level` | 优化级别 | 0（不优化）~ 3（最大）、's'（优化大小）、'z'（最小） |
| `debug` | 保留调试信息 | true / false |
| `panic` | panic 处理方式 | 'unwind'（展开）/ 'abort'（直接终止） |
| `lto` | 链接时优化 | true / false（release 用可提升性能） |
| `overflow-checks` | 整数溢出检查 | true / false |

### dev vs release 对比

| | dev（默认） | release |
|---|---|---|
| 编译速度 | **快**（不优化） | 慢 |
| 运行速度 | 慢 | **快**（优化） |
| 调试信息 | ✅ 有 | 通常无 |
| 用途 | 日常开发 | 发布/性能测试 |

```bash
cargo build              # dev profile
cargo build --release    # release profile
cargo run --release      # 用 release 模式运行
```

---

## 14.2 Workspace（工作空间）⭐

### 什么是 workspace？

一个 workspace 包含**多个相关的 crate**，统一管理：

```
ch14-cargo/
├── Cargo.toml           ← workspace 根（[workspace] 声明）
├── Cargo.lock           ← 共享（所有成员统一版本）
├── target/              ← 共享（节省编译时间和空间）
├── adder/               ← 成员 1：二进制 crate
│   ├── Cargo.toml
│   └── src/main.rs
└── add_one/             ← 成员 2：库 crate
    ├── Cargo.toml
    └── src/lib.rs
```

### workspace 根 Cargo.toml

```toml
[workspace]
members = [
    "adder",      # 二进制
    "add_one",    # 库
]
resolver = "2"
```

### workspace 的三大好处 ⭐

1. **共享 Cargo.lock**：所有成员用同一版本依赖（避免版本冲突）
2. **共享 target/**：节省编译时间和磁盘空间（不会每个 crate 各编译一次）
3. **crate 间互相依赖**：用 `path` 指定本地依赖

### 成员间的依赖（path 依赖）

```toml
# adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }    # 依赖同 workspace 的库
```

```rust
// adder/src/main.rs
use add_one;                          // 直接使用
fn main() {
    println!("{}", add_one::add_one(10));   // 11
}
```

### 常用 workspace 命令

```bash
cargo build                  # 编译所有成员
cargo build -p adder         # 只编译 adder
cargo run -p adder           # 只运行 adder
cargo test                   # 测试所有成员
cargo test -p add_one        # 只测试 add_one
```

### workspace vs package

| | package | workspace |
|---|---------|-----------|
| 是什么 | 一个 Cargo 项目 | 多个 package 的集合 |
| Cargo.toml | `[package]` | `[workspace]` |
| Cargo.lock | 每个 package 一个 | **共享一个** |
| target/ | 每个 package 一个 | **共享一个** |

> 💡 **何时用 workspace**：当一个项目有多个相关的 crate（比如主程序 + 多个内部库），用 workspace 统一管理最方便。

---

## 14.3 安装 crates.io 的二进制

```bash
# 安装一个命令行工具（从 crates.io）
cargo install ripgrep       # 安装 ripgrep（rg 命令）

# 安装的二进制放在 ~/.cargo/bin/
# 确保 ~/.cargo/bin 在 PATH 里就能直接用
```

> `cargo install` 只能安装**二进制**（可执行文件），不是库依赖。库依赖通过 `Cargo.toml` 的 `[dependencies]` 添加。

---

## 14.4 扩展 Cargo：自定义命令

Cargo 允许子命令扩展：任何叫 `cargo-xxx` 的二进制都能用 `cargo xxx` 调用。

```bash
cargo install cargo-expand    # 安装后可以用 cargo expand
cargo install cargo-watch     # 安装后可以用 cargo watch
```

常见的 Cargo 扩展工具：
| 工具 | 作用 |
|------|------|
| `cargo-watch` | 文件变化时自动重新编译/测试 |
| `cargo-expand` | 展开宏 |
| `cargo-edit` | `cargo add` 添加依赖（已内置到新版 cargo） |
| `cargo-audit` | 检查依赖的安全漏洞 |
| `cargo-bench` | 性能基准测试 |

---

## 14.5 发布到 crates.io（概览）

把库发布到 crates.io 让全世界用，步骤：

1. **注册 crates.io 账号** + 获取 API token
2. **完善 Cargo.toml**（必填字段）：
   ```toml
   [package]
   name = "my_lib"
   version = "0.1.0"
   edition = "2024"
   authors = ["Your Name <email>"]
   license = "MIT"              # 必须有许可证
   description = "A short description"
   repository = "https://github.com/..."
   documentation = "https://docs.rs/..."
   keywords = ["search", "cli"]
   categories = ["command-line-utilities"]
   ```
3. **发布**：
   ```bash
   cargo login <token>    # 登录（一次性）
   cargo publish          # 发布
   ```

> 💡 发布是**不可撤销**的：一旦版本发布，该版本号永远占用，不能删除或覆盖。只能发布新版本。

### 版本号约定（语义化版本 SemVer）

```
MAJOR.MINOR.PATCH
  1.    0.    0

MAJOR：破坏性改动（不兼容旧版）
MINOR：新增功能（向后兼容）
PATCH：bug 修复（向后兼容）
```

---

## 14.6 Cargo 工作流总结

| 场景 | 命令 |
|------|------|
| 创建项目 | `cargo new xxx` / `cargo new xxx --lib` |
| 添加依赖 | `cargo add rand`（自动改 Cargo.toml） |
| 编译 | `cargo build` / `cargo build --release` |
| 运行 | `cargo run` |
| 检查 | `cargo check`（不生成可执行文件，最快） |
| 测试 | `cargo test` |
| 文档 | `cargo doc --open` |
| 格式化 | `cargo fmt` |
| Lint | `cargo clippy` |
| 安装二进制 | `cargo install xxx` |
| 发布 | `cargo publish` |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`cargo-bench` 怎么用？

**A：** 内置的 `#[bench]` 在 stable Rust 上不可用（仅 nightly），业界标准是用第三方库 **criterion**。

#### 三种方案对比

| 方案 | 可用性 | 特点 |
|------|--------|------|
| 内置 `#[bench]` | ❌ stable 不可用 | 原生但被 nightly gated |
| **criterion 库** ⭐ | ✅ stable | **业界标准**，统计严谨 |
| 手动 `Instant` + `#[test]` | ✅ 都可用 | 简易近似 |

#### 方案 1：内置 `#[bench]`（需要 nightly）
```rust
#![feature(test)]           // ⚠️ 需要 nightly
extern crate test;

#[bench]
fn bench_concat(b: &mut test::Bencher) {
    b.iter(|| { /* 被测代码 */ test::black_box(s); });
}
```
运行 `cargo bench`，stable 上编译失败。

#### 方案 2：criterion 库（业界标准 ⭐）
```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false       # ⚠️ 禁用内置 harness
```
```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("concat", |b| b.iter(|| concat(black_box(1000))));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```
输出：
```
concat    time:   [180.34 µs 180.89 µs 181.45 µs]
```
**criterion 优势**：统计严谨、自动对比上次（回归检测）、生成 HTML 报告。

#### 方案 3：手动 Instant + #[test]（简易方案）
```rust
#[test]
fn bench_plus() {
    let start = Instant::now();
    let _ = concat_with_plus(100_000);
    println!("用时: {:?}", start.elapsed());
}
```
运行：`cargo test -- --nocapture`

#### black_box 是什么？
```rust
use std::hint::black_box;
let _ = black_box(result);   // 防止编译器把「没用的计算」优化掉
```

#### 实际建议
| 场景 | 推荐 |
|------|------|
| 快速验证 | 方案 3（手动 Instant） |
| 认真性能测试 | **方案 2（criterion）** ⭐ |
| 体验内置功能 | 方案 1（需要 nightly） |

---

### Q2：用 criterion 做个完整例子？

**A：** 用 criterion 对比「迭代器 vs 循环」（验证第 13 章零成本抽象）。

#### 1. 配置 Cargo.toml
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "iter_vs_loop"
harness = false       # ⚠️ 必须禁用内置 harness
```

#### 2. 写基准测试（benches/iter_vs_loop.rs）
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_demo::{sum_loop, sum_iter};

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum (求和)");
    for size in [10_000, 100_000, 1_000_000] {
        let data: Vec<i64> = (0..size as i64).collect();
        group.bench_with_input(BenchmarkId::new("loop", size), &size, |b, _| {
            b.iter(|| sum_loop(black_box(&data)))
        });
        group.bench_with_input(BenchmarkId::new("iter", size), &size, |b, _| {
            b.iter(|| sum_iter(black_box(&data)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
```

#### 3. 运行：`cargo bench -p bench_demo`
```
sum/loop/100000      time:   [11.131 µs 11.191 µs 11.430 µs]
sum/iter/100000      time:   [10.424 µs 10.683 µs 10.748 µs]
sum/loop/1000000     time:   [141.65 µs 143.40 µs 143.84 µs]
sum/iter/1000000     time:   [136.79 µs 137.22 µs 137.33 µs]
```

#### 结果分析（实战数据）⭐

**简单求和（sum）**：
| 规模 | loop | iter | 差距 |
|------|------|------|------|
| 100K | 11.19 µs | 10.68 µs | 迭代器快 4.5% |
| 1M | 143.40 µs | 137.22 µs | 迭代器快 4.3% |

**复合操作（filter+map+sum）**：
| 规模 | loop | iter | 差距 |
|------|------|------|------|
| 100K | 22.20 µs | 25.85 µs | 循环快 16% |
| 1M | 247.35 µs | 266.94 µs | 循环快 8% |

**结论**：
1. **简单求和**：迭代器略快（编译器优化更好）
2. **复合操作**：循环略快（filter+map 链有点开销）
3. **差距都很小（5-15%）**——基本证实了「零成本抽象」，但不是完全零成本

#### criterion 的核心 API
| API | 作用 |
|-----|------|
| `c.bench_function("name", \|b\| b.iter(\|\| ...))` | 单个基准测试 |
| `c.benchmark_group("组名")` | 创建测试组 |
| `group.bench_with_input(id, input, f)` | 参数化测试（不同规模） |
| `black_box(x)` | 防止编译器优化掉计算 |
| `criterion_group!` | 注册测试组 |
| `criterion_main!` | 生成 main 函数 |

#### 常用命令
```bash
cargo bench                              # 运行所有基准测试
cargo bench -- --quick                   # 快速模式（减少采样）
cargo bench -- --save-baseline before    # 保存基线
cargo bench -- --baseline before         # 对比基线（检测性能回归）
```

#### HTML 报告
criterion 会生成 HTML 报告：`target/criterion/report/index.html`
包含：统计图表、分布、对比、回归分析。

#### benchmark_group 的价值
用 `bench_with_input` 可以测试**不同输入规模**的性能曲线，比单纯一个数据点更有信息量。

---

## ✅ 第 14 章 小结

学完本章你应该掌握：
1. ✅ 配置 release profile（dev vs release）
2. ✅ 用 workspace 管理多个相关 crate
3. ✅ 用 `path` 依赖本地 crate
4. ✅ 用 `cargo install` 安装二进制
5. ✅ 了解发布到 crates.io 的流程

---

## 📂 本章练习目录

- `Cargo.toml` —— workspace 根 + profile 配置
- `adder/` —— 二进制 crate（调用 add_one）
- `add_one/` —— 库 crate
