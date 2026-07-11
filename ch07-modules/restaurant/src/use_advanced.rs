// ===== use 的高级用法演示 =====
// 用 cargo run --bin use_advanced 运行

use std::cmp::Ordering;
use std::collections::HashMap;

// ========================================================
// 一、use 的嵌套路径语法（一次 use 多个）
// ========================================================
// 繁琐写法：
// use std::cmp::Ordering;
// use std::collections::HashMap;

// 简洁写法（用 {} 嵌套）：
// use std::{cmp::Ordering, collections::HashMap};

// ========================================================
// 二、as：给引入的项起别名（解决命名冲突）
// ========================================================
use std::fmt::Result;
use std::io::Result as IoResult;   // 起别名避免和上面的 Result 冲突

// ========================================================
// 三、pub use：重导出（让外部能用我的 use 引入的东西）
// ========================================================
// 公开我引入的东西，让外部可以通过我的模块路径访问

mod kitchen {
    pub struct Chef {
        pub name: String,
    }
}

pub use kitchen::Chef;   // pub use：外部可以直接 restaurant::Chef

// ========================================================
// 四、使用外部 crate
// ========================================================
// 在 Cargo.toml 里加依赖后，用 use 引入
// 比如 rand = "0.10" 后：
// use rand::RngExt;

fn main() {
    // ---------- 用 use 引入后，类型用起来更短 ----------
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("hello"), 1);

    // ---------- 演示 as 解决命名冲突 ----------
    let r1: Result = Ok(());
    let r2: IoResult<()> = Ok(());
    println!("{:?} {:?}", r1, r2);

    // ---------- 演示 pub use ----------
    let chef = Chef { name: String::from("Alice") };
    println!("主厨：{}", chef.name);

    println!("\n===== use 常用模式 =====");
    println!("use std::collections::HashMap;              // 引入类型");
    println!("use std::{{cmp::Ordering, collections::HashMap}}; // 嵌套引入");
    println!("use std::io::Result as IoResult;            // as 别名");
    println!("pub use xxx::Yyy;                           // 重导出");
}
