use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Greet { },
    Ping {
        #[arg(long, default_value ="127.0.0.1")]
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

        #[arg(long)]
        output_file: Option<String>,
    },
    Download {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        #[arg(long, default_value_t = 5001)]
        port: u16,

        #[arg(long, default_value_t = 10.0)]
        duration: f64,

        #[arg(long, default_value_t = 65536)]
        buffer_size: usize,

        #[arg(long, default_value_t = 1.0)]
        sample_interval: f64,

        #[arg(long, default_value_t = 1000)]
        timeout_ms: u64,

        #[arg(long, default_value_t = 1)]
        num_streams: usize,

        #[arg(long)]
        output_file: Option<String>,
    },
    Upload {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        #[arg(long, default_value_t = 5001)]
        port: u16,

        #[arg(long, default_value_t = 10.0)]
        duration: f64,

        #[arg(long, default_value_t = 65536)]
        buffer_size: usize,

        #[arg(long, default_value_t = 1.0)]
        sample_interval: f64,
        
        #[arg(long, default_value_t = 1000)]
        timeout_ms: u64,

        #[arg(long, default_value_t = 1)]
        num_streams: usize,

        #[arg(long)]
        output_file: Option<String>,
    },
}
