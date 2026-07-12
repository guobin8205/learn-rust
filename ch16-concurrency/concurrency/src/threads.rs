// ===== 16.1 使用线程 =====
// 用 cargo run --bin threads 运行

use std::thread;
use std::time::Duration;

fn main() {
    println!("===== 1. 创建线程（thread::spawn）=====\n");

    // thread::spawn 接收一个闭包，在新线程执行
    thread::spawn(|| {
        for i in 1..5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ⚠️ 主线程结束时，子线程会被强制终止（不管有没有跑完）
    for i in 1..3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 所以子线程可能没跑完，主线程就结束了

    println!("\n===== 2. join：等待子线程完成 =====\n");

    // thread::spawn 返回 JoinHandle，调用 .join() 会阻塞等待子线程
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("子线程 2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        "子线程完成"   // 返回值
    });

    // handle.join() 阻塞，直到子线程结束
    let result = handle.join().unwrap();
    println!("join 结果: {}", result);
    // 现在保证子线程跑完了

    println!("\n===== 3. ⭐ move 闭包：跨线程转移所有权 =====\n");

    // 线程闭包默认「借用」外部变量，但线程生命周期可能比函数长
    // 所以跨线程传数据必须用 move（转移所有权）
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {   // ⭐ move 把 data 移进线程
        println!("线程内访问 data: {:?}", data);
        data.len()
    });

    // println!("{:?}", data);   // ❌ data 已被 move 到子线程
    println!("线程返回: {}", handle.join().unwrap());

    println!("\n===== 4. 线程返回值 =====\n");

    let handles: Vec<_> = (1..=3).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            format!("任务 {} 完成", i)
        })
    }).collect();

    for handle in handles {
        println!("{}", handle.join().unwrap());
    }

    println!("\n===== 5. build_thread：自定义线程属性 =====\n");

    use std::thread::Builder;

    // 自定义线程名和栈大小
    let child = Builder::new()
        .name("worker-1".to_string())
        .stack_size(4 * 1024 * 1024)   // 4MB 栈
        .spawn(|| {
            println!("自定义线程: {:?}", thread::current().name());
        })
        .unwrap();
    child.join().unwrap();

    println!("\n===== 6. 主线程 vs 子线程 =====\n");

    println!("当前线程 ID: {:?}", thread::current().id());
    println!("是否主线程: {}", thread::current().name().unwrap_or("unnamed"));

    // ========================================================
    // ⭐ 线程核心要点
    // ========================================================
    // 1. thread::spawn(|| {}) 创建线程，返回 JoinHandle
    // 2. handle.join().unwrap() 等待子线程完成
    // 3. 跨线程传数据必须用 move 闭包
    // 4. 主线程结束 = 整个进程结束（子线程被强制终止）
    //
    // | 问题                    | 答案               |
    // |-------------------------|--------------------|
    // | 如何创建线程？            | thread::spawn      |
    // | 如何等待子线程？           | handle.join()      |
    // | 如何跨线程传数据？          | move 闭包           |
    // | 主线程结束会怎样？          | 子线程被终止         |
}
