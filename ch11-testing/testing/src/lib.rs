// ===== 第 11 章：库代码 + 单元测试 =====
// lib.rs 是库 crate 的根，里面的 pub 项可以被 main.rs 和集成测试访问

// ========================================================
// 一、被测试的代码
// ========================================================

/// 加法函数（简单示例）
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 用奇偶判断
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 可能失败的函数：根据字符串猜数字
pub fn guess_number(s: &str) -> Result<i32, String> {
    match s.trim().parse::<i32>() {
        Ok(n) if n >= 0 => Ok(n),
        Ok(_) => Err(String::from("必须是正数")),
        Err(_) => Err(String::from("不是有效数字")),
    }
}

/// 问候语结构体
pub struct Greeting {
    pub name: String,
}

impl Greeting {
    pub fn new(name: &str) -> Greeting {
        Greeting { name: name.to_string() }
    }

    pub fn format(&self) -> String {
        format!("你好，{}！", self.name)
    }
}

/// 内部函数（私有，只能在 lib 内部测试）
#[allow(dead_code)]  // 避免编译器警告未使用
fn internal_helper(x: i32) -> i32 {
    x * 2
}

// ========================================================
// 二、单元测试模块（和代码放在一起）
// ========================================================
// ⭐ 关键约定：
//   1. 用 #[cfg(test)] 标注——只在 cargo test 时编译，正常构建不包含
//   2. 模块名通常叫 tests
//   3. 每个测试函数用 #[test] 标注

#[cfg(test)]
mod tests {
    // 把外层的东西引入作用域（因为 tests 是子模块）
    use super::*;    // super 指父模块（lib.rs），* 引入所有 pub 和私有项

    // ---------- 基本测试：assert! ----------
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);     // assert_eq! 断言相等
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));          // assert! 断言为 true
        assert!(!is_even(3));         // 断言奇数为 false
    }

    // ---------- 测试 Result（更灵活）----------
    // 测试函数可以返回 Result<(), E>，用 ? 代替 assert
    #[test]
    fn test_guess_number() -> Result<(), String> {
        let n = guess_number("42")?;
        assert_eq!(n, 42);
        Ok(())
    }

    // ---------- 测试失败情况 ----------
    #[test]
    fn test_guess_number_invalid() {
        assert!(guess_number("abc").is_err());
        assert!(guess_number("-5").is_err());
    }

    // ---------- 测试结构体方法 ----------
    #[test]
    fn test_greeting() {
        let g = Greeting::new("Alice");
        assert_eq!(g.format(), "你好，Alice！");
    }

    // ---------- 测试私有函数（单元测试的优势）----------
    // 单元测试和代码在同一个模块，可以测试私有函数
    #[test]
    fn test_internal_helper() {
        assert_eq!(internal_helper(5), 10);
    }

    // ---------- 自定义失败消息 ----------
    #[test]
    fn test_with_message() {
        let result = add(1, 2);
        assert_eq!(result, 3, "add(1,2) 应该是 3，但得到了 {}", result);
    }

    // ========================================================
    // 三、should_panic：测试应该 panic 的情况
    // ========================================================
    #[test]
    #[should_panic]    // 预期这个测试函数会 panic，没 panic 反而失败
    fn test_panic() {
        let v = vec![1, 2, 3];
        let _ = v[99];   // 越界会 panic
    }

    // 可以指定 panic 消息（更精确）
    #[test]
    #[should_panic(expected = "必须是正数")]
    fn test_panic_with_message() {
        guess_number("-5").unwrap();
    }
}
