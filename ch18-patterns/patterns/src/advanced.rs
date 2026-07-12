// ===== 18.3 模式实战应用 =====
// 用 cargo run --bin advanced 运行

use std::collections::HashMap;

#[derive(Debug)]
enum ApiError {
    NotFound,
    Unauthorized,
    RateLimited { retry_after: u64 },
    Server(u16),
    Network(String),
}

#[derive(Debug)]
enum HttpResponse {
    Ok(String),
    Error(ApiError),
}

fn fetch(url: &str) -> HttpResponse {
    // 模拟不同的响应
    match url {
        "ok" => HttpResponse::Ok(String::from("成功数据")),
        "404" => HttpResponse::Error(ApiError::NotFound),
        "401" => HttpResponse::Error(ApiError::Unauthorized),
        "429" => HttpResponse::Error(ApiError::RateLimited { retry_after: 60 }),
        "500" => HttpResponse::Error(ApiError::Server(500)),
        "net" => HttpResponse::Error(ApiError::Network(String::from("超时"))),
        _ => HttpResponse::Ok(String::from("默认")),
    }
}

fn main() {
    println!("===== 1. 实战：HTTP 错误处理 =====\n");

    let urls = ["ok", "404", "401", "429", "500", "net"];

    for url in urls {
        match fetch(url) {
            HttpResponse::Ok(data) => println!("✅ {}: {}", url, data),
            HttpResponse::Error(ApiError::NotFound) => {
                println!("❌ {}: 资源不存在", url);
            }
            HttpResponse::Error(ApiError::Unauthorized) => {
                println!("❌ {}: 未授权", url);
            }
            HttpResponse::Error(ApiError::RateLimited { retry_after }) => {
                println!("⚠️ {}: 被限流，{}秒后重试", url, retry_after);
            }
            HttpResponse::Error(ApiError::Server(code)) => {
                println!("❌ {}: 服务器错误 {}", url, code);
            }
            HttpResponse::Error(ApiError::Network(msg)) => {
                println!("❌ {}: 网络错误 {}", url, msg);
            }
        }
    }

    println!("\n===== 2. 实战：JSON 值解析 =====\n");

    // 用 enum 表示 JSON 值（第 6 章学过）
    #[derive(Debug)]
    enum Json {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(HashMap<String, Json>),
    }

    let data = Json::Object({
        let mut m = HashMap::new();
        m.insert("name".to_string(), Json::String("Alice".to_string()));
        m.insert("age".to_string(), Json::Number(30.0));
        m.insert("admin".to_string(), Json::Bool(true));
        m
    });

    // 深度解构 JSON
    fn extract_name(json: &Json) -> Option<&str> {
        match json {
            Json::Object(map) => match map.get("name")? {
                Json::String(s) => Some(s),
                _ => None,
            },
            _ => None,
        }
    }

    if let Some(name) = extract_name(&data) {
        println!("提取到名字: {}", name);
    }

    println!("\n===== 3. 实战：状态机（用模式实现）=====\n");

    #[derive(Debug)]
    enum OrderState {
        Created,
        Paid { amount: u64 },
        Shipped { tracking: String },
        Delivered,
        Cancelled { reason: String },
    }

    fn process_order(state: OrderState) -> OrderState {
        match state {
            OrderState::Created => {
                println!("订单创建 → 待付款");
                OrderState::Paid { amount: 100 }
            }
            OrderState::Paid { amount } => {
                println!("已付款 {}，发货", amount);
                OrderState::Shipped { tracking: "SF123456".to_string() }
            }
            OrderState::Shipped { tracking } => {
                println!("运单 {} 已签收", tracking);
                OrderState::Delivered
            }
            OrderState::Delivered => {
                println!("订单已完成");
                OrderState::Delivered
            }
            OrderState::Cancelled { reason } => {
                println!("订单已取消: {}", reason);
                OrderState::Cancelled { reason }
            }
        }
    }

    let mut state = OrderState::Created;
    state = process_order(state);
    state = process_order(state);
    state = process_order(state);
    state = process_order(state);

    println!("\n===== 4. 实战：解构 Option 链（避免嵌套 if）=====\n");

    let config = Some(Some(Some("production")));

    // 嵌套 if 写法（繁琐）
    // if let Some(level1) = config {
    //     if let Some(level2) = level1 {
    //         if let Some(level3) = level2 {
    //             println!("{}", level3);
    //         }
    //     }
    // }

    // ⭐ 模式匹配写法（简洁）
    match config {
        Some(Some(Some(env))) => println!("环境: {}", env),
        _ => println!("配置缺失"),
    }

    println!("\n===== 5. 实战：绑定 + 守卫的强大组合 =====\n");

    let command = ("delete", Some("important_file"));

    match command {
        ("create", _) => println!("创建"),
        ("read", Some(name)) => println!("读取 {}", name),
        ("read", None) => println!("请指定文件名"),
        ("delete", Some(name)) if name.starts_with("important") => {
            println!("⚠️ 不允许删除重要文件: {}", name);
        }
        ("delete", Some(name)) => println!("删除 {}", name),
        ("delete", None) => println!("请指定要删除的文件"),
        _ => println!("未知命令"),
    }

    println!("\n===== 6. 实战：提取多层数据 =====\n");

    // 假设的用户数据
    let user = (
        "Alice",
        ("Beijing", "China"),
        Some(("010-12345678", "alice@example.com")),
    );

    // 一次性提取所有数据
    let (name, (city, country), contact) = user;
    println!("姓名: {}", name);
    println!("位置: {}, {}", city, country);
    if let Some((phone, email)) = contact {
        println!("联系方式: {} / {}", phone, email);
    }

    println!("\n===== 7. 模式匹配的设计哲学 =====\n");

    println!("| 传统语言（if/switch）       | Rust 模式匹配              |");
    println!("|---------------------------|---------------------------|");
    println!("| 先取值再判断                | 同时解构 + 判断 + 绑定      |");
    println!("| 忘记处理某个 case 是 bug    | 编译器强制穷尽              |");
    println!("| 嵌套条件层层缩进            | 扁平化 match 分支           |");
    println!("| 类型断言可能出错            | 类型安全，编译期检查         |");

    println!("\n⭐ 模式匹配是 Rust 最强大的控制流工具");
    println!("  它让你能同时：");
    println!("  1. 测试值的结构（是不是 Some？是哪个变体？）");
    println!("  2. 提取内部数据（绑定到变量）");
    println!("  3. 加额外条件（守卫）");
    println!("  4. 保证穷尽所有情况（编译器强制）");
}
