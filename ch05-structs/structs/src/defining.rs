// ===== 5.1 定义结构体并实例化 (Defining and Instantiating Structs) =====
// 用 cargo run --bin defining 运行

fn main() {
    // ========================================================
    // 一、经典结构体 (Classic Struct) —— 命名字段
    // ========================================================

    // 定义方式：struct 名字 { 字段: 类型, ... }
    // 注意：定义在 main 外面（见文件底部）

    // 实例化：必须「所有字段都赋值」，字段顺序可以打乱
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // ⚠️ user1 是不可变的，不能修改任何字段
    // user1.email = String::from("new");  // ❌ 编译错误

    // 要修改字段，整个实例必须是 mut
    let mut user2 = User {
        active: true,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("alice_new@example.com");   // ✅ 可以改
    println!("user2 email: {}", user2.email);

    // ---------- 访问字段：用点号 ----------
    println!("user1: active={}, username={}, sign_in_count={}",
             user1.active, user1.username, user1.sign_in_count);

    // ========================================================
    // 二、字段简写 (Field Init Shorthand) —— 变量名和字段名相同时
    // ========================================================

    let username = String::from("bob");
    let email = String::from("bob@example.com");

    // 简写：如果变量名和字段名一样，可以只写变量名
    let user3 = User {
        active: true,
        username,        // ← 简写！等价于 username: username
        email,           // ← 简写！等价于 email: email
        sign_in_count: 1,
    };
    println!("user3 username: {}", user3.username);

    // ========================================================
    // 三、结构体更新语法 (Struct Update Syntax) —— 用另一个实例填充剩余字段
    // ========================================================

    let user4 = User {
        email: String::from("new@example.com"),  // 只改 email
        ..user3      // ← 其余字段从 user3 拷贝/移动过来
        // ⚠️ 注意：..user3 会「移动」user3 中的 String 字段（username, email）
        //    所以 user3 之后不能整体再用了（但未移动的标量字段如 active 仍能用）
    };
    println!("user4: email={}, sign_in_count={}", user4.email, user4.sign_in_count);
    // println!("{}", user3.username);  // ❌ user3.username 已被移动
    // println!("{}", user3.active);    // ✅ active 是 bool（Copy），仍能用

    // ========================================================
    // 四、元组结构体 (Tuple Struct) —— 有名字的元组
    // ========================================================

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // ⚠️ Color 和 Point 虽然内部都是 (i32,i32,i32)，但它们是「不同类型」
    //    let wrong: Point = black;  // ❌ 类型不匹配，编译错误

    // 访问元组结构体：用 .0 .1 .2（和元组一样）
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 何时用元组结构体？当你想给元组起个名字以区分含义，但字段名不重要时
    // 比如坐标、颜色、尺寸这种

    // ========================================================
    // 五、单元结构体 (Unit-Like Struct) —— 没有任何字段
    // ========================================================

    struct AlwaysEqual;

    let _subject = AlwaysEqual;
    // 没有字段，也不占空间。它的价值在于「在某个类型上实现 trait」（第 10 章会讲）

    // ========================================================
    // 六、结构体中的所有权（重要！）
    // ========================================================
    // 结构体可以拥有数据（用 String 而不是 &str），也可以引用数据（用 &str）
    // 但引用需要「生命周期」标注（第 10 章详讲），否则编译错误：
    //
    // struct BadUser {
    //     name: &str,    // ❌ 缺少生命周期标注，编译错误
    // }
    //
    // 现在阶段：结构体字段用 String 这种拥有所有权的类型，避免引用
}

// ========================================================
// 结构体定义
// ========================================================
struct User {
    active: bool,
    username: String,    // 用 String（拥有所有权），不用 &str
    email: String,
    sign_in_count: u64,
}

// ========================================================
// ⚠️ 结构体里的字段不能拿「部分」出来用，要么全有要么全无
// ⚠️ 没有「部分初始化」，所有字段都必须赋值
// ========================================================

// 一个返回结构体的函数（演示构造函数模式）
#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,          // 字段简写
        email,             // 字段简写
        sign_in_count: 1,
    }
}
