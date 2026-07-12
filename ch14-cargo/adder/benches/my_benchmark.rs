// ===== 内置基准测试（test::Bencher）演示 =====
// 这是 Rust 早期内置的 benchmark，需要 nightly 工具链
// 在 stable 上用不了，所以我们另外演示 stable 的方案（criterion）

// 用一个 trick：用 #[test] 配合 Instant 来近似基准测试（stable 可用）
use std::time::Instant;

fn concat_with_plus(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s = s + &i.to_string();
    }
    s
}

fn concat_with_push(n: usize) -> String {
    let mut s = String::with_capacity(n * 4);
    for i in 0..n {
        s.push_str(&i.to_string());
    }
    s
}

// stable 兼容的简易基准测试（用 #[test] 包装）
#[test]
fn bench_plus() {
    let start = Instant::now();
    let _ = concat_with_plus(100_000);
    println!("\n用 + 拼接: {:?}", start.elapsed());
}

#[test]
fn bench_push() {
    let start = Instant::now();
    let _ = concat_with_push(100_000);
    println!("\n用 push_str 拼接: {:?}", start.elapsed());
}
