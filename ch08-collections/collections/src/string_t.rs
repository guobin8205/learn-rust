// ===== 8.2 String 字符串 =====
// 用 cargo run --bin string_t 运行

fn main() {
    // ========================================================
    // 一、Rust 的字符串类型（初学者必懂！）
    // ========================================================

    // &str   字符串切片：不可变，指向某处的字符串（栈上存指针+len）
    let s1: &str = "hello";             // 字面量是 &str

    // String 拥有所有权的字符串：可变，在堆上，可增长
    let s2: String = String::from("world");

    println!("{s1} {s2}");

    // ========================================================
    // 二、创建 String 的多种方式
    // ========================================================
    let empty = String::new();                       // 空字符串
    let from_literal = String::from("hello");        // 从字面量
    let to_string = "hello".to_string();             // 字面量调 to_string()
    let from_str: String = "hello".into();           // into 转换
    println!("{:?} {:?} {:?} {:?}", empty, from_literal, to_string, from_str);

    // ========================================================
    // 三、更新 String
    // ========================================================
    let mut s = String::from("foo");

    // ① push_str：追加 &str 切片
    s.push_str("bar");
    println!("push_str 后: {s}");            // foobar

    // ② push：追加单个字符 char（注意是单引号）
    s.push('!');
    println!("push 后: {s}");                 // foobar!

    // ③ + 运算符：拼接（注意会消耗左侧 String）
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;     // s1 被移动，s2 保留（因为传的是引用）
    println!("+ 拼接: {s3}");
    // println!("{s1}");   // ❌ s1 已被 + 消耗

    // ④ format! 宏：拼接多个（不消耗任何参数，最灵活 ⭐）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("format! 拼接: {s}");           // tic-tac-toe

    // ========================================================
    // 四、⭐ String 内部：为什么不能按索引访问？
    // ========================================================
    // String 内部是 Vec<u8>，存的是 UTF-8 字节
    let hello = String::from("Hola");
    println!("\"Hola\" 的字节数: {}", hello.len());    // 4 字节

    let russian = String::from("Здравствуйте");
    println!("\"Здравствуйте\" 的字节数: {}", russian.len());  // 24 字节（每个西里尔字母 2 字节）

    let chinese = String::from("你好");
    println!("\"你好\" 的字节数: {}", chinese.len());  // 6 字节（每个汉字 3 字节）

    // ⚠️ hello[0] 会怎样？
    // let c = hello[0];   // ❌ 编译错误！Rust 不允许 String 索引访问
    // 为什么？因为字节 ≠ 字符，s[0] 语义不清晰：
    //   "Hola"[0] 是 'H'（1 字节）
    //   "你好"[0] 应该是什么？前 1 字节？那不是合法字符！

    // ========================================================
    // 五、正确的字符串访问方式（三种「视图」）
    // ========================================================
    let s = String::from("你好，rust");

    // ① bytes()：按字节遍历
    println!("字节: {:?}", s.bytes().collect::<Vec<_>>());

    // ② chars()：按 Unicode 字符遍历（最常用）
    for c in s.chars() {
        print!("{c} ");
    }
    println!();

    // ③ char_indices()：字符 + 字节索引
    for (byte_idx, char) in s.char_indices() {
        println!("  字节位置 {byte_idx}: '{char}'");
    }

    // ========================================================
    // 六、切片（要小心 UTF-8 边界！）
    // ========================================================
    let hello = "Здравствуйте";     // 每个字母 2 字节

    // ✅ 在字符边界切（安全）
    let s = &hello[0..4];    // 前 2 个字母（4 字节）
    println!("切片 [0..4]: {s}");

    // ❌ 不在字符边界切（panic！）
    // let bad = &hello[0..1];   // panic：字节 1 不是字符边界
    // 💡 切字符串时要小心，最好先转成 chars() 操作

    // ========================================================
    // 七、遍历修改、查找等常用操作
    // ========================================================
    let s = String::from("hello world");

    println!("包含 'world': {}", s.contains("world"));
    println!("以 'hello' 开头: {}", s.starts_with("hello"));
    println!("替换: {}", s.replace("world", "rust"));
    println!("长度（字节）: {}", s.len());

    // ========================================================
    // ⭐ String vs &str 选择指南
    // ========================================================
    // | 场景               | 用什么     | 原因                  |
    // |-------------------|-----------|----------------------|
    // | 需要拥有、修改      | String    | 有所有权，可变         |
    // | 只读字符串          | &str      | 轻量，不持有所有权      |
    // | 函数参数            | &str ⭐   | 更通用（&String 能转） |
    // | 字符串字面量         | &str      | 天生就是 &str          |
    // | 结构体字段          | String    | 要拥有数据             |
}
