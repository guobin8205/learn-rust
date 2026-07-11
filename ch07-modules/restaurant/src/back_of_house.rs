// ===== back_of_house 模块：后厨（厨房）=====
// 对应 main.rs 里的 mod back_of_house;

// ⚠️ 这一行很关键！没有 pub，main.rs 虽然能声明这个模块
//    但内部的 pub 东西仍然可以通过 crate::back_of_house::... 访问
//    （因为 main.rs 是 crate 根，能直接看到顶层 mod 的公开内容）

// ---------- pub enum：枚举公开 = 所有变体也公开 ----------
#[derive(Debug)]
pub enum Appetizer {
    Soup,    // 自动公开（因为 enum 是 pub）
    Salad,   // 自动公开
}

// ---------- pub struct：结构体公开 ≠ 字段公开 ⭐ ----------
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,           // pub 字段：外部能访问/修改
    seasonal_fruit: String,      // 私有字段：外部不能直接访问
}

impl Breakfast {
    // pub 关联函数（类似构造函数）：外部可以创建实例
    pub fn summer(toast_type: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast_type),
            seasonal_fruit: String::from("peaches"),   // 私有字段，只能在内部设置
        }
    }
}

// ---------- pub fn ----------
pub fn fix_incorrect_order() {
    cook_order();
    // ⭐ super 关键字：引用父模块（类似文件系统的 ..）
    // super::front_of_house::hosting::add_to_waitlist();  // super 回到 crate 根再下钻
}

fn cook_order() {
    println!("烹饪中...");
}
