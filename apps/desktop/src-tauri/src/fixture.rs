//! 本地样例页 HTTP 服务（127.0.0.1 随机端口，仅本机回环）。
//!
//! 用途：为首个 change 提供一个可重跑的本地普通页面样例（E01–E04、E13）。
//! 不启动外部服务、不写盘、不监听非回环地址；应用退出随进程终止。

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const INDEX_HTML: &str = include_str!("../fixtures/sample-page.html");
const SUB_HTML: &str = include_str!("../fixtures/sub-page.html");

/// 在 127.0.0.1 的随机空闲端口启动样例服务，返回 `http://127.0.0.1:<port>` 基址。
/// 绑定失败（如被安全策略拦截）时返回 Err，由调用方明确报错，不静默降级。
pub fn start() -> std::io::Result<String> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let addr = listener.local_addr()?.to_string();
    thread::Builder::new()
        .name("layress-fixture-http".into())
        .spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => handle(s),
                    // 单个连接失败不影响服务
                    Err(_) => continue,
                }
            }
        })
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(format!("http://{addr}"))
}

fn handle(mut stream: TcpStream) {
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    // 只取 "GET <path> HTTP/1.x" 的 path；头与体直接丢弃（夹具无请求体语义）
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string();
    // 丢弃剩余请求头直到空行，避免残留在缓冲区
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) if line == "\r\n" || line == "\n" => break,
            Ok(_) => {}
        }
    }

    let response = match path.as_str() {
        "/" | "/index.html" => ok_html(INDEX_HTML),
        "/sub" => ok_html(SUB_HTML),
        "/download" => download_response(),
        _ => not_found(),
    };
    let _ = stream.write_all(&response);
    let _ = stream.flush();
}

fn ok_html(body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {len}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n{body}",
        len = body.len()
    )
    .into_bytes()
}

fn download_response() -> Vec<u8> {
    let body = "Layress 下载触发样例（E13）：此请求应被工具明确拒绝，不应写入任何下载目录。\n";
    // 必须是 WKWebView 无法显示的类型：text/plain 会被当作普通页面直接渲染，
    // 根本不进入 wry 的下载 delegate ⇒ on_download 不触发，负例测不到「明确拒绝」。
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {len}\r\nContent-Disposition: attachment; filename=\"layress-fixture.txt\"\r\nConnection: close\r\n\r\n{body}",
        len = body.len()
    )
    .into_bytes()
}

fn not_found() -> Vec<u8> {
    let body = "404 not found";
    format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n{body}",
        len = body.len()
    )
    .into_bytes()
}
