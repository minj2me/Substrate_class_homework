use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn main() {
    //选择监听地址 127.0.0.1:3333
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    println!("server listening on port 3333");
    //等待发来的请求
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //分配线程去处理每一个连接
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("connection failure:{}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("new connection:{}", stream.peer_addr().unwrap());
    //从边接中顺序读取1024字节数据
    let mut data = [0 as u8; 1024];
    loop {
        //read()返回:io::Result<usize>，使用match去处理
        match stream.read(&mut data){
            Ok(received_data_size)=>{
                //打印请求的客户端的信息
                //使用from_utf8_lossy, 当其遇到无效的 UTF-8 序列时的行为时也可以处理
                println!("Request: {}", String::from_utf8_lossy(&data[0..received_data_size]));
                //给client响应的内容
                let status_line = "HTTP/1.1 200 OK";//状态行，返回200 ok
                let contents = "<html><head><title>Hi</title><body>This is response from server</body></head></html>";
                /*
                使用format!组装响应内容， 格式如下:
                HTTP/1.1 200 OK
                Content-Length: x
                <html><head><title>Hi</title><body>This is response from server</body></head></html>
                */
                let response = format!(
                    "{}\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    contents.len(),
                    contents
                );
                //把response的字节发送给连接
                stream.write(response.as_bytes()).unwrap();
                //调用flush()等待并阻塞程序执行直到所有字节都被写入连接中
                stream.flush().unwrap();
            }
            Err(_)=>{
                println!("An error occurred, stopping connection with:{}", stream.peer_addr().unwrap());
                //Shuts down the read, write
                stream.shutdown(Shutdown::Both).unwrap()
            }
        }
    }
}
