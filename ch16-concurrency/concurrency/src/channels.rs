// ===== 16.2 消息传递（channel）=====
// 用 cargo run --bin channels 运行

use std::sync::mpsc;    // mpsc = Multiple Producer, Single Consumer
use std::thread;
use std::time::Duration;

fn main() {
    println!("===== 1. 基本消息传递 =====\n");

    // channel：单向通信管道
    // tx = transmitter（发送端）
    // rx = receiver（接收端）
    let (tx, rx) = mpsc::channel();

    // 发送端 move 给子线程
    thread::spawn(move || {
        let val = String::from("来自子线程的问候");
        tx.send(val).unwrap();    // 发送消息
        // ⚠️ send 后 val 的所有权转移到了 channel
        // println!("{}", val);   // ❌ val 已被 move
    });

    // 接收端在主线程
    let received = rx.recv().unwrap();   // 阻塞等待消息
    println!("收到: {}", received);

    println!("\n===== 2. 多条消息 + 迭代器 =====\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("任务 1"),
            String::from("任务 2"),
            String::from("任务 3"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));   // 模拟工作
        }
    });

    // 用迭代器接收（非阻塞，channel 关闭时停止）
    for received in rx {    // rx 实现了 Iterator
        println!("收到: {}", received);
    }
    // ↑ 当所有发送端 drop 后，rx 迭代结束

    println!("\n===== 3. 多生产者（Multiple Producer）=====\n");

    // mpsc 的 m = 多个发送端
    let (tx, rx) = mpsc::channel();

    // tx.clone() 可以创建多个发送端
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
        let msgs = vec!["A1".to_string(), "A2".to_string()];
        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    let handle2 = thread::spawn(move || {
        let msgs = vec!["B1".to_string(), "B2".to_string()];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(80));
        }
    });

    // 主线程接收（注意：必须 drop 原始 tx，否则 rx 永远不结束）
    drop(tx);    // ⚠️ 关键！drop 原始 tx，否则 channel 不会关闭

    for received in rx {
        println!("收到: {}", received);
    }
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("\n===== 4. 同步 vs 异步 channel =====\n");

    // 默认是「异步 channel」：发送方不等接收方，消息排队
    // 用 sync_channel 创建「同步 channel」：缓冲区满时发送方阻塞

    let (sync_tx, sync_rx) = mpsc::sync_channel(2);   // 缓冲区大小 2

    thread::spawn(move || {
        for i in 0..5 {
            sync_tx.send(i).unwrap();
            println!("发送: {}", i);
        }
    });

    thread::sleep(Duration::from_millis(100));
    // 此时缓冲区应该已满（容量 2），子线程阻塞
    println!("（子线程可能因缓冲区满而阻塞）");

    while let Ok(msg) = sync_rx.recv() {
        println!("接收: {}", msg);
        thread::sleep(Duration::from_millis(50));
    }

    // ========================================================
    // ⭐ 消息传递 vs 共享状态（下一节对比）
    // ========================================================
    // | 方式         | 优点             | 缺点             |
    // |-------------|-----------------|-----------------|
    // | 消息传递      | 简单、无数据竞争   | 单向、有拷贝开销  |
    // | 共享状态      | 直接访问         | 需要锁、复杂      |
    //
    // Go 的哲学：「不要通过共享内存通信，而要通过通信共享内存」
    // Rust 两种都支持，但消息传递通常更安全
}
