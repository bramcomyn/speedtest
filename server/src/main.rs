use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

async fn handle(mut sock: TcpStream) -> tokio::io::Result<()> {
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = sock.read(&mut buf)?;
        if n == 0 {
            break;
        }
        sock.write_all(&buf[..n])?;
        sock.flush()?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr = "0.0.0.0:5001";
    println!("Echo server listening on {addr}");
    let listener = TcpListener::bind(addr)?;
    loop {
        let (sock, peer) = listener.accept()?;
        println!("Accepted connection from {peer}");
        tokio::spawn(async move {
            if let Err(e) = handle(sock).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}
