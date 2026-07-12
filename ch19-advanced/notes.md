# 第 19 章：高级特性 (Advanced Features)

> 对应 the book 第 19 章
> 学习目标：掌握 unsafe、高级 trait、newtype、宏等进阶工具

## 📂 示例代码
```bash
cd ch19-advanced/advanced
cargo run --bin unsafe_t         # 19.1 Unsafe Rust
cargo run --bin advanced_traits  # 19.2 高级 Trait
cargo run --bin advanced_types   # 19.3 高级类型 + 宏
```

---

## 19.1 Unsafe Rust

### Unsafe 的「五扇门」

```rust
unsafe {
    // 1. 解引用裸指针
    // 2. 调用 unsafe 函数
    // 3. 实现或访问 unsafe trait
    // 4. 访问/修改 static mut
    // 5. 访问 union 字段
}
```

> ⚠️ **unsafe 不关闭借用检查器！** 只是打开上述五扇门，其他安全规则照常生效。

### 裸指针

```rust
let mut num = 5;
let r1 = &num as *const i32;      // 不可变裸指针
let r2 = &mut num as *mut i32;    // 可变裸指针

unsafe { *r2 = 10; }              // 只能在 unsafe 里解引用
```

### ⭐ 用 unsafe 实现安全 API（经典模式）

```rust
// split_at_mut 内部用 unsafe，但对外是安全 API
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let ptr = slice.as_mut_ptr();
    unsafe {
        (from_raw_parts_mut(ptr, mid),
         from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
```

> 这是标准库大量使用 unsafe 的方式：**底层 unsafe，对外安全**。

### FFI（调用 C 函数）

```rust
unsafe extern "C" {           // Rust 2024 要求 unsafe extern
    fn abs(input: i32) -> i32;
}
unsafe { abs(-5); }
```

### 使用原则

1. 尽量不用 unsafe（99% 代码不需要）
2. 需要时，隔离在小范围 + 封装成安全 API
3. 加注释说明「为什么安全」
4. 充分测试

---

## 19.2 高级 Trait

### ⭐ 关联类型（Associated Types）

```rust
trait MyIterator {
    type Item;                    // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

impl MyIterator for Counter {
    type Item = u32;              // 实现时指定
    fn next(&mut self) -> Option<u32> { ... }
}
```

#### 关联类型 vs 泛型参数
| | 泛型 trait | 关联类型 trait |
|---|----------|--------------|
| 一个类型能多次实现？ | ✅ | ❌ 只能一次 |
| 使用时需指定类型？ | ✅ Iterator<Item=T> | ❌ 自动推断 |
| 代表 | trait 的「参数」 | 实现 trait 的「输出」 |

### 运算符重载

```rust
use std::ops::Add;

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point { ... }
}
// 现在 Point 可以用 + 运算符
```

### 完全限定语法（消除歧义）

```rust
trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }
impl Human { fn fly(&self); }

person.fly();                // 默认调用原方法
Pilot::fly(&person);         // 指定 Pilot trait
Wizard::fly(&person);        // 指定 Wizard trait
```

### Supertrait

```rust
trait Greet: fmt::Display {       // Greet 依赖 Display
    fn greet(&self) {
        println!("Hello, {}!", self);   // 可以用 Display 方法
    }
}
```

### ⭐ newtype 模式

```rust
struct Meters(u32);            // ⭐ 和 u32 是不同类型
struct Kilometers(u32);        // 类型安全！
```

**两个用途**：
1. **类型安全**：Meters 和 Kilometers 不会混淆
2. **绕过孤儿规则**：为外部类型实现外部 trait

```rust
// 孤儿规则：Vec 和 Display 都不是我定义的，不能直接 impl
// 解决：newtype 包装
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper { ... }   // ✅ Wrapper 是我的类型
```

---

## 19.3 高级类型与函数

### 类型别名（type）

```rust
type Kilometers = i32;          // ⚠️ Kilometers 和 i32 是同一类型（不像 newtype）
type Thunk = Box<dyn Fn() + Send + 'static>;   // 简化复杂类型
```

> ⚠️ type 别名**不提供类型安全**（和 newtype 不同）。要类型安全用 newtype。

### Never 类型 `!`

```rust
fn bar() -> ! {                 // 永不返回
    panic!("...");
}
// ! 可以强转为任何类型
match opt {
    Some(n) => n,
    None => panic!("None"),     // panic! 返回 !，可以当 i32
}
```

### 函数指针（fn 类型）

```rust
fn add(x: i32, y: i32) -> i32 { x + y }

fn do_math(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}
do_math(add, 2, 3);
```

| | fn 指针 | 闭包 |
|---|--------|------|
| 捕获环境 | ❌ | ✅ |
| 大小 | 固定 | 不固定 |
| Copy | ✅ | 取决于捕获 |

### 返回闭包

```rust
// 方式 1：Box<dyn Fn>
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 方式 2：impl Fn（更高效）
fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

### 宏（macro_rules!）

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    };
}

let v = my_vec![1, 2, 3];   // 编译期展开生成代码
```

#### 宏 vs 函数
| | 宏 | 函数 |
|---|----|----|
| 何时展开 | 编译期 | 运行期 |
| 参数数量 | 可变 | 固定 |
| 参数类型 | 任意（模式匹配） | 固定 |

---

## ⭐ newtype vs type 别名（重要区分）

| | newtype `struct Meters(u32)` | type 别名 `type Meters = i32` |
|---|--------------------------|---------------------------|
| 是否新类型 | ✅ 是 | ❌ 不是（还是 i32） |
| 类型安全 | ✅ Meters ≠ i32 | ❌ Meters == i32 |
| 运行时开销 | 无（零成本） | 无 |
| 用途 | 类型安全 | 简化复杂类型 |

```rust
struct Meters(u32);           // newtype
type MetersAlias = u32;       // type 别名

let m = Meters(5);
// let n: u32 = m;            // ❌ newtype 类型不同
let a: MetersAlias = 5;
let b: u32 = a;               // ✅ 别名就是 u32
```

---

## 📋 高级特性速查

| 特性 | 何时用 |
|------|--------|
| `unsafe` | 调用 C / 底层操作 / 裸指针 |
| 关联类型 | trait 的「输出类型」 |
| 运算符重载 | 实现 + - * / 等 |
| newtype | 类型安全 + 绕过孤儿规则 |
| type 别名 | 简化复杂类型 |
| 函数指针 fn | 不需要捕获的回调 |
| 宏 | 编译期代码生成 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

（暂无提问）

---

## ✅ 第 19 章 小结

学完本章你应该掌握：
1. ✅ 知道 unsafe 的五扇门和使用原则
2. ✅ 用关联类型定义 trait
3. ✅ 运算符重载、完全限定语法
4. ✅ 用 newtype 模式获得类型安全
5. ✅ 知道 newtype vs type 别名的区别
6. ✅ 了解宏的基本语法

---

## 📂 本章练习目录

- `advanced/src/unsafe_t.rs` —— 19.1 Unsafe Rust
- `advanced/src/advanced_traits.rs` —— 19.2 高级 Trait
- `advanced/src/advanced_types.rs` —— 19.3 高级类型 + 宏
