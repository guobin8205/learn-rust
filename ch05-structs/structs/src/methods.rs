// ===== 5.2 方法与关联函数 (Methods & Associated Functions) =====
// 用 cargo run --bin methods 运行

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // ---------- 方法调用：用点号 ----------
    println!(
        "矩形 {:?} 的面积是 {} 平方像素",
        rect1,
        rect1.area()       // ← 方法调用
    );

    // ---------- 关联函数调用：用 ::（类似「静态方法」）----------
    let sq = Rectangle::square(3);   // ← 关联函数，创建 3x3 的正方形
    println!("正方形: {:?}", sq);
    println!("正方形面积: {}", sq.area());

    let rect2 = Rectangle::new(10, 20);
    println!("new 创建: {:?}, 面积 {}", rect2, rect2.area());

    // ---------- 方法链（如果方法返回 Self 可以链式调用）----------
    let mut rect3 = Rectangle::new(5, 5);
    rect3.scale(2);     // 把宽高都放大 2 倍
    println!("放大后: {:?}, 面积 {}", rect3, rect3.area());

    // ---------- 带借用参数的方法 ----------
    let rect4 = Rectangle::new(40, 20);
    let rect5 = Rectangle::new(10, 5);
    println!("rect4 能装下 rect5 吗？ {}", rect4.can_hold(&rect5));
    println!("rect5 能装下 rect4 吗？ {}", rect5.can_hold(&rect4));

    // ---------- 自动引用和解引用 ----------
    // rect1.area() 实际上是 Rust 自动加了 &（或 &mut）
    // C/Java 要手动写 obj.method() 或 obj->method()，Rust 自动处理
    // 你只需要决定「读」还是「写」，Rust 自动加 &
}

// ========================================================
// impl 块：给 Rectangle 定义方法和关联函数
// ========================================================
impl Rectangle {
    // ---------- 方法：第一个参数是 self / &self / &mut self ----------
    // &self 是 self: &Self 的简写，表示「借用实例」来读

    /// 计算面积（只读借用 self）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 判断能否容纳另一个矩形（只读借用 self 和 other）
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// 放大尺寸（可变借用 self）
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // ---------- 关联函数：没有 self 参数（类似「静态方法」/构造函数）----------

    /// 关联函数：创建正方形
    /// 调用方式：Rectangle::square(3)，用 :: 而不是 .
    fn square(size: u32) -> Self {
        // Self 是当前类型的别名，这里等价于 Rectangle
        Self {
            width: size,
            height: size,
        }
    }

    /// 关联函数：普通构造函数
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // ⚠️ 方法 vs 关联函数 区别：
    //   方法：有 self/&self/&mut self 参数，用「.」调用（rect.area()）
    //   关联函数：无 self 参数，用「::」调用（Rectangle::square(3)）
    //   关联函数常用于构造函数
}

// ========================================================
// self 参数的三种形式（⭐ 重要）
// ========================================================
// | 形式             | 含义              | 能否改 self  | 何时用              |
// |-----------------|-------------------|-------------|---------------------|
// | &self           | 只读借用          | ❌          | 读数据（最常用）     |
// | &mut self       | 可变借用          | ✅          | 改数据              |
// | self            | 获取所有权（移动） | ✅（消耗）  | 转换/消耗 self       |
// | （无 self 参数）  | 关联函数          | —          | 构造函数等          |
//
// 💡 绝大多数情况用 &self（只读）或 &mut self（可写）
//    很少用 self（会消耗实例，实例调用后就失效了）

// ========================================================
// 可以写多个 impl 块（虽然不常见）
// ========================================================
// impl Rectangle {
//     fn another_method(&self) { ... }
// }
// 这是合法的，但通常把相关方法放一个 impl 块里更清晰
