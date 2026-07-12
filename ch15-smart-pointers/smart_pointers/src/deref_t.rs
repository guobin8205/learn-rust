// ===== 15.2 Deref trait：自动解引用 =====
// 用 cargo run --bin deref_t 运行

use std::ops::Deref;

fn main() {
    println!("===== 1. 普通解引用 =====\n");

    let x = 5;
    let r = &x;
    assert_eq!(5, x);
    assert_eq!(5, *r);      // * 显式解引用：从 &i32 取出 i32

    let b = Box::new(5);
    assert_eq!(5, *b);      // Box 也能用 * 解引用

    println!("Box 也能用 * 解引用: {}", *b);

    println!("\n===== 2. 自定义智能指针 + 实现 Deref =====\n");

    let m = MyBox::new(String::from("Rust"));
    // 用 * 解引用（因为实现了 Deref）
    // *m 调用 deref() 得到 &String，再 * 得到 String
    println!("m = {}", *m);

    println!("\n===== 3. ⭐ Deref 强制转换（deref coercion）=====\n");

    // ⭐ 关键魔法：函数需要 &str，传 &MyBox<String> 也能用！
    let m = MyBox::new(String::from("Hello"));
    hello(&m);   // ✅ 自动转换：&MyBox<String> → &String → &str

    // 转换链：
    //   &MyBox<String>  ──deref──→  &String  ──deref──→  &str
    //
    // Rust 看到函数参数类型不匹配时，会自动链式调用 deref() 转换

    // 对比：不用 Deref 强制转换的写法
    hello(&(*m)[..]);   // 手动转换，繁琐

    println!("\n===== 4. Deref 在标准库的应用 =====\n");

    // String 实现了 Deref<Target=str>
    // 所以 &String 能自动转成 &str
    let s = String::from("hello");
    takes_str(&s);   // ✅ &String 自动转 &str

    // Vec<T> 实现了 Deref<Target=[T]>
    // 所以 &Vec<T> 能自动转成 &[T]
    let v = vec![1, 2, 3];
    takes_slice(&v);   // ✅ &Vec 自动转 &[i32]

    println!("\n===== 5. DerefMut：可变解引用 =====\n");

    // Deref 用于 &T，DerefMut 用于 &mut T
    let mut s = MyBox::new(String::from("hi"));
    s.push_str(" there");   // 需要 DerefMut 才能修改内部值
    println!("修改后: {}", *s);
}

// ---------- 接收 &str 的函数 ----------
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn takes_str(s: &str) { println!("takes_str: {s}"); }
fn takes_slice(s: &[i32]) { println!("takes_slice: {:?}", s); }

// ========================================================
// 自定义智能指针：MyBox
// ========================================================
struct MyBox<T>(T);    // 元组结构体，包含一个 T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ⭐ 实现 Deref：让 MyBox 支持 * 解引用
impl<T> Deref for MyBox<T> {
    type Target = T;    // 解引用的目标类型

    fn deref(&self) -> &Self::Target {
        &self.0     // 返回内部值的引用
    }
}

// 实现 DerefMut：支持可变解引用
use std::ops::DerefMut;
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ========================================================
// ⭐ Deref 强制转换的三种情形
// ========================================================
// 当 T: Deref<Target=U> 时：
//   &T → &U      （不可变转换）
//
// 当 T: DerefMut<Target=U> 时：
//   &mut T → &mut U  （可变转换）
//   &mut T → &U      （可变也能转不可变）
//
// 优先级：编译器优先匹配，不行才尝试 deref 转换
// 完整转换链：&MyBox<String> → &String → &str
