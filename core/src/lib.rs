use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, time::{Duration, Instant}};

use pyo3::{prelude::*};

#[pyfunction]
fn hello_rust() -> PyResult<String> {
    Ok("Hail, my Lord! I am Rust speaking through Python.".to_string())
}

#[pyfunction]
fn run_download_test(host: String, port: u16, duration: u64) -> PyResult<f64> {
    // TODO: implement TCP download test
    Ok(123.45) // Mbps placeholder
}

#[pyfunction]
fn run_upload_test(host: String, port: u16, duration: u64) -> PyResult<f64> {
    // TODO: implement TCP upload test
    Ok(67.89) // Mbps placeholder
}

#[pyclass]
pub struct LatencyStats {
    #[pyo3(get)]
    pub avg_ms: f64,
    #[pyo3(get)]
    pub min_ms: f64,
    #[pyo3(get)]
    pub max_ms: f64,
    #[pyo3(get)]
    pub jitter_ms: f64,
    #[pyo3(get)]
    pub samples_ms: Vec<f64>,
}

fn compute_stats(samples_ms: &[f64]) -> (f64, f64, f64, f64) {
    let n = samples_ms.len() as f64;
    
    let min = samples_ms
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);

    let max = samples_ms
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let avg = samples_ms.iter().sum::<f64>() / n;

    let var = if samples_ms.len() > 1 {
        samples_ms.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / n
    } else {
        0.0
    };

    let stddev = var.sqrt();
    (avg, min, max, stddev)
}

#[pyfunction]
fn measure_latency(
    host: String,
    port: u16,
    count: u32,
    payload_size: usize,
    timeout_ms: u64,
    warmup: u32
) -> PyResult<LatencyStats> {
    let addr = format!("{}:{}", host, port);
    let mut addrs = addr
        .to_socket_addrs()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("resolve error: {e}")))?;
    let target = addrs
        .next()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("no address resolved"))?;

    let mut stream = TcpStream::connect(target)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("connect: {e}")))?;

    stream
        .set_nodelay(true)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("nodelay: {e}")))?;

    let timeout = Some(Duration::from_millis(timeout_ms));

    stream
        .set_read_timeout(timeout)
        .and_then(|_| stream.set_write_timeout(timeout))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("timeout: {e}")))?;

    let mut tx = vec![0u8; payload_size.max(8)];
    let mut rx = vec![0u8; tx.len()];

    let mut samples_ms: Vec<f64> = Vec::with_capacity(count as usize);
    
    let total = warmup + count;

    for i in 0..total {
        tx[0..8].copy_from_slice(&(i as u64).to_le_bytes());

        let t0 = Instant::now();
        stream.write_all(&tx).map_err(|e| PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("write: {e}")))?;
        stream.flush().ok();

        let mut read = 0usize;
        while read < rx.len() {
            let n = stream.read(&mut rx[read..]).map_err(|e| PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("read: {e}")))?;

            if n == 0 {
                return Err(PyErr::new::<pyo3::exceptions::PyConnectionError, _>("connection closed by peer"));
            }

            read += n;
        }

        let dt = t0.elapsed();
        if rx[0..8] != tx[0..8] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("echo mismatch"));
        }

        if i >= warmup {
            samples_ms.push(dt.as_secs_f64() * 1_000.0);
        }
    }

    let (avg, min, max, jitter) = compute_stats(&samples_ms);
    Ok(LatencyStats {
        avg_ms: avg,
        min_ms: min,
        max_ms: max,
        jitter_ms: jitter,
        samples_ms
    })

}

#[pymodule]
fn speedtest_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_rust, m)?)?;
    m.add_function(wrap_pyfunction!(run_download_test, m)?)?;
    m.add_function(wrap_pyfunction!(run_upload_test, m)?)?;
    m.add_function(wrap_pyfunction!(measure_latency, m)?)?;
    Ok(())
}
