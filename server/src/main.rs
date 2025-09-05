use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, time::{Duration, Instant}};

async fn handle(mut socket: TcpStream) -> tokio::io::Result<()> {
    let mut buf = [0u8; 64 * 1024];
    socket.read_exact(&mut buf[0..1])?;

    loop {
        match buf[0] {
            0 => { // ping
                let n = socket.read(&mut buf[1..])?;
                if n == 0 { break ; }
                socket.write_all(&buf[1..n+1])?;
                socket.flush()?;
            },
            1 => { // download
                let start = Instant::now();
                let duration = Duration::from_secs(10);

                while start.elapsed() < duration {
                    socket.write_all(&buf)?;
                }
            },
            2 => { // upload
                let start = Instant::now();
                let duration = Duration::from_secs(10);

                while start.elapsed() < duration {
                    let _ = socket.read_exact(&mut buf)?;
                }
            },
            _ => break,
        }
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
