// ===== 线程池（Thread Pool）=====
// 这是第 20 章的核心：用第 16 章的并发知识构建一个线程池

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// ========================================================
// Type 别名：Message 是线程接收的任务消息
// ========================================================

// ⭐ 用 Box<dyn FnOnce() + Send + 'static>
//   - FnOnce：闭包只执行一次（消费任务）
//   - Send：可以跨线程转移
//   - 'static：没有借用（线程可能活得比创建者久）
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),    // 新任务
    Terminate,      // 终止信号
}

// ========================================================
// ThreadPool：线程池
// ========================================================

pub struct ThreadPool {
    workers: Vec<Worker>,               // 工作线程
    sender: Option<mpsc::Sender<Message>>,   // 任务发送端（Option 用于优雅关闭）
}

impl ThreadPool {
    /// 创建线程池，size 是线程数量
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "线程池大小必须大于 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));   // ⭐ Arc<Mutex> 共享接收端

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// 执行任务（发送任务到 channel）
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // ⭐ 优雅关闭：发送 Terminate 给每个 worker
        // 每个 worker 收到 Terminate 就退出循环

        // 1. 先 drop sender（关闭 channel 的发送端）
        //    这会让 receiver 在任务处理完后收到错误
        drop(self.sender.take());

        // 2. 等待每个 worker 完成
        for worker in &mut self.workers {
            println!("关闭 worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// ========================================================
// Worker：工作线程
// ========================================================

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // ⭐ 关键：lock 获取任务，处理完释放锁，再循环
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("worker {} 执行任务", id);
                    job();
                }
                Message::Terminate => {
                    println!("worker {} 终止", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
