mod commands;

use clap::Parser;
use commands::Commands;
use speedtest_core::{
    measure_latency,
    run_download_test,
    run_upload_test
};

#[derive(Parser)]
#[command(name = "speedtest-cli", version = "0.1.0", about = "TCP Speedtest CLI in Rust", long_about = None)]
struct CLI {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Some(Commands::Greet {}) => {
            println!("Hello from speedtest-cli!");
        },
        Some(Commands::Ping { host, port, count, payload_size, timeout_ms, warmup }) => {
            println!("Starting latency test...");

            if let Ok(stats) = measure_latency(&host, *port, *count, *payload_size, *timeout_ms, *warmup) {
                println!("Latency results for {}:{}", host, port);
                println!("  Avg: {:.2} ms", stats.avg_ms);
                println!("  Min: {:.2} ms", stats.min_ms);
                println!("  Max: {:.2} ms", stats.max_ms);
                println!("  Jitter: {:.2} ms", stats.jitter_ms);
            }
        },
        Some(Commands::Download { host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams }) => {
            println!("Starting download test...");

            if let Ok(samples) = run_download_test(host, *port, *duration, *buffer_size, *sample_interval, *timeout_ms, *num_streams) {
                for sample in &samples {
                    println!("t = {:.1}s: {:.2} Mbps", sample.seconds_elapsed, sample.mbps);
                }

                println!("Final download speed: {:.2} Mbps", samples.last().unwrap().mbps);
            }
        },
        Some(Commands::Upload { host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams }) => {
            println!("Starting upload test...");
            
            if let Ok(samples) = run_upload_test(host, *port, *duration, *buffer_size, *sample_interval, *timeout_ms, *num_streams) {
                for sample in &samples {
                    println!("t = {:.1}s: {:.2} Mbps", sample.seconds_elapsed, sample.mbps);
                }

                println!("Final download speed: {:.2} Mbps", samples.last().unwrap().mbps);
            }
        },
        None => {}
    }
}
