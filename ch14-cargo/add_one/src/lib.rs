/// 加法（cargo new --lib 自动生成）
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// 加一（adder 用的函数）
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(5), 6);
    }
}
