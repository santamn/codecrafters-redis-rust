// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                loop {
                    let mut buf = [0; 1024];
                    stream.read(&mut buf)?;
                    print!("server receive: ");
                    print_buffer(&buf);

                    match stream.write_all("+PONG\r\n".as_bytes()) {
                        Ok(_) => println!("response succeeded"),
                        Err(e) => println!("response failed: {}", e),
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn print_buffer(buf: &[u8]) {
    let mut result = Vec::<char>::with_capacity(buf.len());
    let mut iter = buf.windows(2).into_iter();
    loop {
        match iter.next() {
            Some(s) => {
                if s[0] == b'\r' && s[1] == b'\n' {
                    result.extend(['[', 'C', 'R', 'L', 'F', ']']);
                    iter.next();
                } else {
                    result.push(char::try_from(s[0]).unwrap_or('.'));
                }
            }
            None => break,
        }
    }

    println!("{}", result.into_iter().collect::<String>());
}
