// ===== minigrep 入口 =====
// main.rs 的职责：解析参数 → 调用 lib::run → 处理错误
// 标准的「分离关注点」结构

use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    // ① 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // ② 解析配置（可能失败）
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析参数出错: {err}");
        process::exit(1);
    });

    // ③ 执行核心逻辑（用闭包处理错误）
    if let Err(e) = run(&config) {
        println!("运行出错: {e}");
        process::exit(1);
    }
}
