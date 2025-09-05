use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::{Duration, Instant},
};

use kdam::{tqdm, BarExt};

pub struct DownloadSample {
    pub seconds_elapsed: f64,
    pub mbps: f64,
}

pub fn run_download_test(
    host: &str,
    port: u16,
    duration_sec: f64,
    buffer_size: usize,
    sample_interval: f64,
    timeout_ms: u64,
    _num_streams: usize
) -> std::io::Result<Vec<DownloadSample>> {
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

    stream.write_all(&[1u8]).unwrap();
    stream.flush().unwrap();

    let mut buf = vec![0u8; buffer_size];
    let mut total_bytes = 0;
    let mut next_sample = sample_interval;

    let mut results = Vec::new();

    let mut progress = tqdm!(total = (duration_sec * 1_000.0).floor() as usize, desc = "download");
    let mut last_elapsed = 0.0;
    
    let start = Instant::now();
    while start.elapsed().as_secs_f64() < duration_sec {
        let n = stream.read(&mut buf).unwrap();
        if n == 0 { break ; }
        total_bytes += n;

        let elapsed = start.elapsed().as_secs_f64();
        progress.update(((elapsed - last_elapsed) * 1_000.0).floor() as usize)?;
        last_elapsed = elapsed;

        if elapsed > next_sample {
            let mbps = total_bytes as f64 * 8.0 / 1_000_000.0 / elapsed;
            results.push(DownloadSample { seconds_elapsed: elapsed, mbps });
            next_sample += sample_interval
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let mbps = total_bytes as f64 * 8.0 / 1_000_000.0 / elapsed;
    results.push(DownloadSample { seconds_elapsed: elapsed, mbps: mbps });
    Ok(results)
}
