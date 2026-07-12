// ===== 19.1 Unsafe Rust =====
// 用 cargo run --bin unsafe_t 运行

fn main() {
    println!("===== 1. 为什么需要 unsafe？=====\n");

    // Rust 默认保证内存安全，但有些场景需要绕过：
    //   1. 调用 C 语言的函数（FFI）
    //   2. 实现底层的数据结构（如 split_at_mut）
    //   3. 直接操作内存（裸指针）
    //   4. 访问/修改可变全局变量
    //
    // unsafe 块打开「五扇门」，让你做平时不让做的事

    println!("===== 2. 裸指针（裸 *const / *mut）=====\n");

    let mut num = 5;

    // 裸指针：不保证有效，不实现自动清理
    let r1 = &num as *const i32;      // 不可变裸指针
    let r2 = &mut num as *mut i32;    // 可变裸指针

    // 裸指针可以在 unsafe 里解引用
    unsafe {
        println!("r1 指向: {}", *r1);
        *r2 = 10;
        println!("r2 修改后: {}", *r2);
    }

    println!("\n===== 3. 调用 unsafe 函数 =====\n");

    // unsafe 函数必须在 unsafe 块里调用
    unsafe {
        dangerous();
    }

    println!("\n===== 4. ⭐ 用 unsafe 实现安全 API（split_at_mut）=====\n");

    // 经典例子：标准库的 split_at_mut 用 unsafe 实现
    // 回顾第 4 章问答：两个可变切片不能同时存在
    // 但 split_at_mut 用 unsafe 保证两个切片不重叠 → 安全

    let mut v = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut v, 2);
    println!("left: {:?}, right: {:?}", left, right);
    // ✅ 用户不需要写 unsafe，因为函数内部保证了安全

    println!("\n===== 5. 调用外部 C 函数（FFI）=====\n");

    // extern "C" 声明外部函数（Rust 2024 要求 unsafe extern）
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("abs(-5) = {}", abs(-5));   // 调用 C 标准库的 abs
    }

    // ⭐ FFI 是 unsafe 的主要原因：编译器无法验证外部代码的安全性

    println!("\n===== 6. 可变静态变量 =====\n");

    // static mut 需要在 unsafe 里访问（数据竞争风险）
    // 但通常应该用 Mutex 或 Atomic 代替

    println!("COUNTER 初始: {}", unsafe { COUNTER });
    unsafe {
        COUNTER += 1;
    }
    println!("COUNTER 修改后: {}", unsafe { COUNTER });

    println!("\n===== 7. unsafe 的五扇门 =====\n");

    println!("unsafe 块让你可以：");
    println!("  1. 解引用裸指针");
    println!("  2. 调用 unsafe 函数");
    println!("  3. 实现或访问 unsafe trait");
    println!("  4. 访问/修改 static mut");
    println!("  5. 访问 union 字段");
    println!();
    println!("⚠️ unsafe 不关闭借用检查器！也不关闭其他安全检查！");
    println!("   unsafe 只是打开上述「五扇门」，其他规则照常生效。");

    println!("\n===== 8. unsafe 的责任 =====\n");

    println!("unsafe 块 = 「我保证这操作是安全的，编译器无法验证的部分我来负责」");
    println!();
    println!("unsafe 不能保证：");
    println!("  - 裸指针有效（可能悬垂）");
    println!("  - 共享可变数据无竞争");
    println!("  - 外部函数行为正确");
    println!();
    println!("所以：尽量缩小 unsafe 块的范围，加注释说明为什么安全");
}

// ---------- unsafe 函数 ----------
unsafe fn dangerous() {
    println!("这是一个 unsafe 函数");
}

// ---------- ⭐ 用 unsafe 实现安全 API ----------
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();    // 裸指针

    // ⭐ 用 unsafe 拆分，但对外是安全 API
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
    // 这个函数是安全的，因为 mid 把切片分成不重叠的两段
    // 这是「用 unsafe 实现底层，对外暴露安全 API」的典范
}

// ---------- 可变静态变量 ----------
static mut COUNTER: i32 = 0;

// ========================================================
// ⭐ unsafe 的使用原则
// ========================================================
// 1. 尽量不用 unsafe（99% 的代码不需要）
// 2. 需要时，把 unsafe 隔离在小范围，封装成安全 API
// 3. 加注释说明「为什么这段 unsafe 是安全的」
// 4. 充分测试 unsafe 代码
//
// 标准库大量使用 unsafe（Vec、String、Mutex 等底层都用）
// 但对用户暴露的是安全 API——这就是「用 unsafe 实现安全」的模式
