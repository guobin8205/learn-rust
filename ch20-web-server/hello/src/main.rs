// ===== 第 20 章：多线程 Web 服务器（终章）=====
// 用 cargo run 运行
// 浏览器访问 http://127.0.0.1:7878
//
// ⭐ 综合运用前 19 章的所有知识：
//   第 4 章：所有权、move
//   第 8 章：String、文件 I/O
//   第 9 章：错误处理、?
//   第 15 章：Arc、Mutex
//   第 16 章：线程、channel、Send
//   第 18 章：模式匹配

use hello::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);      // ⭐ 4 个线程的线程池

    println!("多线程服务器启动：http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // ⭐ 把连接处理任务交给线程池
        pool.execute(|| {
            handle_connection(stream);   // stream 被 move 进闭包
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // ⭐ 用模式匹配（第 18 章）判断请求
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 模拟慢请求（演示多线程优势）
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = std::fs::read_to_string(filename).unwrap_or_else(|_| {
        format!("<html><body><h1>{}</h1></body></html>", status_line)
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
