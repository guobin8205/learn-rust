# 第 18 章：模式与匹配 (Patterns and Matching)

> 对应 the book 第 18 章
> 学习目标：系统掌握模式匹配的全部语法和实战应用

## 📂 示例代码
```bash
cd ch18-patterns/patterns
cargo run --bin positions    # 18.1 模式出现的位置
cargo run --bin syntax       # 18.2 模式语法全解
cargo run --bin advanced     # 18.3 实战应用
```

---

## 18.1 模式可以出现的位置 ⭐

**模式在 Rust 里无处不在**——几乎所有「绑定变量」的地方都可以用模式：

| 位置 | 例子 |
|------|------|
| `match` 分支 | `match x { 1 => ..., _ => ... }` |
| `if let` | `if let Some(v) = x { }` |
| `while let` | `while let Some(v) = x.pop() { }` |
| `for` 循环 | `for (i, v) in x.enumerate()` |
| `let` 语句 | `let (a, b) = tuple` |
| 函数参数 | `fn f((a, b): (i32, i32))` |
| 闭包参数 | `\|(a, b)\| ...` |

### ⭐ 两个重要概念：可反驳 vs 不可反驳

| 类型 | 含义 | 例子 | 能用在 |
|------|------|------|--------|
| **不可反驳** (irrefutable) | 总能匹配成功 | `let x = 5;` `let (a,b) = (1,2);` | 任何地方 |
| **可反驳** (refutable) | 可能匹配失败 | `Some(x)` `5` | if let / match |

```rust
let (a, b) = (1, 2);           // ✅ 不可反驳，可以用在 let
let Some(x) = some_option;     // ❌ 可反驳，编译错误！
if let Some(x) = some_option { } // ✅ 用 if let
```

---

## 18.2 模式语法全解 ⭐

### 语法速查表

| 模式 | 含义 | 例子 |
|------|------|------|
| 字面量 | 匹配具体值 | `1`、`true`、`'a'` |
| 变量 | 绑定到变量 | `n` |
| `a \| b` | 或（多模式） | `1 \| 2 \| 3` |
| `1..=5` | 范围匹配（含两端） | `'a'..='j'` |
| `Struct{x, y}` | 解构结构体 | `Point { x, y }` |
| `Variant(x)` | 解构枚举 | `Some(x)` |
| `&pattern` | 解构引用 | `&Point{x, y}` |
| `_` | 忽略整个值 | `let _ = 5;` |
| `_x` | 忽略但绑定（抑制警告） | `let _x = 5;` |
| `..` | 忽略剩余 | `Point { x, .. }` |
| `x if cond` | 守卫（额外条件） | `n if n > 0` |
| `n @ 1..=5` | 范围匹配 + 绑定 | `age @ 0..=12` |

### 关键语法详解

#### ① 多模式 `|`
```rust
match x {
    1 | 2 => println!("一或二"),
    3 | 4 | 5 => println!("三、四、五"),
    _ => println!("其他"),
}
```

#### ② 范围 `..=`
```rust
match x {
    1..=5 => println!("1 到 5"),     // 数字范围
    _ => println!("其他"),
}
match c {
    'a'..='j' => ...                 // 字符范围
}
```

#### ③ 解构结构体
```rust
let p = Point { x: 5, y: 10 };
let Point { x, y } = p;   // 简写（字段名 = 变量名）
let Point { x: a, y: b } = p;   // 重命名
```

#### ④ 解构枚举
```rust
match msg {
    Message::Quit => ...,
    Message::Move { x, y } => ...,    // 结构体变体
    Message::Write(text) => ...,      // 元组变体
}
```

#### ⑤ 忽略值
```rust
let (_, y, _) = (1, 2, 3);        // 只取 y
match p {
    Point { x, .. } => ...,        // 忽略剩余字段
}
match tuple {
    (first, .., last) => ...,      // 取头尾，忽略中间
}
```

#### ⑥ ⭐ match 守卫（guard）
```rust
match num {
    n if n % 2 == 0 => println!("偶数"),   // 加 if 条件
    n => println!("奇数"),
}
```
守卫可以解决「变量遮蔽」问题——在模式里比较外部变量必须用守卫。

#### ⑦ ⭐ @ 绑定（测试 + 绑定）
```rust
match age {
    n @ 0..=12 => println!("儿童 {}", n),   // 范围匹配 + 绑定 n
    n @ 13..=19 => println!("少年 {}", n),
    _ => ...,
}
```

---

## 18.3 模式实战应用

### 1. HTTP 错误处理（深度解构）
```rust
match fetch(url) {
    HttpResponse::Ok(data) => ...,
    HttpResponse::Error(ApiError::RateLimited { retry_after }) => ...,
    HttpResponse::Error(ApiError::Server(code)) => ...,
}
```

### 2. 解析嵌套 JSON
```rust
fn extract_name(json: &Json) -> Option<&str> {
    match json {
        Json::Object(map) => match map.get("name")? {
            Json::String(s) => Some(s),
            _ => None,
        },
        _ => None,
    }
}
```

### 3. 状态机
```rust
fn process(state: OrderState) -> OrderState {
    match state {
        OrderState::Created => OrderState::Paid { amount: 100 },
        OrderState::Paid { amount } => OrderState::Shipped { ... },
        OrderState::Shipped { tracking } => OrderState::Delivered,
        ...
    }
}
```

### 4. 解构 Option 链（避免嵌套 if）
```rust
// 繁琐：三层嵌套 if let
// 优雅：一个 match
match config {
    Some(Some(Some(env))) => println!("{}", env),
    _ => println!("配置缺失"),
}
```

### 5. 绑定 + 守卫组合
```rust
match command {
    ("delete", Some(name)) if name.starts_with("important") => {
        println!("不允许删除重要文件");
    }
    ("delete", Some(name)) => ...,
    ...
}
```

---

## 📋 模式匹配的设计哲学

| 传统语言（if/switch） | Rust 模式匹配 |
|---------------------|---------------|
| 先取值再判断 | **同时解构 + 判断 + 绑定** |
| 忘记某个 case 是 bug | **编译器强制穷尽** |
| 嵌套条件层层缩进 | **扁平化 match 分支** |
| 类型断言可能出错 | **类型安全，编译期检查** |

> ⭐ 模式匹配是 Rust 最强大的控制流工具。它让你能同时：
> 1. 测试值的结构（是哪个变体？）
> 2. 提取内部数据（绑定到变量）
> 3. 加额外条件（守卫）
> 4. 保证穷尽所有情况（编译器强制）

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 18 章 小结

学完本章你应该掌握：
1. ✅ 知道模式可以出现在 7 个位置
2. ✅ 区分可反驳和不可反驳模式
3. ✅ 掌握全部模式语法（字面量、范围、解构、_、..、守卫、@）
4. ✅ 用模式匹配优雅地处理复杂业务逻辑
5. ✅ 理解模式匹配比传统 switch 强在哪

---

## 📂 本章练习目录

- `patterns/src/positions.rs` —— 18.1 模式位置
- `patterns/src/syntax.rs` —— 18.2 语法全解
- `patterns/src/advanced.rs` —— 18.3 实战应用
