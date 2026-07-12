// ===== 19.2 高级 Trait =====
// 用 cargo run --bin advanced_traits 运行

use std::fmt;

// ========================================================
// 一、关联类型（Associated Types）⭐
// ========================================================

// 普通泛型 trait：每次实现要指定类型
#[allow(dead_code)]
trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

// 关联类型：trait 里有个 type，实现时指定
trait MyIterator {
    type Item;                    // ⭐ 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

// 实现时确定 Item 的具体类型
struct Counter { count: u32, max: u32 }

impl MyIterator for Counter {
    type Item = u32;              // ← 这里指定具体类型
    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else { None }
    }
}

// ⭐ 关联类型 vs 泛型参数
// | 特性          | 泛型 trait         | 关联类型 trait       |
// |-------------|-------------------|--------------------|
// | 一个类型能多次实现？ | ✅ 能             | ❌ 只能一次           |
// | 使用时需指定类型？  | ✅ Iterator<Item=T> | ❌ 自动推断           |
// | 代表           | trait 的「参数」    | 实现 trait 的「输出」  |

// ========================================================
// 二、运算符重载
// ========================================================

#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }

use std::ops::Add;

impl Add for Point {
    type Output = Point;          // ⭐ Add trait 的关联类型

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

// 完全限定语法（Fully Qualified Syntax）：调用特定 trait 的方法
trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }

struct Human;
impl Human { fn fly(&self) { println!("人类不会飞（原方法）"); } }
impl Pilot for Human { fn fly(&self) { println!("机长在开飞机"); } }
impl Wizard for Human { fn fly(&self) { println!("巫师骑着扫帚飞"); } }

// ========================================================
// 三、Supertrait：trait 依赖另一个 trait
// ========================================================

// trait C: A + B  表示实现 C 必须先实现 A 和 B
trait Greet: fmt::Display {
    fn greet(&self) {
        println!("Hello, {}!", self);   // 可以用 Display 的方法
    }
}

struct Person { name: String }
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Greet for Person {}   // ⭐ 必须先实现 Display 才能实现 Greet

// ========================================================
// 四、newtype 模式（绕过孤儿规则 + 类型安全）
// ========================================================

// newtype：用元组结构体包装一个类型
struct Meters(u32);        // ⭐ 和 u32 是不同类型
struct Kilometers(u32);

impl Kilometers {
    fn to_meters(self) -> Meters {
        Meters(self.0 * 1000)
    }
}

// 这样就不会把 Meters 和 Kilometers 混淆了
fn add_kilometers(a: Kilometers, b: Kilometers) -> Kilometers {
    Kilometers(a.0 + b.0)
}
// fn add_wrong(a: Kilometers, b: Meters) -> ???   // ❌ 编译时就能发现单位错误

// newtype 还能绕过孤儿规则：为外部类型实现外部 trait
struct Wrapper(Vec<String>);   // 包装 Vec<String>

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("===== 1. 关联类型 =====\n");
    let mut counter = Counter { count: 0, max: 3 };
    println!("next: {:?}", counter.next());
    println!("next: {:?}", counter.next());
    println!("next: {:?}", counter.next());
    println!("next: {:?}", counter.next());

    println!("\n===== 2. 运算符重载 =====\n");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;   // ⭐ 用 + 运算符
    println!("Point + Point = {:?}", p3);

    println!("\n===== 3. 完全限定语法（消除歧义）=====\n");
    let person = Human;
    person.fly();               // 默认调用原方法
    Pilot::fly(&person);        // 指定 Pilot trait
    Wizard::fly(&person);       // 指定 Wizard trait

    println!("\n===== 4. Supertrait =====\n");
    let p = Person { name: String::from("Alice") };
    p.greet();   // 用了 Display 的能力

    println!("\n===== 5. newtype 模式 =====\n");
    let km = Kilometers(5);
    let m = km.to_meters();
    println!("5 公里 = {} 米", m.0);

    let total = add_kilometers(Kilometers(3), Kilometers(4));
    println!("3km + 4km = {}km", total.0);

    // newtype 绕过孤儿规则
    let w = Wrapper(vec![String::from("a"), String::from("b")]);
    println!("Wrapper: {}", w);   // ✅ 为 Vec 包装类型实现 Display

    println!("\n===== 6. 高级 trait 速查 =====\n");
    println!("| 特性             | 用途                      |");
    println!("|-----------------|---------------------------|");
    println!("| 关联类型 type    | trait 的「输出类型」        |");
    println!("| 运算符重载       | 实现 + - * / 等运算符       |");
    println!("| 完全限定语法     | 消除同名方法歧义           |");
    println!("| Supertrait       | trait 依赖另一个 trait     |");
    println!("| newtype 模式     | 类型安全 + 绕过孤儿规则    |");
}

// ========================================================
// ⭐ 孤儿规则（Orphan Rule）
// ========================================================
// Rust 规定：要为类型 T 实现 trait Trait，T 或 Trait 至少有一个
// 在当前 crate 定义。否则不能实现。
//
// 为什么？避免不同 crate 的实现冲突。
//
// 解决方案：newtype 模式
//   想为 Vec<String> 实现 Display，但 Vec 和 Display 都不是你定义的
//   → 用 Wrapper(Vec<String>) 包装，Wrapper 是你的类型 → 可以实现
