# 第 8 章：常见集合 (Common Collections)

> 对应 the book 第 8 章
> 学习目标：掌握 Vec、String、HashMap 三大常用集合

## 📂 示例代码
```bash
cd ch08-collections/collections
cargo run --bin vec_t       # 8.1 Vec<T>
cargo run --bin string_t    # 8.2 String
cargo run --bin hashmap_t   # 8.3 HashMap
```

---

## 8.1 Vec\<T\> 动态数组

### 创建
```rust
let v1: Vec<i32> = Vec::new();       // 空数组（需标类型）
let v2 = vec![1, 2, 3];              // 带初值（最常用）
let v3 = vec![0; 5];                 // 5 个 0
```

### 添加/删除
```rust
let mut v = Vec::new();
v.push(1);                  // 追加元素
v.push(2);
let last = v.pop();         // 弹出末尾，返回 Option<T>
```

### 读取（两种方式 ⭐）

| 方式 | 越界行为 | 用法 |
|------|---------|------|
| `&v[i]` | **panic** 💥 | 确定安全时 |
| `v.get(i)` | 返回 `None` ✅ | 用户输入/不确定时 |

```rust
let v = vec![10, 20, 30];
let third = &v[2];              // 30
let safe = v.get(100);          // None（不 panic）
```

### 遍历
```rust
for x in &v { ... }            // 只读遍历
for x in &mut v { *x += 1; }   // 可变遍历（解引用修改）
for x in v { ... }             // 消耗遍历（拿走所有权）
```

### 借用规则（第 4 章的应用）
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
println!("{first}");    // ← first 最后一次使用
v.push(6);              // ✅ 此时可变借用（NLL）
```
> ⚠️ 为什么 push 可能冲突？因为 Vec 可能扩容搬移内存，旧引用会悬垂。

### 存不同类型的数据：用 enum
```rust
enum Cell { Int(i32), Float(f64), Text(String) }
let row = vec![Cell::Int(3), Cell::Text(String::from("hi"))];
```

### Vec 常用方法速查
| 操作 | 方法 |
|------|------|
| 创建 | `Vec::new()` / `vec![]` |
| 添加 | `v.push(val)` |
| 弹出 | `v.pop() → Option<T>` |
| 读取（panic） | `&v[i]` |
| 读取（安全） | `v.get(i) → Option<&T>` |
| 长度 | `v.len()` |
| 是否空 | `v.is_empty()` |
| 排序 | `v.sort()` |
| 反转 | `v.reverse()` |
| 包含 | `v.contains(&val)` |

---

## 8.2 String 字符串

### String vs &str

| 类型 | 所有权 | 可变 | 内存 |
|------|--------|------|------|
| `String` | 有 | 可变 | 堆，可增长 |
| `&str` | 无 | 只读 | 指向某处字符串 |

### 创建 String
```rust
let s1 = String::new();                 // 空
let s2 = String::from("hello");         // 从字面量
let s3 = "hello".to_string();           // 字面量转 String
```

### 更新
```rust
let mut s = String::from("foo");
s.push_str("bar");      // 追加 &str
s.push('!');            // 追加 char（单引号）

// + 拼接（消耗左侧）
let s3 = s1 + &s2;      // s1 被移动，s2 保留

// format! 拼接（不消耗参数，最灵活）⭐
let s = format!("{s1}-{s2}-{s3}");
```

### ⭐ 为什么 String 不能用索引访问？

**因为 String 内部是 `Vec<u8>`（UTF-8 字节），字节 ≠ 字符：**

```rust
let s = String::from("你好");
s.len();    // 6！不是 2（每个汉字 3 字节 UTF-8）
// s[0]     // ❌ 编译错误！字节 0 不是合法字符
```

| 字符串 | 长度（字节） | 说明 |
|--------|-----------|------|
| `"Hola"` | 4 | 每个字母 1 字节 |
| `"你好"` | 6 | 每个汉字 3 字节 |
| `"Здравствуйте"` | 24 | 每个西里尔字母 2 字节 |

### 正确的访问方式（三种视图）

```rust
let s = String::from("你好");

// ① bytes()：按字节
s.bytes()

// ② chars()：按字符（最常用）
for c in s.chars() { print!("{c} "); }

// ③ char_indices()：字符 + 字节索引
for (i, c) in s.char_indices() { ... }
```

### 切片（小心 UTF-8 边界）
```rust
let s = "Здравствуйте";
let part = &s[0..4];    // ✅ 在字符边界（前 2 个字母 4 字节）
// let bad = &s[0..1];  // 💥 panic！字节 1 不是字符边界
```

### String vs &str 选择指南

| 场景 | 用什么 |
|------|--------|
| 需要拥有、修改 | String |
| 只读 | &str |
| 函数参数 | **&str** ⭐（更通用，&String 能自动转） |
| 字符串字面量 | &str |
| 结构体字段 | String |

---

## 8.3 HashMap\<K, V\> 键值对

> ⚠️ HashMap 不在 prelude，需要 `use std::collections::HashMap;`

### 创建
```rust
use std::collections::HashMap;

let mut map: HashMap<String, i32> = HashMap::new();
map.insert(String::from("Blue"), 10);
```

### 读取（get 返回 Option）
```rust
let score = map.get(&team);          // Option<&V>
let score = map.get(&team).unwrap_or(&0);   // 带默认值
```

### 所有权（⭐）
```rust
let key = String::from("k");
let val = String::from("v");
let mut map = HashMap::new();
map.insert(key, val);
// println!("{key}");   // ❌ String 被 move 进 HashMap
// i32 等 Copy 类型不会移动
```

### 更新值的三种模式 ⭐

**模式 1：insert 覆盖**
```rust
map.insert("Blue", 10);
map.insert("Blue", 25);   // 覆盖
```

**模式 2：entry.or_insert（有就不插，没有才插）⭐**
```rust
map.entry(String::from("Yellow")).or_insert(50);
```

**模式 3：基于旧值更新（词频统计经典例子）**
```rust
let text = "hello world world";
let mut count = HashMap::new();
for word in text.split_whitespace() {
    let c = count.entry(word).or_insert(0);   // 返回 &mut V
    *c += 1;
}
// {"hello":1, "world":2}
```

### 常用方法
| 操作 | 方法 |
|------|------|
| 插入/覆盖 | `map.insert(k, v)` |
| 查找 | `map.get(&k) → Option<&V>` |
| 有就插入 | `map.entry(k).or_insert(v)` ⭐ |
| 删除 | `map.remove(&k)` |
| 包含键 | `map.contains_key(&k)` |
| 长度 | `map.len()` |
| 遍历 | `for (k, v) in &map`（**无序**） |

---

## 📋 三大集合对比

| 集合 | 用途 | 访问方式 | 有序？ |
|------|------|---------|--------|
| `Vec<T>` | 列表/栈 | 索引 | ✅ 按插入顺序 |
| `String` | UTF-8 字符串 | chars/bytes | ✅ |
| `HashMap<K,V>` | 键值映射 | 键查找 | ❌ 无序 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：`teams.into_iter().zip(initial_scores.into_iter()).collect()` 怎么理解？`zip` 是什么？

**A：** 这是一条**迭代器链**，数据像流水线一步步处理。

#### 流程图
```
teams / initial_scores    →  .into_iter()     →  .zip()           →  .collect()
["Blue","Yellow"]            迭代器              拉链配对             HashMap
[10, 50]                     Blue→Yellow        (Blue,10)→(Yellow,50) {Blue:10, Yellow:50}
                             10→50
```

#### 三个方法逐个理解

**① `into_iter()`：把集合变成迭代器**
```rust
teams.into_iter()   // 产出：Blue → Yellow（消耗 teams）
```
类比：把箱子倒空，东西一个个拿出来。

**② `zip()`：拉链式配对 ⭐**
来自「拉链 (zipper)」比喻——两排齿左右一一咬合：
```
左边：Blue  Yellow      右边：10  50
         ↕    ↕                ↕    ↕
      (Blue,10)            (Yellow,50)
```
关键特性：**长度对齐，以短的为准**
```
zip([1,2,3,4,5], ["a","b","c"]) → [(1,"a"),(2,"b"),(3,"c")]  // 4,5 丢弃
```

**③ `collect()`：万能收集器**
根据声明的目标类型决定收集成什么：
```rust
let v: Vec<_> = iter.collect();              // 收集成 Vec
let m: HashMap<_, _> = tuple_iter.collect(); // 收集成 HashMap
```

#### `HashMap<_, _>` 的含义
`_` = 让编译器推断类型。从元组 `(String, i32)` 推断出 `HashMap<String, i32>`，省得写全。

#### 用循环对比（更直观）
```rust
// 迭代器链写法（简洁）
let m: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

// 循环写法（等价）
let mut m: HashMap<String, i32> = HashMap::new();
for (team, score) in teams.iter().zip(scores.iter()) {
    m.insert(team.clone(), *score);
}
```

#### 类比理解
- `zip` = 把两列数据**并排对齐**（Alice↔25, Bob↔30）
- `collect` = 把对齐结果**抄到表格里**（HashMap）

> 💡 `into_iter`/`zip`/`collect`/`map` 都属于 Rust **迭代器**，第 13 章系统讲。特点：链式调用、零成本抽象、函数式风格。

---

## ✅ 第 8 章 小结

学完本章你应该掌握：
1. ✅ Vec 的创建、增删、读取（索引 vs get）、遍历
2. ✅ String 的本质（UTF-8 字节），为什么不能索引访问
3. ✅ HashMap 的 entry API（or_insert 模式）
4. ✅ 理解集合与所有权、借用的关系

---

## 📂 本章练习目录

- `collections/src/vec_t.rs` —— 8.1 Vec<T>
- `collections/src/string_t.rs` —— 8.2 String
- `collections/src/hashmap_t.rs` —— 8.3 HashMap
