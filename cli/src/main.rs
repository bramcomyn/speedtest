use clap::{
    Parser, 
    Subcommand
};

use speedtest_core::{
    measure_latency
};

#[derive(Parser)]
#[command(name = "speedtest-cli", version = "0.1.0", about = "TCP Speedtest CLI in Rust", long_about = None)]
struct CLI {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Greet {

    },
    Ping {
        #[arg(long, default_value_t = String::from("127.0.0.1"))]
        host: String,
        
        #[arg(long, default_value_t = 5001)]
        port: u16,

        #[arg(long, default_value_t = 20)]
        count: u32,
        
        #[arg(long, default_value_t = 32)]
        payload_size: usize,
        
        #[arg(long, default_value_t = 1000)]
        timeout_ms: u64,
        
        #[arg(long, default_value_t = 2)]
        warmup: u32,
    }
}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Some(Commands::Greet {}) => {
            println!("Hello from speedtest-cli!");
        }
        Some(Commands::Ping { host, port, count, payload_size, timeout_ms, warmup }) => {
            match measure_latency(
                &host, *port, *count, *payload_size, *timeout_ms, *warmup
            ) {
                Ok(stats) => {
                    println!("Latency results for {}:{}", host, port);
                    println!("  Avg: {:.2} ms", stats.avg_ms);
                    println!("  Min: {:.2} ms", stats.min_ms);
                    println!("  Max: {:.2} ms", stats.max_ms);
                    println!("  Jitter: {:.2} ms", stats.jitter_ms);
                }
                Err(e) => eprintln!("Error measuring latency: {}", e)
            }
        }
        None => {}
    }
}
