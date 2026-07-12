// ===== 13.2 迭代器 (Iterators) =====
// 用 cargo run --bin iterators 运行

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    println!("===== 1. 迭代器基础 =====\n");

    let v = vec![1, 2, 3, 4, 5];

    // ⭐ 迭代器是「惰性」的：创建时什么都不做，只有被消费才求值
    let iter = v.iter();           // 创建迭代器（还没开始遍历）
    for val in iter {              // 这里才真正遍历
        print!("{val} ");
    }
    println!();

    println!("\n===== 2. Iterator trait =====\n");

    // 实现了 Iterator trait 的类型就是迭代器
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;   // 核心方法
    // }
    // 只需实现 next，其他方法都有默认实现

    let mut iter = v.iter();
    println!("next: {:?}", iter.next());   // Some(1)
    println!("next: {:?}", iter.next());   // Some(2)
    println!("next: {:?}", iter.next());   // Some(3)
    println!("next: {:?}", iter.next());   // Some(4)
    println!("next: {:?}", iter.next());   // Some(5)
    println!("next: {:?}", iter.next());   // None（结束）

    println!("\n===== 3. 三种迭代方式 =====\n");

    let v = vec![10, 20, 30];

    // iter()：借用元素 &T
    for val in v.iter() { print!("{val} "); }
    println!("→ iter（借用，v 仍可用: {:?})", v);

    // into_iter()：获取所有权 T（消耗集合）
    // for val in v.into_iter() { print!("{val} "); }
    // println!("→ into_iter（消耗 v）");
    // ↑ v 被消耗了

    // iter_mut()：可变借用 &mut T
    let mut v = vec![1, 2, 3];
    for val in v.iter_mut() { *val *= 10; }
    println!("iter_mut 后: {:?}", v);

    println!("\n===== 4. 消费适配器（consuming adaptors）=====\n");

    let v = vec![1, 2, 3, 4, 5];

    // sum：求和（消耗迭代器）
    let total: i32 = v.iter().sum();
    println!("sum: {}", total);

    // count：计数
    let count = v.iter().count();
    println!("count: {}", count);

    // collect：收集成集合（第 8 章学过）
    // iter() 产出 &i32，用 copied() 解引用为 i32
    let doubled: Vec<i32> = v.iter().copied().collect();
    println!("collect: {:?}", doubled);

    // any / all：是否存在/全部满足
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("any even: {}, all positive: {}", has_even, all_positive);

    // max / min
    println!("max: {:?}", v.iter().max());
    println!("min: {:?}", v.iter().min());

    println!("\n===== 5. 迭代器适配器（iterator adaptors）⭐ =====\n");

    let v = vec![1, 2, 3, 4, 5];

    // map：变换每个元素
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map *2: {:?}", doubled);

    // filter：过滤
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter even: {:?}", evens);

    // 链式调用（函数式风格）⭐
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)           // 只留偶数
        .map(|x| x * 10)                    // 乘以 10
        .collect();
    println!("链式 filter+map: {:?}", result);

    // enumerate：加上索引
    for (i, val) in v.iter().enumerate() {
        println!("  [{i}] = {val}");
    }

    // zip：拉链配对（第 8 章学过）
    let names = vec!["Alice", "Bob"];
    let ages = vec![25, 30];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zip: {:?}", pairs);

    // flat_map：展开（把嵌套拍平）
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.iter().flat_map(|v| v.iter()).copied().collect();
    println!("flat_map: {:?}", flat);

    // take / skip：取/跳过前 n 个
    let first3: Vec<&i32> = v.iter().take(3).collect();
    let skip2: Vec<&i32> = v.iter().skip(2).collect();
    println!("take 3: {:?}, skip 2: {:?}", first3, skip2);

    println!("\n===== 6. 实战：鞋子筛选（the book 经典例子）=====\n");

    let shoes = vec![
        Shoe { size: 40, style: String::from("运动鞋") },
        Shoe { size: 42, style: String::from("皮鞋") },
        Shoe { size: 41, style: String::from("帆布鞋") },
        Shoe { size: 43, style: String::from("靴子") },
    ];

    // 找出尺码 >= 42 的鞋子
    let big_shoes = shoes_in_size(&shoes, 42);
    println!("尺码 >= 42 的鞋子:");
    for shoe in big_shoes {
        println!("  {} (尺码 {})", shoe.style, shoe.size);
    }

    println!("\n===== 7. 创建自定义迭代器 =====\n");

    // 实现自己的迭代器（生成 1 到 5）
    let counter = Counter::new(5);
    for n in counter {
        print!("{n} ");
    }
    println!();

    // 自定义迭代器也能用所有迭代器方法
    let sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))   // (1,2)(2,3)(3,4)(4,5)
        .map(|(a, b)| a * b)             // 2,6,12,20
        .filter(|x| x % 2 == 0)          // 全是偶数
        .sum();
    println!("自定义迭代器链式求和: {}", sum);   // 40
}

// ========================================================
// 实战：筛选鞋子
// ========================================================
fn shoes_in_size(shoes: &[Shoe], min_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size >= min_size).collect()
}

// ========================================================
// 自定义迭代器：Counter
// ========================================================
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// ========================================================
// ⭐ 零成本抽象（zero-cost abstraction）
// ========================================================
// 迭代器是「零成本抽象」的典范：
//
// 这段代码：
//   let sum: i32 = vec.iter().filter(|x| *x > 0).map(|x| x * 2).sum();
//
// 编译后和手写循环一样快：
//   let mut sum = 0;
//   for x in &vec {
//       if *x > 0 { sum += x * 2; }
//   }
//
// 甚至可能更快（编译器能做更多优化）！
// 这就是 Rust「高级抽象 + 零运行时成本」的精髓
