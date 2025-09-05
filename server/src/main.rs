use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle(mut socket: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 64 * 1024];
    socket.read_exact(&mut buf[0..1])?;

    loop {
        match buf[0] {
            0 => {
                let n = socket.read(&mut buf[1..])?;
                if n == 0 {
                    break;
                }
                socket.write_all(&buf[1..n + 1])?;
                socket.flush()?;
            }
            1 => {
                loop {
                    if let Err(_e) = socket.write_all(&buf) {
                        return Ok(());
                    }
                }
            }
            2 => {
                loop {
                    if let Err(_e) = socket.read_exact(&mut buf) {
                        return Ok(());
                    }
                }
            }
            _ => break,
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:5001";
    println!("Echo server listening on {addr}");
    let listener = TcpListener::bind(addr)?;
    loop {
        let (sock, peer) = listener.accept()?;
        println!("Accepted connection from {peer}");
        thread::spawn(move || {
            if let Err(e) = handle(sock) {
                eprintln!("connection error: {e}");
            }
        });
    }
}
