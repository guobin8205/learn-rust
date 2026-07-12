// ===== cargo bench 性能基准测试示例 =====
// 运行方式：cargo bench

// Rust 内置的 benchmark API 需要用到 test crate 的 Bencher
// 注意：Rust 2024 edition 后内置 #[bench] 已经移除，
//       推荐使用第三方库 criterion，这里演示内置的简易版本

// 我们对比两种「拼接字符串」的性能
use std::hint::black_box;

fn main() {
    println!("===== 基准测试演示 =====\n");
    println!("（实际的 cargo bench 在 Rust 2024 中已改用第三方库 criterion）\n");

    // 直接手动测量对比两种方法
    let n = 100_000;

    // 方法 1：用 + 拼接
    let start = std::time::Instant::now();
    let result1 = concat_with_plus(n);
    let duration1 = start.elapsed();
    println!("方法1 用 + 拼接 {}: {:?}", n, duration1);

    // 方法 2：用 String::push_str
    let start = std::time::Instant::now();
    let result2 = concat_with_push(n);
    let duration2 = start.elapsed();
    println!("方法2 用 push_str {}: {:?}", n, duration2);

    // 用 black_box 防止编译器优化掉计算
    let _ = black_box(&result1);
    let _ = black_box(&result2);

    println!("\n两种方法结果长度相同: {}", result1.len() == result2.len());

    println!("\n===== 为什么方法 2 更快？=====\n");
    println!("方法1 (+)：每次 + 都可能触发堆分配");
    println!("方法2 (push_str)：预先 with_capacity，避免重复分配");
}

fn concat_with_plus(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s = s + &i.to_string();   // 每次都可能分配新内存
    }
    s
}

fn concat_with_push(n: usize) -> String {
    let mut s = String::with_capacity(n * 4);   // 预分配
    for i in 0..n {
        s.push_str(&i.to_string());
    }
    s
}
