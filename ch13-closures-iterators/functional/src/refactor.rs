// ===== 13.3 用迭代器重构 minigrep =====
// 用 cargo run --bin refactor 运行
// 演示：用迭代器改写 search 函数，对比循环版本

fn main() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    println!("===== 对比：循环版本 vs 迭代器版本 =====\n");

    // 循环版本（第 12 章写的）
    let result1 = search_loop("duct", contents);
    println!("循环版本: {:?}", result1);

    // 迭代器版本（第 13 章重构）
    let result2 = search_iter("duct", contents);
    println!("迭代器版本: {:?}", result2);

    assert_eq!(result1, result2);   // 结果完全相同
    println!("\n两个版本结果相同 ✅");

    println!("\n===== 迭代器版本更简洁、更函数式 =====\n");

    // 循环版本（7 行）：
    //   pub fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //       let mut results = Vec::new();
    //       for line in contents.lines() {
    //           if line.contains(query) {
    //               results.push(line);
    //           }
    //       }
    //       results
    //   }
    //
    // 迭代器版本（1 行核心逻辑！）：
    //   pub fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //       contents.lines().filter(|line| line.contains(query)).collect()
    //   }

    // ========================================================
    // ⭐ 零成本抽象的验证
    // ========================================================
    // 两种写法编译后**等价**，运行时性能相同！
    // 迭代器版本没有额外的开销，因为：
    //   1. 迭代器适配器（filter、map）是内联优化的
    //   2. 没有中间集合分配（filter 返回的是惰性迭代器，不是 Vec）
    //   3. 编译器能做更激进的优化
    //
    // 所以你可以放心用迭代器，既简洁又不损失性能！

    println!("\n===== 更多迭代器重构例子 =====\n");

    // 例子 1：求和（循环 vs 迭代器）
    let nums = vec![1, 2, 3, 4, 5];

    // 循环版本
    let mut sum_loop = 0;
    for n in &nums { sum_loop += n; }
    println!("循环求和: {}", sum_loop);

    // 迭代器版本
    let sum_iter: i32 = nums.iter().sum();
    println!("迭代器求和: {}", sum_iter);

    // 例子 2：找出最大值
    let mut max_loop = nums[0];
    for n in &nums { if n > &max_loop { max_loop = *n; } }
    println!("循环最大值: {}", max_loop);

    let max_iter = nums.iter().max();
    println!("迭代器最大值: {:?}", max_iter);

    // 例子 3：复合操作（filter + map + collect）
    // 找出偶数并翻倍
    let result: Vec<i32> = nums.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("偶数翻倍: {:?}", result);   // [4, 8]
}

// ========================================================
// 循环版本（第 12 章的风格）
// ========================================================
fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// ========================================================
// 迭代器版本（第 13 章重构）⭐
// ========================================================
fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
