// ===== 9.2 Result<T, E> 可恢复错误 =====
// 用 cargo run --bin result_t 运行

use std::fs::File;
use std::io::{self, Read};

// ========================================================
// Result 的定义（和 Option 很像，第 6 章学过 Option）
// ========================================================
// enum Result<T, E> {
//     Ok(T),    // 成功，携带值 T
//     Err(E),   // 失败，携带错误 E
// }
//
// 对比 Option：
//   Option<T>   = 有值 / 无值（不关心原因）
//   Result<T,E> = 成功值 / 错误（带错误信息）

fn main() {
    // ========================================================
    // 一、处理 Result 的几种方式
    // ========================================================

    // ---------- 方式 1：match 处理（最清晰）----------
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => {
            println!("文件打开成功");
            file
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                // 文件不存在，尝试创建
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("文件不存在，已创建");
                        fc
                    }
                    Err(e) => panic!("创建文件失败: {:?}", e),
                }
            }
            other_error => panic!("打开文件失败: {:?}", other_error),
        }
    };

    // ---------- 方式 2：unwrap（成功取值，失败 panic）----------
    // 只用于「确定不会失败」或「演示代码」
    // let f = File::open("hello.txt").unwrap();   // 失败会 panic

    // ---------- 方式 3：expect（带自定义消息的 unwrap）----------
    // 比 unwrap 好：错误信息更有用
    // let f = File::open("hello.txt").expect("无法打开 hello.txt");

    // ---------- 方式 4：unwrap_or / unwrap_or_else（安全）----------
    let f = File::open("不存在.txt").unwrap_or_else(|error| {
        // 失败时用闭包处理
        println!("打开失败: {:?}，使用备用方案", error);
        File::create("hello.txt").unwrap()
    });
    println!("使用了: {:?}", f);

    // ========================================================
    // 二、错误传播：把错误返回给调用者（关键概念 ⭐）
    // ========================================================
    // 很多时候，你不知道怎么处理错误，应该「向上传递」给调用者
    // 让能处理的人处理

    // 传统写法（繁琐）：
    match read_username_from_file_verbose() {
        Ok(name) => println!("用户名: {name}"),
        Err(e) => println!("读取失败: {e}"),
    }

    // ========================================================
    // 三、? 运算符：错误传播的语法糖（下节详讲）
    // ========================================================
    match read_username_from_file_short() {
        Ok(name) => println!("用户名（简写）: {name}"),
        Err(e) => println!("读取失败: {e}"),
    }

    // ========================================================
    // 四、自己返回 Result 的函数
    // ========================================================
    match parse_age("25") {
        Ok(age) => println!("年龄: {age}"),
        Err(e) => println!("解析失败: {e}"),
    }

    match parse_age("abc") {
        Ok(age) => println!("年龄: {age}"),
        Err(e) => println!("解析失败: {e}"),
    }
}

// ========================================================
// 错误传播：传统写法（繁琐）
// ========================================================
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),    // 打开失败 → 把错误返回给调用者
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),             // 成功 → 返回字符串
        Err(e) => Err(e),           // 失败 → 把错误返回
    }
}

// ========================================================
// 同样的逻辑，用 ? 运算符（下节详讲，这里先看效果）
// ========================================================
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;      // ? 自动传播错误
    let mut s = String::new();
    f.read_to_string(&mut s)?;                  // ? 自动传播错误
    Ok(s)
}

// ========================================================
// 自己定义返回 Result 的函数
// ========================================================
use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<u32, ParseIntError> {
    // parse 返回 Result，直接传播错误
    let age: u32 = s.parse()?;
    if age > 150 {
        Ok(age)
    } else {
        Ok(age)
    }
}

// ========================================================
// ⭐ Option vs Result 选择
// ========================================================
// | 情况                 | 用什么   | 例子                |
// |---------------------|---------|--------------------|
// | 有值/无值（无错误信息） | Option  | vec.get(i)         |
// | 成功/失败（带错误信息） | Result  | File::open()       |
//
// 两者可以互转：
//   ok_or()      Option → Result
//   ok()         Result → Option（丢弃错误）
//   ?            两者都能用
