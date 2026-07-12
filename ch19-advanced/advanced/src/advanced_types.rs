// ===== 19.3 高级类型与函数 =====
// 用 cargo run --bin advanced_types 运行

// ========================================================
// 一、类型别名（type）
// ========================================================

// type 创建别名，不创建新类型
type Kilometers = i32;           // ⚠️ Kilometers 和 i32 是同一类型

type Thunk = Box<dyn Fn() + Send + 'static>;   // 简化复杂类型

// 函数指针类型
type MathOp = fn(i32, i32) -> i32;

// ========================================================
// 二、Never 类型（!）
// ========================================================
// ! 表示「永不返回」的类型
// panic!、无限 loop、continue 都返回 !

fn bar() -> ! {                  // 永不返回的函数
    panic!("这个函数永远不会正常返回");
}

// never 类型可以强转为任何类型（因为它不会产生值）
fn never_demo() {
    let _x: i32 = match Some(5) {
        Some(n) => n,
        None => panic!("None"),      // panic! 返回 !，可以当 i32
    };
}

// ========================================================
// 三、动态大小类型（DST）和 Sized
// ========================================================

// str 是 DST（编译期不知道大小），只能通过 &str 使用
// 因为 &str 是胖指针（指针 + 长度）

// ?Sized：表示「可以是 Sized 或 DST」
fn generic<T: ?Sized>(item: &T) {   // ⭐ ?Sized 允许 DST
    // 只能通过引用使用 T
}

// ========================================================
// 四、函数指针（fn 类型）
// ========================================================

fn add(x: i32, y: i32) -> i32 { x + y }
fn mul(x: i32, y: i32) -> i32 { x * y }

fn do_math(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)   // 函数指针，不是闭包
}

// ========================================================
// 五、返回闭包
// ========================================================

// 闭包是 trait 对象，必须 Box 包装才能返回
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 也可以用 impl Fn（更高效）
fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// ========================================================
// 六、宏入门（macro_rules!）
// ========================================================

// 声明宏：模式匹配生成代码
macro_rules! my_vec {
    // 匹配模式，$x 是元变量
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

// 简单的求值宏
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )*
            total
        }
    };
}

fn main() {
    println!("===== 1. 类型别名 =====\n");
    let distance: Kilometers = 5;
    let count: i32 = 10;
    println!("distance + count = {}", distance + count);  // ⚠️ 可以相加！

    // ⚠️ type 别名的风险：类型安全没提升
    // Kilometers 和 i32 是同一类型，编译器分不出来
    // 真要类型安全，用 newtype 模式（struct Kilometers(u32)）

    let f: Thunk = Box::new(|| println!("执行"));
    drop(f);

    println!("\n===== 2. 函数指针 =====\n");
    println!("add(2, 3) = {}", do_math(add, 2, 3));
    println!("mul(2, 3) = {}", do_math(mul, 2, 3));

    // 函数指针 vs 闭包
    // | 特性       | fn 指针      | 闭包           |
    // |-----------|-------------|----------------|
    // | 捕获环境   | ❌ 不能      | ✅ 能           |
    // | 大小       | 固定（指针） | 不固定（有状态） |
    // | 能 Copy    | ✅           | 取决于捕获      |

    println!("\n===== 3. 返回闭包 =====\n");
    let f1 = returns_closure();
    println!("闭包(5) = {}", f1(5));

    let f2 = returns_closure_impl();
    println!("闭包(5) = {}", f2(5));

    println!("\n===== 4. 宏 =====\n");
    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec! = {:?}", v);

    let v2 = my_vec![10, 20];
    println!("my_vec! = {:?}", v2);

    println!("sum! = {}", sum![1, 2, 3, 4, 5]);

    println!("\n===== 5. 宏 vs 函数 =====\n");
    println!("| 特性          | 宏                   | 函数              |");
    println!("|--------------|---------------------|-------------------|");
    println!("| 何时展开       | 编译期                | 运行期             |");
    println!("| 参数数量       | 可变                  | 固定               |");
    println!("| 参数类型       | 任意（模式匹配）        | 固定               |");
    println!("| 能否拿到 AST    | ✅                   | ❌                |");
    println!("| 可见性         | 需要导入（同文件外）     | 同 crate 可用      |");

    println!("\n===== 6. 高级特性总结 =====\n");
    println!("| 特性          | 何时用                  |");
    println!("|--------------|------------------------|");
    println!("| 类型别名 type  | 简化复杂类型             |");
    println!("| newtype 模式  | 类型安全（推荐）         |");
    println!("| 函数指针 fn    | 不需要捕获环境的回调      |");
    println!("| 宏             | 编译期代码生成           |");
    println!("| unsafe         | 调用 C / 底层操作        |");
}
