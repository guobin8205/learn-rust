// ===== 6.1 枚举 (Enums) =====
// 用 cargo run --bin defining 运行

fn main() {
    // ========================================================
    // 一、枚举：表达「一个值可能是几种情况之一」
    // ========================================================
    // 比 C/Java 的 enum 强大：每个变体可以携带不同的数据！

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 枚举变体都在 V4/V6 这个命名空间下，用 :: 访问
    println!("{:?} {:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // ========================================================
    // 二、枚举变体可以携带数据（⭐ Rust enum 的杀手锏）
    // ========================================================

    // ① 简单版：枚举只表示「类型」，数据另存
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home: {:?}", home);

    // ② 进阶版：枚举变体直接携带数据！
    // 这是 Rust enum 和其他语言 enum 最大的区别
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home2: {:?}", home2);
    println!("loopback: {:?}", loopback);
    // ↑ 一个枚举值就把「类型」和「数据」打包在一起了，比 struct 更简洁

    // ③ 变体可以携带「不同类型、不同数量」的数据
    let m1 = Message::Quit;                        // 无数据
    let m2 = Message::Write(String::from("hi"));   // 一个 String
    let m3 = Message::Move { x: 10, y: 20 };       // 像结构体一样的字段
    let m4 = Message::ChangeColor(255, 0, 0);      // 三个 i32
    println!("m1={:?} m2={:?} m3={:?} m4={:?}", m1, m2, m3, m4);

    // ⚠️ 重点：虽然变体携带的数据类型不同，但它们都是同一个枚举类型 Message
    //   let msgs: Vec<Message> = vec![m1, m2, m3, m4];  // ✅ 可以放一个 Vec 里

    // ========================================================
    // 三、枚举也可以有方法（impl 块，和 struct 一样）
    // ========================================================
    let m = Message::Write(String::from("hello"));
    m.call();   // 枚举的方法，用 . 调用，和 struct 一样

    // ========================================================
    // 四、枚举 vs 结构体：什么时候用哪个？
    // ========================================================
    // | 问题              | 结构体                      | 枚举                     |
    // |------------------|----------------------------|--------------------------|
    // | 「和」：各字段同时存在 | ✅                          | ❌                        |
    // | 「或」：几种情况之一  | ❌                          | ✅                        |
    //
    // 例：「一个用户有名字、邮箱、年龄」→ struct（字段同时存在）
    // 例：「一条消息是 Quit / Write / Move / ChangeColor 之一」→ enum（多选一）

    // ========================================================
    // 五、Option<T>：标准库最重要的枚举（消灭 null！）
    // ========================================================
    // 见 6.2 节详解
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// ⭐ 用枚举把类型和数据打包，比 struct + enum 更简洁
#[derive(Debug)]
enum IpAddr2 {
    V4(String),    // V4 变体携带一个 String
    V6(String),    // V6 变体携带一个 String
}

// 变体可以携带「不同类型、不同数量」的数据
#[derive(Debug)]
enum Message {
    Quit,                        // 无数据
    Write(String),               // 携带一个 String
    Move { x: i32, y: i32 },     // 携带命名字段（像 struct）
    ChangeColor(i32, i32, i32),  // 携带三个 i32（像 tuple）
}

impl Message {
    fn call(&self) {
        println!("调用了 Message 的方法: {:?}", self);
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("路由: {:?}", ip_kind);
}
