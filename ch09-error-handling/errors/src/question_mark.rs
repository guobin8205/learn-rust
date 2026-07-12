// ===== 9.3 ? 运算符 =====
// 用 cargo run --bin question_mark 运行

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    // ========================================================
    // 一、? 运算符是什么？⭐
    // ========================================================
    // ? 是「错误传播」的语法糖，写一个字符代替 match + return

    // ---------- 看对比 ----------
    // 不用 ?（繁琐）：
    //   let mut f = match File::open("hello.txt") {
    //       Ok(file) => file,
    //       Err(e) => return Err(e),
    //   };

    // 用 ?（简洁）：
    //   let mut f = File::open("hello.txt")?;

    // ========================================================
    // 二、? 的工作逻辑
    // ========================================================
    // 表达式后面加 ?：
    //   1. 如果是 Ok(v)  → 取出 v，继续执行
    //   2. 如果是 Err(e) → 立即 return Err(e)，把错误传播给调用者
    //
    // 本质：等价于 match + return，但简洁得多

    // ========================================================
    // 三、? 用于 Option（不只是 Result）
    // ========================================================
    // ? 也能用在 Option 上：
    //   Some(v) → 取出 v
    //   None    → 立即 return None

    let result = first_char(String::from("hello"));
    println!("第一个字符: {:?}", result);

    let result = first_char(String::new());
    println!("空字符串第一个字符: {:?}", result);   // None

    // ========================================================
    // 四、? 的链式调用（非常优雅 ⭐）
    // ========================================================
    match read_username_chained() {
        Ok(name) => println!("用户名: {name}"),
        Err(e) => println!("读取失败: {e}"),
    }

    // ========================================================
    // 五、? 配合 main 函数（让 main 返回 Result）
    // ========================================================
    // main 默认返回 ()，但也可以返回 Result
    // 这样在 main 里用 ? 就不用手动处理错误了
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("hello.txt")?;   // ✅ 在 main 里也能用 ?
    //     Ok(())
    // }
    println!("\n===== ? 运算符要点 =====");
    println!("1. ? 是错误传播的语法糖（代替 match + return）");
    println!("2. Ok(v)? → 取值 v；Err(e)? → 立即 return Err(e)");
    println!("3. 能用于 Result 和 Option");
    println!("4. 链式调用让代码极简");
    println!("5. 函数返回类型必须匹配（Result 或 Option）");

    // ========================================================
    // ⭐ ? 的使用条件
    // ========================================================
    // 函数返回类型必须和 ? 处理的类型匹配：
    //
    //   fn foo() -> Result<String, io::Error> {
    //       let f = File::open("x")?;   // ✅ io::Error 匹配
    //       Ok(String::new())
    //   }
    //
    //   fn bar() -> Option<i32> {
    //       let v = some_option?;       // ✅ Option 匹配
    //       Some(v + 1)
    //   }
    //
    //   fn baz() -> Result<String, io::Error> {
    //       let n: Option<i32> = None;
    //       let n = n?;                  // ❌ Option 不能用在 Result 函数里
    //       Ok(n.to_string())
    //   }
}

// ---------- ? 用于 Option ----------
fn first_char(s: String) -> Option<char> {
    // s.chars().next() 返回 Option<char>
    // ? 让 None 时直接返回 None
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

// ---------- ? 用于 Result，链式调用 ----------
fn read_username_chained() -> Result<String, io::Error> {
    let mut s = String::new();

    // 三步操作，任何一步失败都会自动返回错误
    File::open("hello.txt")?.read_to_string(&mut s)?;

    // 等价于：
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;
    // Ok(s)

    Ok(s)
}

// ---------- ? 用于多种错误类型（需要转换，第 10 章后讲）----------
// 如果函数可能返回多种错误，需要 Box<dyn Error> 或自定义错误类型
use std::error::Error;

#[allow(dead_code)]
fn parse_and_double(s: &str) -> Result<i32, Box<dyn Error>> {
    // parse 返回 ParseIntError，但函数返回 Box<dyn Error>
    // ? 会自动转换（因为 Box<dyn Error> 实现了 From）
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// ========================================================
// 完整对比：不用 ? vs 用 ?
// ========================================================
#[allow(dead_code)]
fn verbose_version() -> Result<String, io::Error> {
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

#[allow(dead_code)]
fn short_version() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
// 这两个函数完全等价！但 short_version 只用 3 行
