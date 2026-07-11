use rand::RngExt; // rand 0.10 中，random_range 方法在 RngExt 这个 trait 里
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！");

    // 生成一个 1..=100 的随机数
    let secret_number = rand::rng().random_range(1..=100);

    // 反复让用户猜，直到猜对
    loop {
        println!("请猜一个 1 到 100 之间的数字：");

        // 存放用户输入的字符串
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // 把用户输入的字符串转换成数字
        // 用 shadowing（隐藏）复用 guess 这个名字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字！");
                continue; // 输入非法，跳过本次循环，重新让用户猜
            }
        };

        println!("你猜的数字是：{guess}");

        // 比较猜的数字和目标数字
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),    // 猜的 < 目标
            Ordering::Greater => println!("太大了！"), // 猜的 > 目标
            Ordering::Equal => {
                println!("🎉 猜对了！");
                break; // 猜对了，退出循环
            }
        }
    }
}
