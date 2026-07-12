/// bench_demo：演示 criterion 基准测试
/// 对比「迭代器版本」vs「循环版本」——验证第 13 章的零成本抽象

/// 循环版本：求和
pub fn sum_loop(data: &[i64]) -> i64 {
    let mut total = 0;
    for &x in data {
        total += x;
    }
    total
}

/// 迭代器版本：求和
pub fn sum_iter(data: &[i64]) -> i64 {
    data.iter().sum()
}

/// 循环版本：过滤 + 变换 + 求和
pub fn filter_map_sum_loop(data: &[i64]) -> i64 {
    let mut total = 0;
    for &x in data {
        if x % 2 == 0 {
            total += x * x;
        }
    }
    total
}

/// 迭代器版本：过滤 + 变换 + 求和
pub fn filter_map_sum_iter(data: &[i64]) -> i64 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}
