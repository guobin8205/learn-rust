// 这个文件用于演示 shadowing 的内存行为，帮助你理解
// 可以单独看，不需要运行

fn main() {
    // ===== 例子 1：shadowing 不等于赋值覆盖 =====
    let x = 5;
    let x = x + 1;      // shadowing：用 x 的旧值 5 算出新值 6，创建新变量 x=6
    let x = x * 2;      // 再次 shadowing：用 x 的当前值 6 算出 12，创建新变量 x=12
    // 现在 x 是 12
    // 过程中出现过三个不同的 x（值分别是 5、6、12），它们是三个不同的变量

    // ===== 例子 2：shadowing 可以改变类型 =====
    let guess = String::new();   // guess 是 String 类型
    let guess: u32 = 42;         // shadowing：guess 现在变成 u32 类型（这用 mut 做不到！）

    // ===== 例子 3：原变量何时释放？=====
    {
        let s1 = String::from("hello");  // 在堆上分配 "hello"
        let s1 = String::from("world");  // shadowing：新建另一个 String

        // ⚠️ 注意：第一个 "hello" 的 String 并没有在这里立即释放！
        //    它要等到这个作用域（{}）结束才被 drop（释放）
        //    虽然名字 s1 已经指向 "world"，但旧的 "hello" 仍占用内存直到作用域结束
    } // ← 这里作用域结束，两个 String 都被 drop（先 drop 后声明的 "world"，再 drop "hello"）
}
