// ===== 5.3 打印结构体 & 派生 Debug trait =====
// 用 cargo run --bin debug_trait 运行

fn main() {
    // ========================================================
    // 问题：怎么打印结构体？
    // ========================================================

    let rect = Rectangle { width: 30, height: 50 };

    // ❌ println!("{ }", rect);   // 编译错误！默认不知道怎么打印结构体
    // 解决方法 1：#[derive(Debug)]  让结构体支持 Debug 打印（见下方结构体定义）
    // 解决方法 2：手动实现 Display 或 Debug trait（第 10 章详讲）

    // ---------- 方式 1：{:?} 调试输出 ----------
    println!("{:?}", rect);
    println!("rect: ({}, {})", rect.width, rect.height);
    // 输出：Rectangle { width: 30, height: 50 }

    // ---------- 方式 2：{:#?} 美化调试输出 ----------
    println!("{:#?}", rect);
    // 输出（多行美化）：
    // Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    // ---------- 方式 3：dbg! 宏（开发调试利器）----------
    // dbg! 会：① 打印到「标准错误」 ② 打印「文件名:行号: 表达式 = 值」③ 返回值
    let scale = 2;
    let r = Rectangle {
        width: dbg!(scale * 10),   // 会打印 [src/debug_trait.rs:35] scale * 10 = 20
        height: 50,
    };
    dbg!(&r);   // 打印 r 的内容，传引用避免移动

    // println! vs dbg! 区别：
    // | 特性     | println!           | dbg!               |
    // |---------|--------------------|--------------------|
    // | 输出流   | stdout（标准输出）  | stderr（标准错误）  |
    // | 打印格式 | 自定义              | 固定 Debug 格式     |
    // | 行号     | ❌                  | ✅ 文件名:行号      |
    // | 返回值   | ❌（返回 ()）       | ✅ 返回表达式的值   |
    // | 场景     | 给用户看的输出      | 开发调试用          |
}

// ========================================================
// #[derive(Debug)] 让结构体可调试打印
// ========================================================
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ========================================================
// 拓展：常见的派生 trait（预览，第 10 章详讲）
// ========================================================
// #[derive(Debug)]      → 支持用 {:?} 打印
// #[derive(Clone)]      → 支持 .clone() 深拷贝
// #[derive(Copy)]       → 支持按位拷贝（需要所有字段都 Copy）
// #[derive(PartialEq)]  → 支持 == != 比较
// #[derive(Hash)]       → 支持放入 HashMap
//
// derive 的意义：Rust 自动为结构体生成这些 trait 的实现代码，不用手写
// 例如 #[derive(Debug)] 等价于编译器帮你写了：
//   impl Debug for Rectangle {
//       fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//           f.debug_struct("Rectangle")
//               .field("width", &self.width)
//               .field("height", &self.height)
//               .finish()
//       }
//   }
