// ===== 集成测试（完全在库外部测试）=====
// 文件位置：tests/integration_test.rs
// ⭐ 集成测试的特点：
//   1. 只能测试 pub 接口（不像单元测试能测私有函数）
//   2. 像普通用户一样使用这个库
//   3. 每个 tests/ 下的文件都是一个独立的集成测试 crate

// 引入我们的库（testing 是 Cargo.toml 里的 package name）
use testing::{add, Greeting};

#[test]
fn integration_test_add() {
    assert_eq!(add(10, 20), 30);
    assert_eq!(add(100, 200), 300);
}

#[test]
fn integration_test_greeting() {
    let g = Greeting::new("World");
    assert!(g.format().contains("World"));
}

#[test]
fn integration_test_multiple() {
    // 测试多个功能组合
    let sum = add(1, 2);
    let g = Greeting::new(&sum.to_string());
    assert!(g.format().contains("3"));
}
