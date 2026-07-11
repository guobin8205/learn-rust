# 第 3 章：常见编程概念 (Common Programming Concepts)

> 对应 the book 第 3 章
> 学习目标：系统掌握变量、数据类型、函数、控制流——Rust 的基础语法

## 📂 示例代码
本章有 4 个独立可运行的示例，在 `concepts/` 目录下：
```bash
cd ch03-concepts/concepts
cargo run --bin variables      # 3.1 变量
cargo run --bin data_types     # 3.2 数据类型
cargo run --bin functions      # 3.3 函数
cargo run --bin control_flow   # 3.4 控制流
```

---

## 3.1 变量与可变性

### 三种「变量」对比（核心！）

| 特性 | `let` | `let mut` | `const` |
|------|-------|-----------|---------|
| 可变性 | 不可变 | **可变** | 不可变 |
| 类型可改 | ✅（shadowing） | ✅（shadowing） | ❌ |
| 类型推断 | ✅ | ✅ | ❌ 必须标注 |
| 设置时机 | 运行时 | 运行时 | **编译时** |
| 作用域 | 任何 | 任何 | **全局可用** |

### 代码示例
```rust
// const 常量：全大写+下划线，必须标类型，编译时确定
const MAX_POINTS: u32 = 100_000;

let x = 5;          // 不可变
let mut y = 5;
y = 6;              // ✅ mut 可修改

// shadowing：用 let 复用名字，可以改变类型！
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize —— 这是 shadowing 的杀手锏
```

### shadowing vs mut 的区别（⭐ 重点）
- `mut`：修改**同一个**变量的值，**类型不能变**
- shadowing：创建**新**变量，**类型可以变**，旧变量被名字遮蔽（但未释放，等作用域结束 drop）

```rust
let mut count = 5;
count = "five";  // ❌ 类型错误

let count = 5;
let count = "five";  // ✅ shadowing 可以改类型
```

### 整数字面量的多种写法
```rust
let decimal = 98_222;      // 十进制，下划线分隔
let hex     = 0xff;        // 十六进制
let octal   = 0o77;        // 八进制
let binary  = 0b1111_0000; // 二进制
let byte    = b'A';        // 字节 u8，= ASCII 码 65
```

---

## 3.2 数据类型

Rust 是**静态强类型**语言：编译时确定类型，且不会隐式转换。

### 一、标量类型（Scalar，单个值）

| 类型 | 种类 | 说明 |
|------|------|------|
| 整数 | `i8`~`i128` `isize`（有符号）<br>`u8`~`u128` `usize`（无符号） | 默认 `i32`。`isize/usize` 等于机器字长 |
| 浮点 | `f32` `f64` | 默认 `f64`（双精度） |
| 布尔 | `bool` | `true` / `false`，占 1 字节 |
| 字符 | `char` | **4 字节** Unicode 标量值，单引号 |

> ⚠️ `char`（单引号，4字节 Unicode）≠ `String`/`&str`（双引号，UTF-8 字节序列）

> ⚠️ **整数溢出**：debug 模式 panic；release 模式环绕（wrap around）

### 整数类型速查表
| 长度 | 有符号 | 无符号 | 范围（以 8 位为例） |
|------|--------|--------|-------------------|
| 8-bit | `i8` | `u8` | -128~127 / 0~255 |
| 32-bit | `i32`（默认） | `u32` | 约 ±21 亿 / 0~42 亿 |
| 64-bit | `i64` | `u64` | 非常大 |
| arch | `isize` | `usize` | 64 位机器=64 位，常用于索引 |

### 二、复合类型（Compound，多个值）

| 类型 | 长度 | 元素类型 | 内存 |
|------|------|---------|------|
| **元组 Tuple** | 固定 | **可不同** | 栈 |
| **数组 Array** | 固定 | **必须相同** | 栈 |
| Vec（第 8 章） | 可变 | 必须相同 | 堆 |

#### 元组 Tuple
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 解构
let (x, y, z) = tup;

// 点访问
println!("{}", tup.0);  // 500

// 单元类型 ()：空元组，表示「无有意义的值」
let unit: () = ();
```

#### 数组 Array
```rust
let arr = [1, 2, 3, 4, 5];       // [i32; 5]
let zeros = [0; 5];              // [0, 0, 0, 0, 0]，初始化语法 [值; 长度]
println!("{}", arr[0]);          // 索引访问，从 0 开始
// arr[10]  // ❌ 越界：常量索引在编译期报错，变量索引在运行时 panic（见 Q&A Q1）
```

> 💡 数组**固定长度**，长度是类型的一部分（`[i32; 5]` 和 `[i32; 6]` 是不同类型）。需要可变长度用 `Vec`（第 8 章）。

---

## 3.3 函数

### 函数定义
```rust
// fn 函数名(参数: 类型, ...) -> 返回类型
fn add(a: i32, b: i32) -> i32 {
    a + b     // ← 没有分号！这是表达式，作为返回值
}
```

### 🔑 核心：语句 (Statement) vs 表达式 (Expression) ⭐⭐⭐

**这是 Rust 最关键的语法概念之一，必须吃透！**

| | 语句 Statement | 表达式 Expression |
|---|---------------|------------------|
| 是否返回值 | ❌ 不返回 | ✅ 返回值 |
| 分号 | 以分号结尾 | **不能**加分号 |
| 例子 | `let x = 5;` | `5`、`x + 1`、`if c {1} else {2}` |

**关键规则：加分号 → 变成语句 → 返回 `()`**

```rust
// 块 {} 也是表达式，其值是最后一个表达式的值
let y = {
    let z = 3;
    z + 1        // ← 无分号！是表达式，块的值是 4
};               // y = 4

// 对比：加了分号就变成语句，块的值变成 ()
let y = {
    let z = 3;
    z + 1;       // ← 加了分号！变成语句
};               // y = ()
```

**控制流也是表达式**：
```rust
let n = if condition { 5 } else { 6 };   // if 是表达式
```

### 返回值规则
- 函数最后一个**表达式**（无分号）就是返回值
- 提前返回用 `return` 关键字（通常只在提前退出时用）

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;   // 提前返回，用 return
    }
    b               // 默认返回最后一个表达式，不用 return
}
```

---

## 3.4 控制流

### 一、if 表达式
```rust
if number % 4 == 0 {
    println!("能被 4 整除");
} else if number % 3 == 0 {
    println!("能被 3 整除");
} else {
    println!("都不是");
}

// if 是表达式，可用于赋值
let n = if condition { 5 } else { 6 };
```
> ⚠️ 条件必须是 `bool`，不会隐式转换（不像 C/JS）
> ⚠️ 各分支类型必须相同

### 二、三种循环

#### `loop`：无限循环，靠 break 退出
```rust
loop {
    // ...
    if cond { break; }
}

// ⭐ loop 可以返回值！break 带值
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // 整个 loop 的返回值
    }
};  // result = 20
```

#### `while`：条件循环
```rust
let mut n = 3;
while n != 0 {
    println!("{n}!");
    n -= 1;
}
```

#### `for`：遍历集合（最推荐！）
```rust
let arr = [10, 20, 30];
for e in arr { println!("{e}"); }   // 遍历数组

for i in 0..5  { /* 0,1,2,3,4 */ } // 半开范围
for i in 1..=3 { /* 1,2,3 */ }      // 包含范围
for i in (1..4).rev() { /* 3,2,1 */ } // 反向
```

### 三种循环怎么选

| 循环 | 适用场景 |
|------|---------|
| `loop` | 无限循环、需要 break **返回值** |
| `while` | 基于条件的循环 |
| `for` | 遍历集合/范围 ⭐ **最安全最常用** |

> 💡 Rust 中 `for` 比「while + 索引」更安全高效（不会越界 panic）

---

## 📋 本章概念速查表

| 概念 | 要点 |
|------|------|
| `let` | 不可变变量（默认） |
| `let mut` | 可变变量 |
| `const` | 常量，必须标类型，全局可用 |
| shadowing | `let` 复用名字，可改类型 |
| `i32`/`f64` | 默认整数/浮点类型 |
| Tuple `(T1,T2)` | 定长，类型可不同 |
| Array `[T; n]` | 定长，类型相同 |
| 语句 vs 表达式 | 分号决定！加分号=语句=返回 `()` |
| `loop`+`break 值` | loop 可以返回值 |
| `if`/`for` 是表达式 | 可用于赋值 |

---

## 📝 提问与解答（Q&A）

> 这一节会随着学习过程中的提问持续更新。

### Q1：数组越界到底何时报错？笔记说运行时 panic，但我试出来是编译期报错

**A：** 好观察！两种情况都对，取决于**编译器能否在编译期推断出索引值**。

#### 情况 1：索引是编译期常量 → 编译期就报错 ✅

```rust
let arr = [1, 2, 3, 4, 5];
let x = arr[10];   // ❌ 编译错误！
```

编译器报错：
```
error: this operation will panic at runtime
 index out of bounds: the length is 5 but the index is 10
 = note: #[deny(unconditional_panic)] on default
```

**原因**：数组长度 5 和索引 10 都是编译期常量，编译器能算出来 `10 >= 5`，一定会 panic。这个 lint 叫 `unconditional_panic`（无条件 panic），默认拒绝编译。

#### 情况 2：索引要运行时才知道 → 运行时 panic

```rust
let index: usize = 从用户输入读取;
let x = arr[index];  // ✅ 能编译，运行时若越界才 panic
```

**原因**：索引值要等程序运行才知道，编译器无法预判。只有真正运行到越界那行才 panic：
```
thread 'main' panicked at ...
index out of bounds: the len is 5 but the index is 10
```

#### 对比总结

| 情况 | 例子 | 何时报错 | 原因 |
|------|------|---------|------|
| 索引是编译期常量 | `arr[10]` | **编译期** ❌ | `unconditional_panic` lint 检测到 |
| 索引是运行时变量 | `arr[用户输入]` | **运行时** 💥 | 编译器无法预判 |

> 💡 **更正**：笔记原先笼统说「数组越界运行时 panic」不够准确。准确说法是——编译器会尽可能在编译期帮你发现问题（常量索引时），但依赖运行时值的越界只能在运行时 panic。
>
> 💡 **延伸**：整数溢出也有类似的「debug vs release」区分（debug 模式 panic，release 模式环绕），但那针对的是算术运算，不是索引越界。两者不要混淆。

---

### Q2：`{ let y = 1; y + 1; }` 如果块后面带分号呢？

**A：** 要区分**两层分号**——「块内部的分号」和「块外部的分号」。

#### 核心原则
块 `{}` 是一个**表达式**，它有「值」：
- 块的值 = 块内最后一个**无分号表达式**的值
- 如果块内最后一条带分号（或块为空），块的值是 `()`

#### 四种组合（实验验证）

| # | 代码 | 块的值 | 说明 |
|---|------|-------|------|
| ① | `let a = { y+1 };` | `2` (i32) | 块内无分号 → 值是表达式 |
| ② | `let b = { y+1; };` | `()` | 块内有分号 → 值变成 `()` |
| ③ | `{ y+1; };` | 丢弃 | **块后带分号** → 整个块作语句执行，值丢弃 |
| ④ | `{ y+1 };` | 丢弃 | 块是表达式但不赋给谁，值丢弃（会 warning） |

#### 你问的情况：块后面带分号
```rust
{ let y = 1; y + 1; };   // ← 块本身作为一条语句执行
```
1. 块内部 `y + 1;` 有分号 → 块的值是 `()`
2. 块外部末尾有分号 → 整个 `{ ... }` 变成一条独立语句，值（`()`）被丢弃
3. 块照常执行（`y = 1` 会执行，块结束后 `y` 被 drop），但不产生可用值

#### 用「上下文」理解分号
- `let a = { ... };` → 块在**表达式位置**（= 右边），值赋给 a
- `{ ... };`        → 块在**语句位置**，值被丢弃

#### 类比理解
把 `{}` 想象成函数：
- `let a = func();` → 接收返回值
- `func();`         → 不接收返回值（作为语句执行）

同理：
- `let a = { y+1 };` → 接收块的值
- `{ y+1 };`         → 不接收块的值

---

## ✅ 第 3 章 小结

学完本章你应该掌握：
1. ✅ 理解 `let` / `let mut` / `const` / shadowing 的区别
2. ✅ 知道 Rust 的标量类型和复合类型（Tuple、Array）
3. ✅ **理解语句 vs 表达式的区别**（加分号=语句！）
4. ✅ 会定义函数，理解返回值规则
5. ✅ 熟练使用 if、loop、while、for

---

## 📂 本章练习目录

- `concepts/src/variables.rs` —— 3.1 变量与可变性
- `concepts/src/data_types.rs` —— 3.2 数据类型
- `concepts/src/functions.rs` —— 3.3 函数
- `concepts/src/control_flow.rs` —— 3.4 控制流
- `concepts/Cargo.toml` —— 配置了 4 个 bin target，可独立运行
