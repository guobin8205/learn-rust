// ===== minigrep 库代码 =====
// 把核心逻辑放 lib.rs，方便测试和复用

use std::error::Error;
use std::fs;

// ========================================================
// 一、配置：用结构体组织参数（第 5 章）
// ========================================================

/// minigrep 的运行配置
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub query: String,      // 要搜索的字符串
    pub file_path: String,  // 要搜索的文件路径
    pub ignore_case: bool,  // 是否忽略大小写
}

impl Config {
    /// 从命令行参数构建 Config
    /// 返回 Result 因为可能参数不足（第 9 章）
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("参数不足，用法：minigrep <查询> <文件>"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// ========================================================
// 二、run：主逻辑（错误处理第 9 章，文件 I/O 第 8 章）
// ========================================================

/// 执行搜索逻辑
/// 返回 Result，用 Box<dyn Error> 表示「任意错误」（第 10 章）
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容（可能失败 → ?）
    let contents = fs::read_to_string(&config.file_path)?;

    // 根据是否忽略大小写，选择不同的搜索函数
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // 打印匹配的行
    for line in results {
        println!("{line}");
    }

    Ok(())
}

// ========================================================
// 三、search：核心搜索函数（返回匹配的行）
// ========================================================

/// 在 contents 中搜索包含 query 的行（区分大小写）
/// ⭐ 生命周期标注：返回的 &str 和 contents 同生命周期（第 10 章）
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// 忽略大小写的搜索
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// ========================================================
// 四、单元测试（第 11 章）
// ========================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_config_build() {
        let args = vec![
            String::from("minigrep"),
            String::from("hello"),
            String::from("poem.txt"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "hello");
        assert_eq!(config.file_path, "poem.txt");
    }

    #[test]
    fn test_config_build_error() {
        let args = vec![String::from("minigrep")];
        assert!(Config::build(&args).is_err());
    }
}
