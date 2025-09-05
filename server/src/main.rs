use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream, Shutdown},
    thread,
};

use chrono::Local;
use log::info;

fn handle(mut socket: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 64 * 1024];
    socket.read_exact(&mut buf[0..1])?;

    match buf[0] {
        0 => loop {
            let n = socket.read(&mut buf[1..])?;
            if n == 0 {
                break;
            }
            socket.write_all(&buf[1..=n])?;
            socket.flush()?;
        },
        1 => loop {
            if socket.write_all(&buf).is_err() {
                break;
            }
        },
        2 => loop {
            if socket.read_exact(&mut buf).is_err() {
                break;
            }
        },
        _ => {}
    }

    socket.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let addr = "0.0.0.0:5001";
    info!("Echo server listening on {addr}");
    let listener = TcpListener::bind(addr)?;

    loop {
        let (sock, peer) = listener.accept()?;
        let timestamp = Local::now();
        info!("Accepted connection from {peer} ({timestamp})");
        thread::spawn(move || {
            if let Err(e) = handle(sock) {
                eprintln!("connection error: {e}");
            }
        });
    }
}
