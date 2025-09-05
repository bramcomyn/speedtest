use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::{Duration, Instant},
};

pub struct LatencyStats {
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub jitter_ms: f64,
    pub samples_ms: Vec<f64>,
}

pub fn measure_latency(
    host: &str,
    port: u16,
    count: u32,
    payload_size: usize,
    timeout_ms: u64,
    warmup: u32,
) -> std::io::Result<LatencyStats> {
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

    stream.write_all(&[0u8; 1])?;
    stream.flush()?;

    let mut tx = vec![0u8; payload_size.max(8)];
    let mut rx = vec![0u8; tx.len()];

    let mut samples_ms = Vec::with_capacity(count as usize);

    for i in 0..(warmup + count) {
        tx[..8].copy_from_slice(&(i as u64).to_le_bytes());

        let t0 = Instant::now();
        stream.write_all(&tx)?;
        stream.flush()?;

        stream.read_exact(&mut rx)?;

        if rx[..8] != tx[..8] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "echo mismatch",
            ));
        }

        if i >= warmup {
            samples_ms.push(t0.elapsed().as_secs_f64() * 1_000.0);
        }
    }

    let (avg, min, max, jitter) = compute_stats(&samples_ms);

    Ok(LatencyStats {
        avg_ms: avg,
        min_ms: min,
        max_ms: max,
        jitter_ms: jitter,
        samples_ms,
    })
}

fn compute_stats(samples_ms: &[f64]) -> (f64, f64, f64, f64) {
    if samples_ms.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let n = samples_ms.len() as f64;
    let min = samples_ms.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = samples_ms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg = samples_ms.iter().sum::<f64>() / n;

    // RFC 5481: mean absolute delta of consecutive samples
    let mean_jitter = if samples_ms.len() > 1 {
        samples_ms.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f64>() 
            / (samples_ms.len() - 1) as f64
    } else {
        0.0
    };

    (avg, min, max, mean_jitter)
}
