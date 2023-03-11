// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buf = [0; 256];
                loop {
                    stream.read(&mut buf)?;
                    print!("server receive: ");
                    print_buffer(&buf);

                    match stream.write_all("+PONG\r\n".as_bytes()) {
                        Ok(_) => println!("response succeeded"),
                        Err(_) => println!("response failed"),
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
    for s in buf.windows(2) {
        if s[0] == b'\r' && s[1] == b'\n' {
            break;
        }

        print!("{}", char::try_from(s[0]).unwrap_or('.'))
    }
    println!()
}
