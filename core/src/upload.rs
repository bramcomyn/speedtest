use std::{
    io::Write,
    net::{TcpStream, ToSocketAddrs, Shutdown},
    time::{Duration, Instant},
};

pub struct UploadSample {
    pub seconds_elapsed: f64,
    pub mbps: f64,
}

pub fn run_upload_test(
    host: &str,
    port: u16,
    duration_sec: f64,
    buffer_size: usize,
    sample_interval: f64,
    timeout_ms: u64,
    _num_streams: usize,
) -> std::io::Result<Vec<UploadSample>> {
    let addr = format!("{}:{}", host, port);
    let target = addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no address resolved"))?;

    let mut stream = TcpStream::connect(target)?;
    stream.set_nodelay(true)?;

    let timeout = Some(Duration::from_millis(timeout_ms));
    stream.set_read_timeout(timeout)?;
    stream.set_write_timeout(timeout)?;

    stream.write_all(&[2u8])?;
    stream.flush()?;

    let buf = vec![0u8; buffer_size];
    let mut total_bytes = 0usize;
    let mut next_sample = sample_interval;
    let mut results = Vec::new();

    let start = Instant::now();
    while start.elapsed().as_secs_f64() < duration_sec {
        stream.write_all(&buf)?;
        total_bytes += buf.len();
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed >= next_sample {
            let mbps = total_bytes as f64 * 8.0 / 1_000_000.0 / elapsed;
            results.push(UploadSample { seconds_elapsed: elapsed, mbps });
            next_sample += sample_interval;
        }
    }

    stream.shutdown(Shutdown::Both)?;

    let elapsed = start.elapsed().as_secs_f64();
    if elapsed > 0.0 {
        let mbps = total_bytes as f64 * 8.0 / 1_000_000.0 / elapsed;
        results.push(UploadSample { seconds_elapsed: elapsed, mbps });
    }

    Ok(results)
}
