mod commands;

use std::{
    fs::File,
    io::{Write, BufWriter, stdout},
};

use spinoff::{spinners, Spinner, Color};
use anyhow::Result;
use clap::Parser;

use commands::Commands;
use speedtest_core::{measure_latency, run_download_test, run_upload_test};

#[derive(Parser)]
#[command(name = "speedtest-cli", version = "0.1.0", about = "TCP Speedtest CLI in Rust")]
struct CLI {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

fn write_csv_header(file: &mut BufWriter<Box<dyn Write>>, headers: &[&str]) -> Result<()> {
    writeln!(file, "{}", headers.join(","))?;
    Ok(())
}

fn write_csv_row(file: &mut BufWriter<Box<dyn Write>>, values: &[String]) -> Result<()> {
    writeln!(file, "{}", values.join(","))?;
    Ok(())
}

/// helper: open writer to file or stdout
fn get_writer(path: &Option<String>) -> Result<BufWriter<Box<dyn Write>>> {
    match path {
        Some(p) => Ok(BufWriter::new(Box::new(File::create(p)?) as Box<dyn Write>)),
        None    => Ok(BufWriter::new(Box::new(stdout()) as Box<dyn Write>)),
    }
}

fn handle_ping(
    host: &str, port: u16, count: u32, payload_size: usize,
    timeout_ms: u64, warmup: u32, output_file: &Option<String>
) -> Result<()> {
    let mut spinner = Spinner::new(spinners::Dots, "measuring latency", Color::Cyan);

    let stats = measure_latency(host, port, count, payload_size, timeout_ms, warmup)?;
    spinner.success("latency test finished");

    let mut writer = get_writer(output_file)?;
    write_csv_header(&mut writer, &["host", "port", "avg_ms", "min_ms", "max_ms", "jitter_ms"])?;
    write_csv_row(
        &mut writer,
        &[
            host.to_string(),
            port.to_string(),
            format!("{:.2}", stats.avg_ms),
            format!("{:.2}", stats.min_ms),
            format!("{:.2}", stats.max_ms),
            format!("{:.2}", stats.jitter_ms),
        ],
    )?;

    Ok(())
}

fn handle_download(
    host: &str, port: u16, duration: f64, buffer_size: usize,
    sample_interval: f64, timeout_ms: u64, num_streams: usize,
    output_file: &Option<String>
) -> Result<()> {
    let mut spinner = Spinner::new(spinners::Dots, "measuring download speed", Color::Cyan);

    let samples = run_download_test(host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams)?;
    spinner.success("download test finished");

    let mut writer = get_writer(output_file)?;
    write_csv_header(&mut writer, &["seconds_elapsed", "mbps"])?;
    for sample in &samples {
        write_csv_row(&mut writer, &[
            format!("{:.1}", sample.seconds_elapsed),
            format!("{:.2}", sample.mbps),
        ])?;
    }

    Ok(())
}

fn handle_upload(
    host: &str, port: u16, duration: f64, buffer_size: usize,
    sample_interval: f64, timeout_ms: u64, num_streams: usize,
    output_file: &Option<String>
) -> Result<()> {
    let mut spinner = Spinner::new(spinners::Dots, "measuring upload speed", Color::Cyan);

    let samples = run_upload_test(host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams)?;
    spinner.success("upload test finished");

    let mut writer = get_writer(output_file)?;
    write_csv_header(&mut writer, &["seconds_elapsed", "mbps"])?;
    for sample in &samples {
        write_csv_row(&mut writer, &[
            format!("{:.1}", sample.seconds_elapsed),
            format!("{:.2}", sample.mbps),
        ])?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    
    match cli.command {
        Some(Commands::Greet {}) 
            => println!("Hello from speedtest-cli!"),
        Some(Commands::Ping { host, port, count, payload_size, timeout_ms, warmup, output_file }) 
            => handle_ping(&host, port, count, payload_size, timeout_ms, warmup, &output_file)?,
        Some(Commands::Download { host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams, output_file }) 
            => handle_download(&host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams, &output_file)?,
        Some(Commands::Upload { host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams, output_file })
            => handle_upload(&host, port, duration, buffer_size, sample_interval, timeout_ms, num_streams, &output_file)?,
        None => ()
    }

    Ok(())
}
