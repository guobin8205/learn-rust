// ===== 10.1 泛型 (Generics) =====
// 用 cargo run --bin generics_t 运行

// ========================================================
// 一、为什么需要泛型？
// ========================================================
// 没有泛型，每种类型都要写一个函数：

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// ↑ 两个函数逻辑完全一样，只是类型不同！太重复了

// ========================================================
// 二、泛型函数：用 <T> 表示「任意类型」
// ========================================================

// 问题：不是所有类型都能比较大小（>），需要加约束（下一节 Trait 讲）
// 这里先用 PartialOrd 约束（表示「能比较大小」）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// ✅ 一个函数搞定所有「能比较大小」的类型！

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("最大 i32: {}", largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大 char: {}", largest(&chars));

    // ========================================================
    // 三、泛型结构体
    // ========================================================

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}", int_point);
    println!("浮点点: {:?}", float_point);

    // 两个不同类型的泛型
    let mixed = Point2 { x: 5, y: 4.0 };
    println!("混合点: {:?}", mixed);

    // ========================================================
    // 四、泛型方法
    // ========================================================
    let p = Point { x: 5, y: 10 };
    println!("x = {}", p.x());
    println!("只拿 x: {:?}", p.into_x());

    // ========================================================
    // 五、泛型枚举（你已经在用了！）
    // ========================================================
    // Option<T>、Result<T,E>、Vec<T> 都是泛型枚举
    let opt: Option<i32> = Some(5);
    let res: Result<String, i32> = Ok(String::from("hi"));
    println!("泛型枚举: {:?}, {:?}", opt, res);

    // ========================================================
    // ⭐ 泛型的性能：零成本抽象
    // ========================================================
    // 泛型在编译时「单态化 (monomorphization)」：
    //   编译器为每个具体类型生成一份专用代码
    //
    //   largest::<i32>  → 生成 largest_i32 版本
    //   largest::<char> → 生成 largest_char 版本
    //
    // 所以泛型运行时和手写特定类型一样快，没有性能损失！
    println!("\n泛型通过单态化实现零成本抽象");
}

// ========================================================
// 泛型结构体定义
// ========================================================
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,       // x 和 y 必须是相同类型 T
}

// 两个不同类型参数（用逗号分隔）
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,       // x 和 y 可以是不同类型
}

// ========================================================
// 泛型方法
// ========================================================
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn into_x(self) -> T {
        self.x
    }
}

// 也可以只为特定类型实现方法（i32 的 Point 才有 distance）
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
