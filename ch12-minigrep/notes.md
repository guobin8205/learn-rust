# 第 12 章：I/O 项目 minigrep ⭐ 实战

> 对应 the book 第 12 章
> 学习目标：综合运用前 11 章知识，做一个完整的命令行工具

## 📂 项目结构

```
minigrep/
├── Cargo.toml
├── poem.txt              ← 测试用诗歌文件
└── src/
    ├── lib.rs            ← 核心逻辑 + 单元测试
    └── main.rs           ← 入口（解析参数、调用 lib、处理错误）
```

## 运行方式
```bash
cd ch12-minigrep/minigrep
cargo run -- body poem.txt              # 搜索 body（区分大小写）
IGNORE_CASE=1 cargo run -- to poem.txt  # 不区分大小写
cargo test                              # 运行测试
```

---

## 项目功能

一个迷你版 grep：
1. 从命令行接收「查询词」和「文件路径」
2. 读取文件，找出所有包含查询词的行并打印
3. 支持通过环境变量 `IGNORE_CASE` 忽略大小写

---

## 12.1 架构设计：lib + binary 分离 ⭐

### 关注点分离（Separation of Concerns）

```
src/
├── lib.rs    ← 核心逻辑（Config、run、search）+ 测试
└── main.rs   ← 入口（解析参数、调用 run、处理错误）
```

**main.rs 的职责（尽量轻量）**：
1. 解析命令行参数
2. 调用 lib 的 run
3. 处理错误

**lib.rs 的职责（核心逻辑）**：
1. Config：配置结构体
2. run：执行搜索
3. search：搜索函数
4. 单元测试

> 💡 这是 Rust 项目的最佳实践——把逻辑放 lib.rs，main.rs 只做「入口」。这样逻辑可以被测试、被其他项目复用。

---

## 12.2 Config：用结构体组织参数（第 5 章）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("参数不足"));
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        })
    }
}
```

**知识点综合**：
- 结构体组织数据（第 5 章）
- `impl` 关联函数做构造器（第 5 章）
- 返回 `Result` 处理参数不足（第 9 章）

---

## 12.3 run：主逻辑，错误传播（第 8、9 章）

```rust
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;  // ? 传播错误

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
```

**知识点综合**：
- 文件 I/O（第 8 章）
- `?` 运算符传播错误（第 9 章）
- `Box<dyn Error>` 接受任意错误（第 9、10 章）
- `if/else` 表达式（第 3 章）

---

## 12.4 search：带生命周期的搜索函数（第 10 章）⭐

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
```

**为什么返回值需要生命周期标注**：返回的 `&'a str` 引用 contents 的数据，必须标注「这些行的生命周期和 contents 一样长」。

---

## 12.5 main.rs：入口和错误处理（第 9 章）

```rust
use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析参数出错: {err}");
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("运行出错: {e}");
        process::exit(1);
    }
}
```

**知识点综合**：
- 收集命令行参数（Vec 第 8 章）
- `unwrap_or_else` + 闭包（第 6 章闭包）
- `process::exit` 退出进程

---

## 12.6 单元测试（第 11 章）

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
```

---

## 📋 本章用到的知识点回顾

| 章节 | 知识点 | 在本项目中的应用 |
|------|--------|---------------|
| 第 3 章 | 控制流、字符串 | for 循环遍历行 |
| 第 4 章 | 借用、切片 | contents.lines() 返回切片迭代器 |
| 第 5 章 | 结构体、impl | Config 结构体 + build 方法 |
| 第 8 章 | String、Vec、文件 I/O | 读文件、存结果到 Vec |
| 第 9 章 | Result、?、错误传播 | Config::build、run |
| 第 10 章 | 生命周期、Box<dyn Error> | search 的生命周期、run 的错误类型 |
| 第 11 章 | 单元测试 | 测试 search 函数 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 12 章 小结

学完本章你应该掌握：
1. ✅ 用 lib + binary 分离的方式组织项目
2. ✅ 用结构体组织配置
3. ✅ 用 ? 运算符处理文件 I/O 错误
4. ✅ 写带生命周期的搜索函数
5. ✅ 为核心函数写单元测试
6. ✅ 综合运用前 11 章知识做出一个完整工具

---

## 📂 本章练习目录

- `minigrep/src/lib.rs` —— 核心逻辑（Config、run、search）+ 4 个单元测试
- `minigrep/src/main.rs` —— 入口
- `minigrep/poem.txt` —— 测试用诗歌
