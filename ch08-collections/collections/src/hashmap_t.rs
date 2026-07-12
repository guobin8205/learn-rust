// ===== 8.3 HashMap<K, V> 键值对存储 =====
// 用 cargo run --bin hashmap_t 运行

use std::collections::HashMap;

fn main() {
    // ========================================================
    // 一、创建 HashMap
    // ========================================================

    // 方式 1：HashMap::new()
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("scores = {:?}", scores);

    // 方式 2：从 Vec 元组创建（collect）
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从 Vec 创建: {:?}", scores2);

    // ⚠️ HashMap 使用频率比 Vec/String 低，所以没有 prelude 自动引入
    //    必须手动 use std::collections::HashMap;

    // ========================================================
    // 二、读取（get 返回 Option）
    // ========================================================
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team = String::from("Blue");
    // get 返回 Option<&V>（因为键可能不存在）
    match scores.get(&team) {
        Some(score) => println!("Blue 分数: {score}"),
        None => println!("没有 Blue 队"),
    }
    // 简写：unwrap_or 提供默认值
    let score = scores.get(&team).unwrap_or(&0);
    println!("Blue 分数（unwrap_or）: {score}");

    // 不存在的键
    let none = scores.get("Green");
    println!("Green 分数: {:?}", none);   // None

    // ========================================================
    // 三、所有权 ⭐
    // ========================================================
    // insert 会拿走键值的所有权（对于 String 等堆类型）
    let key = String::from("favorite");
    let value = String::from("red");
    let mut map = HashMap::new();
    map.insert(key, value);
    // println!("{key}");    // ❌ key 已被 move 进 HashMap
    // println!("{value}");  // ❌ value 已被 move

    // Copy 类型（如 i32）不移动，会拷贝
    let mut map2: HashMap<i32, i32> = HashMap::new();
    let n = 1;
    map2.insert(n, 100);
    println!("{n}");   // ✅ i32 是 Copy，n 仍可用

    // ========================================================
    // 四、遍历
    // ========================================================
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Green"), 30);

    // 遍历 &K, &V（注意：HashMap 无序！）
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ========================================================
    // 五、更新值的三种模式 ⭐
    // ========================================================

    // ---------- 模式 1：insert 覆盖 ----------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);   // 覆盖了 10
    println!("覆盖后: {:?}", scores);            // Blue: 25

    // ---------- 模式 2：只在键不存在时插入（entry.or_insert）⭐ ----------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);   // Yellow 不存在 → 插入
    scores.entry(String::from("Blue")).or_insert(50);     // Blue 已存在 → 不插入
    println!("entry.or_insert: {:?}", scores);             // Blue:10, Yellow:50

    // 💡 entry 返回 Entry 枚举，or_insert 「有就用旧的，没有就插入新的」

    // ---------- 模式 3：基于旧值更新 ----------
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);   // 返回 &mut V
        *count += 1;                                         // 解引用修改
    }
    println!("词频统计: {:?}", word_count);
    // {"hello":1, "world":2, "wonderful":1}

    // ========================================================
    // 六、删除、判断
    // ========================================================
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    map.remove("a");                          // 删除键
    println!("删除后: {:?}", map);

    println!("包含 b: {}", map.contains_key("b"));
    println!("长度: {}", map.len());

    // ========================================================
    // ⭐ HashMap 速查表
    // ========================================================
    // | 操作              | 方法                          |
    // |-------------------|------------------------------|
    // | 创建              | HashMap::new()                |
    // | 插入/覆盖          | map.insert(k, v)              |
    // | 查找（安全）       | map.get(&k) → Option<&V>      |
    // | 有就用没有就插     | map.entry(k).or_insert(v) ⭐   |
    // | 删除              | map.remove(&k)                |
    // | 是否包含键         | map.contains_key(&k)          |
    // | 镍长度            | map.len()                     |
    // | 遍历              | for (k, v) in &map            |
}
