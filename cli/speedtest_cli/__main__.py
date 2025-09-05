import click
from speedtest_core import run_download_test, run_upload_test, measure_latency, hello_rust # type: ignore

@click.group()
def cli():
    """Speedtest CLI (TCP)"""
    pass

@cli.command()
def download():
    speed = run_download_test("192.168.1.42", 5001, 10)
    click.echo(f"Download speed: {speed:.2f} Mbps")

@cli.command()
def upload():
    speed = run_upload_test("192.168.1.42", 5001, 10)
    click.echo(f"Upload speed: {speed:.2f} Mbps")

@cli.command()
@click.option("--host", default="127.0.0.1", show_default=True)
@click.option("--port", default=5001, type=int, show_default=True)
@click.option("--count", default=20, type=int, show_default=True, help="Measured pings (excludes warmup).")
@click.option("--payload", "payload_size", default=32, type=int, show_default=True, help="Bytes per ping.")
@click.option("--timeout-ms", default=1000, type=int, show_default=True)
@click.option("--warmup", default=2, type=int, show_default=True)
def ping(host, port, count, payload_size, timeout_ms, warmup):
    """Measure TCP round-trip latency & jitter against an echo server."""
    stats = measure_latency(host, port, count, payload_size, timeout_ms, warmup)
    click.echo(f"Host: {host}:{port}")
    click.echo(f"Samples: {len(stats.samples_ms)}  Payload: {payload_size} B  Warmup: {warmup}")
    click.echo(f"Latency avg/min/max: {stats.avg_ms:.2f}/{stats.min_ms:.2f}/{stats.max_ms:.2f} ms")
    click.echo(f"Jitter (stdev): {stats.jitter_ms:.2f} ms")

@cli.command()
def hello():
    message = hello_rust()
    click.echo(message)

if __name__ == "__main__":
    cli()
