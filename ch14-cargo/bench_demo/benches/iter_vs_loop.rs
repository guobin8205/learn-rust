// ===== criterion 基准测试：迭代器 vs 循环 =====
// 运行：cargo bench -p bench_demo

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_demo::{sum_loop, sum_iter, filter_map_sum_loop, filter_map_sum_iter};

// ========================================================
// 基准 1：简单求和（对比循环 vs 迭代器）
// ========================================================
fn bench_sum(c: &mut Criterion) {
    let data: Vec<i64> = (0..1_000_000).collect();

    // 用 benchmark_group 测试不同输入规模
    let mut group = c.benchmark_group("sum (求和)");

    for size in [10_000, 100_000, 1_000_000].iter() {
        let data: Vec<i64> = (0..*size as i64).collect();

        group.bench_with_input(BenchmarkId::new("loop", size), size, |b, _| {
            b.iter(|| sum_loop(black_box(&data)))
        });

        group.bench_with_input(BenchmarkId::new("iter", size), size, |b, _| {
            b.iter(|| sum_iter(black_box(&data)))
        });
    }
    group.finish();
}

// ========================================================
// 基准 2：filter + map + sum（对比链式 vs 循环）
// ========================================================
fn bench_filter_map_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter+map+sum (复合操作)");

    for size in [10_000, 100_000, 1_000_000].iter() {
        let data: Vec<i64> = (0..*size as i64).collect();

        group.bench_with_input(BenchmarkId::new("loop", size), size, |b, _| {
            b.iter(|| filter_map_sum_loop(black_box(&data)))
        });

        group.bench_with_input(BenchmarkId::new("iter", size), size, |b, _| {
            b.iter(|| filter_map_sum_iter(black_box(&data)))
        });
    }
    group.finish();
}

// ========================================================
// 注册基准测试组
// ========================================================
criterion_group!(benches, bench_sum, bench_filter_map_sum);
criterion_main!(benches);

// ========================================================
// 运行方式：
//   cargo bench -p bench_demo
//
// 第一次运行会建立基线，后续运行会自动对比：
//   concat         time:   [1.2345 ms 1.2400 ms 1.2465 ms]
//                          change: [-3.5% -2.1% +0.8%] (p = 0.12 > 0.05)
//                          No change in performance detected
//
// 还会生成 HTML 报告：target/criterion/report/index.html
// ========================================================
