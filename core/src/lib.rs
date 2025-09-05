mod latency;
mod download;
mod upload;

pub use latency::{LatencyStats, measure_latency};
pub use download::{DownloadSample, run_download_test};
pub use upload::{UploadSample, run_upload_test};
