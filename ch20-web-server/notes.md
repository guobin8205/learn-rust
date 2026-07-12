# 第 20 章：多线程 Web 服务器（终章）🎓

> 对应 the book 第 20 章
> 学习目标：综合运用前 19 章知识，构建一个多线程 Web 服务器

## 📂 项目结构

```
hello/
├── Cargo.toml
├── hello.html              ← 首页
├── 404.html                ← 404 页面
└── src/
    ├── lib.rs              ← 线程池（ThreadPool）
    └── main.rs             ← Web 服务器入口
```

## 运行方式
```bash
cd ch20-web-server/hello
cargo run
# 浏览器访问 http://127.0.0.1:7878
# 访问 http://127.0.0.1:7878/sleep 模拟慢请求（5 秒）
# 访问 http://127.0.0.1:7878/其他 触发 404
```

---

## 20.1 项目架构

```
浏览器请求
    ↓
TcpListener（监听 7878 端口）
    ↓ 每个连接
ThreadPool（4 个线程）── channel ──→ Worker 们
                                    ↓
                                handle_connection
                                    ↓
                                返回 HTTP 响应
```

### 三个核心组件

1. **TcpListener**：监听端口，接收连接
2. **ThreadPool**：4 个工作线程，并发处理
3. **handle_connection**：解析请求，返回响应

---

## 20.2 单线程版（基础）

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);   // ⚠️ 单线程：一个慢请求阻塞所有连接
}
```

### HTTP 响应格式
```
HTTP/1.1 200 OK\r\n
Content-Length: xxx\r\n
\r\n
<HTML 内容>
```

---

## 20.3 线程池（Thread Pool）⭐ 核心

### 为什么需要线程池？

- 单线程：一个慢请求阻塞所有连接
- 每请求一线程：连接多时创建/销毁线程开销大
- **线程池**：预创建固定数量的线程，复用处理任务 ✅

### 线程池设计

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,                    // 工作线程
    sender: Option<mpsc::Sender<Message>>,   // 任务发送端
}

enum Message {
    NewJob(Job),     // 新任务
    Terminate,       // 终止信号
}

type Job = Box<dyn FnOnce() + Send + 'static>;
```

### 核心机制：channel + Arc<Mutex>

```
main（生产者）           Workers（消费者）
    │                          │
    │ execute(task)            │ loop {
    ↓                          │   lock receiver
  sender ──── channel ───→ receiver
                             │   recv() → 任务
                             │   执行任务
                             │ }
```

### execute：发送任务

```rust
pub fn execute<F>(&self, f: F)
where F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
}
```

### Worker：循环取任务执行

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
```

### ⭐ 关键知识点综合

| 章节 | 知识点 | 在线程池中的应用 |
|------|--------|---------------|
| 第 4 章 | 所有权、move | `move ||` 把 receiver 移进线程 |
| 第 8 章 | Vec | `Vec<Worker>` 存工作线程 |
| 第 9 章 | 错误处理 | `unwrap()` 处理 lock/send 错误 |
| 第 15 章 | Arc、Mutex | `Arc<Mutex<Receiver>>` 共享接收端 |
| 第 16 章 | 线程、channel | `thread::spawn`、`mpsc::channel` |
| 第 17 章 | trait 对象 | `Box<dyn FnOnce() + Send>` |
| 第 18 章 | 模式匹配 | `match message { ... }` |

---

## 20.4 优雅关闭（Drop）

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());      // 1. 关闭发送端
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); // 2. 等待每个 worker 完成
            }
        }
    }
}
```

---

## 20.5 最终版 main.rs

```rust
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);   // 4 个线程

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);   // 任务交给线程池
        });
    }
}
```

---

## 📋 本项目的「集大成」特性

### 类型系统
- `Box<dyn FnOnce() + Send + 'static>`（trait 对象 + 多重约束）
- `Arc<Mutex<Receiver<Message>>>`（三层包装）
- `enum Message`（和类型）
- `type Job = ...`（类型别名）

### 并发
- `thread::spawn`（创建线程）
- `mpsc::channel`（线程间通信）
- `Arc<Mutex<T>>`（共享可变状态）
- `move` 闭包（跨线程转移所有权）

### 设计模式
- **RAII**（Drop 优雅关闭）
- **生产者-消费者**（channel）
- **线程池**（复用线程）
- **策略模式**（match 分支返回不同响应）

---

## 🎓 课程总结：你掌握了什么

学完 20 章，你已经掌握了 Rust 的完整体系：

### 语言核心
- ✅ 所有权、借用、生命周期（第 4、10 章）
- ✅ 枚举、模式匹配（第 6、18 章）
- ✅ 泛型、Trait（第 10、17、19 章）
- ✅ 错误处理（第 9 章）
- ✅ 闭包、迭代器（第 13 章）

### 工程实践
- ✅ 模块化组织（第 7 章）
- ✅ 测试驱动（第 11 章）
- ✅ Cargo 项目管理（第 14 章）
- ✅ 完整项目实战（第 12、20 章）

### 高级特性
- ✅ 智能指针（第 15 章）
- ✅ 无畏并发（第 16 章）
- ✅ OOP 特性（第 17 章）
- ✅ unsafe 与宏（第 19 章）

---

## ✅ 第 20 章（终章）小结

1. ✅ 从零构建多线程 Web 服务器
2. ✅ 用 ThreadPool 管理并发
3. ✅ 综合 Arc/Mutex/channel/线程
4. ✅ 实现 Drop 优雅关闭
5. ✅ 完成了一个真实可用的项目！

---

## 📂 本章练习目录

- `hello/src/lib.rs` —— 线程池实现
- `hello/src/main.rs` —— Web 服务器
- `hello/hello.html` —— 首页
- `hello/404.html` —— 404 页面
