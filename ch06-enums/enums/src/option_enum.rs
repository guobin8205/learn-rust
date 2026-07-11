// ===== 6.2 Option<T> —— 消灭 null 的利器 =====
// 用 cargo run --bin option_enum 运行

fn main() {
    // ========================================================
    // 一、Option 是什么？为什么 Rust 没有 null？
    // ========================================================
    // 标准库定义：
    //   enum Option<T> {
    //       Some(T),   // 有值，T 是任意类型
    //       None,      // 无值
    //   }
    //
    // Rust 没有 null！用 Option<T> 表达「可能有值，也可能没有」
    // 这是 Rust 消灭「空指针异常 (NullPointerException)」的核心设计

    // ---------- ① 创建 Option ----------
    let some_number = Some(5);          // Option<i32>（类型自动推断）
    let some_string = Some("hello");    // Option<&str>
    let absent: Option<i32> = None;     // 必须标注类型（否则编译器不知道 None 是什么类型）
    println!("{:?} {:?} {:?}", some_number, some_string, absent);

    // ---------- ② Option<T> 和 T 是不同的类型！⭐ ----------
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    // let sum = x + y;   // ❌ 编译错误！不能把 i32 和 Option<i32> 相加
    // 为什么？因为 Option<i32> 不一定是 i32，可能没有值
    // 编译器强制你「先处理 None 的情况」，才能拿到里面的值
    let sum = x + y.unwrap_or(0);   // ✅ 明确处理 None：没有就当 0
    println!("sum = {sum}");

    // ========================================================
    // 二、从 Option 中取值的方式
    // ========================================================

    // ---------- ③ unwrap：直接取，None 就 panic（危险！）----------
    let n = some_number.unwrap();       // ✅ Some(5) → 5
    println!("unwrap Some: {n}");
    // absent.unwrap();                 // ❌ panic！如果是 None 会直接崩
    // 💡 unwrap 只在「你确定一定是 Some」或「原型快速验证」时用

    // ---------- ④ expect：和 unwrap 一样，但能自定义错误信息 ----------
    let n2 = some_number.expect("数字应该是 Some");
    println!("expect Some: {n2}");

    // ---------- ⑤ unwrap_or：None 时用默认值（安全 ✅）----------
    let v1 = some_number.unwrap_or(0);  // Some(5) → 5
    let v2 = absent.unwrap_or(0);       // None → 0
    println!("unwrap_or: {v1}, {v2}");

    // ---------- ⑥ unwrap_or_else：None 时用闭包计算默认值 ----------
    let v3 = absent.unwrap_or_else(|| expensive_default());
    println!("unwrap_or_else: {v3}");

    // ---------- ⑦ is_some / is_none：判断 ----------
    println!("some_number 有值? {}", some_number.is_some());
    println!("absent 有值? {}", absent.is_some());

    // ---------- ⑧ map / and_then：函数式风格（第 13 章详讲）----------
    let doubled = some_number.map(|n| n * 2);   // Some(5) → Some(10)
    println!("map: {:?}", doubled);

    // ========================================================
    // 三、match 是处理 Option 最 Rust 的方式（6.3 节详解）
    // ========================================================
    let result = match some_number {
        Some(n) => format!("有值: {n}"),
        None => String::from("无值"),
    };
    println!("match: {result}");

    // ========================================================
    // 四、常见返回 Option 的标准库方法
    // ========================================================
    // | 方法/场景                | 返回            | 说明              |
    // |-------------------------|-----------------|------------------|
    // | vec.get(index)          | Option<&T>      | 越界返回 None      |
    // | map.get(&key)           | Option<&V>      | 键不存在返回 None   |
    // | str.find(pattern)       | Option<usize>   | 没找到返回 None     |
    // | "123".parse::<i32>()    | Result（类似）    | 解析失败返回 Err    |
    // | env::var("PATH")        | Result（类似）    | 变量不存在返回 Err  |

    let v = vec![10, 20, 30];
    let safe_access = v.get(1);   // Some(&20)
    let out_of_bounds = v.get(10); // None（不是 panic！）
    println!("安全访问: {:?}, 越界: {:?}", safe_access, out_of_bounds);
    // 对比：v[10] 会直接 panic（第 3 章 Q1 学过）

    // ========================================================
    // ⭐ Option 的核心价值
    // ========================================================
    // 1. 「可能为空」这件事写进了类型，编译器强制你处理
    // 2. 忘记处理 None → 编译错误（而不是运行时空指针崩溃）
    // 3. T 和 Option<T> 是不同类型，你不能「不小心」用了可能为空的值
}

fn expensive_default() -> i32 {
    999   // 假装这是很昂贵的计算
}
