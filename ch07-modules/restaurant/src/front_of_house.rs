// ===== front_of_house 模块：前厅（接待客人）=====
// 对应 main.rs 里的 mod front_of_house;

// ⚠️ 模块本身默认是私有的！要被 main.rs 访问，必须在 main.rs 里声明
//    而 main.rs 能声明它就表示它能「看到」这个模块

// pub mod：让 hosting 模块公开
pub mod hosting {
    // pub fn：让函数公开，外部可调用
    pub fn add_to_waitlist() {
        println!("把客人加入等候名单");
        seating_at_table();   // 同模块内可调用私有函数
    }

    // 没有 pub，是私有函数：只有 hosting 模块内部能用
    fn seating_at_table() {
        println!("安排客人入座");
    }
}

// 这个 mod 没有 pub，所以是私有模块：只有 front_of_house 内部能用
mod serving {
    fn take_order() {
        println!("点单");
    }

    fn serve_order() {
        println!("上菜");
    }

    fn take_payment() {
        println!("结账");
    }
}
