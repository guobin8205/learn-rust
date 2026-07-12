# 第 17 章：Rust 的面向对象特性

> 对应 the book 第 17 章
> 学习目标：掌握 trait 对象（dyn Trait）、动态分发、对象安全、OOP 模式

## 📂 示例代码
```bash
cd ch17-oo-features/oo_features
cargo run --bin trait_objects    # 17.1 trait 对象
cargo run --bin object_safety    # 17.2 对象安全
cargo run --bin oo_patterns      # 17.3 OOP 模式
```

---

## 17.1 Trait 对象（dyn Trait）⭐

### 解决的问题

如何把「不同类型」的实例放进同一个集合？

```rust
// GUI 库：Button、TextBox、Image 不同类型，都要 draw
trait Draw { fn draw(&self); }

// ⭐ Box<dyn Draw> = trait 对象，存储不同类型
struct Screen {
    components: Vec<Box<dyn Draw>>,
}
```

### 用法

```rust
let mut screen = Screen::new();
screen.add(Box::new(Button { label: String::from("OK") }));
screen.add(Box::new(TextBox { text: String::from("hi") }));
screen.add(Box::new(Image { filename: String::from("logo.png") }));
// ✅ 不同类型放进同一个 Vec！
```

### 静态分发 vs 动态分发 ⭐

| | 静态分发（泛型） | 动态分发（dyn Trait） |
|---|---------------|---------------------|
| 何时确定方法 | **编译期**（单态化） | **运行期**（虚表 vtable） |
| 性能 | 最快（零成本） | 稍慢（虚表查找） |
| 能否存混合类型 | ❌（Vec<T> 只能一种） | ✅（Vec<Box<dyn T>>） |
| 二进制大小 | 更大 | 更小 |

```rust
// 静态分发（泛型 + trait bound）
fn static_draw<T: Draw>(item: &T) { item.draw(); }   // 编译期生成专用代码

// 动态分发（trait 对象）
fn dynamic_draw(item: &dyn Draw) { item.draw(); }     // 运行时查 vtable
```

### &dyn Trait vs Box<dyn Trait>

```rust
// &dyn Trait：借用，不拿所有权
let refs: Vec<&dyn Draw> = vec![&btn, &txt];

// Box<dyn Trait>：拥有，堆上分配
let owned: Vec<Box<dyn Draw>> = vec![Box::new(btn), Box::new(txt)];
```

### 何时用 trait 对象？

| 场景 | 用什么 |
|------|--------|
| 存不同类型的集合（GUI 组件、插件） | **dyn Trait** ✅ |
| 类型运行时才知道 | dyn Trait ✅ |
| 类型固定且少 | 泛型（更快） |
| 性能敏感 | 泛型 |

---

## 17.2 对象安全（Object Safety）⭐

### 不是所有 trait 都能做 dyn Trait

trait 要「对象安全」，必须满足两条规则：

1. **方法不能返回 `Self` 类型**
2. **方法不能有泛型类型参数**

```rust
// ❌ 不安全：返回 Self
trait Clone { fn clone(&self) -> Self; }
// dyn Clone 不知道具体类型，无法返回 Self

// ❌ 不安全：有泛型参数
trait Foo { fn f<T>(&self, x: T); }
// 泛型会单态化，vtable 装不下无限方法

// ✅ 安全
trait Draw { fn draw(&self); }
```

### 标准库中的例子

| trait | 对象安全？ | 原因 |
|-------|----------|------|
| Display, Debug, Write | ✅ | 无 Self 返回，无泛型 |
| Iterator | ❌ | next() 返回 Self::Item |
| Clone | ❌ | clone() 返回 Self |
| Default | ❌ | default() 返回 Self |

### 遇到不安全怎么办？

1. 重构 trait，去掉不安全的方法
2. 用泛型代替 trait 对象
3. 把不安全的方法拆到另一个 trait

---

## 17.3 面向对象模式

### Rust 是不是 OOP 语言？

| OOP 特性 | Rust 支持？ | 实现方式 |
|---------|----------|---------|
| 封装 | ✅ | pub / 私有字段方法 |
| 继承 | ❌ | 用 trait + 组合代替 |
| 多态 | ✅ | trait + 泛型/dyn |
| 抽象 | ✅ | trait 定义接口 |

### 封装

```rust
pub struct AveragedCollection {
    list: Vec<i32>,      // 私有
    average: f64,        // 私有
}
impl AveragedCollection {
    pub fn add(&mut self, v: i32) { ... }     // 公开方法
    fn update_average(&mut self) { ... }      // 私有方法
}
```

### Rust 没有继承，用什么代替？

| OOP 继承解决 | Rust 方案 |
|-------------|----------|
| 代码复用 | 组合 + trait 默认方法 |
| 多态 | trait + 泛型/dyn |
| 类型系统 | trait bound |

### 为什么不用继承？

- 耦合重（子类依赖父类实现）
- 违背「组合优于继承」原则
- Rust 用 trait 定义接口 + 组合实现，更灵活

---

## 📋 trait 对象速查表

| 需求 | 用什么 |
|------|--------|
| 存不同类型到集合 | `Vec<Box<dyn Trait>>` |
| 函数参数接收任何实现某 trait 的类型 | `&dyn Trait` 或 `impl Trait` |
| 需要零成本 | 泛型 `<T: Trait>` |
| 需要 trait 对象 | trait 必须对象安全 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 17 章 小结

学完本章你应该掌握：
1. ✅ 用 `Box<dyn Trait>` / `&dyn Trait` 实现 trait 对象
2. ✅ 区分静态分发（泛型）和动态分发（dyn）
3. ✅ 理解对象安全的两条规则
4. ✅ 知道 Rust 没有继承，用 trait + 组合代替
5. ✅ 理解封装、多态在 Rust 中的实现

---

## 📂 本章练习目录

- `oo_features/src/trait_objects.rs` —— 17.1 trait 对象
- `oo_features/src/object_safety.rs` —— 17.2 对象安全
- `oo_features/src/oo_patterns.rs` —— 17.3 OOP 模式
