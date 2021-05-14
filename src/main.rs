#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate async_recursion;
extern crate tokio;

mod config;
mod errors;
mod models;
mod services;

use anyhow::Result;
use config::CONFIG;
use dotenv::dotenv;
use models::s3_client::S3Client;
use services::S3BucketDownloader;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let client = S3Client::new(&CONFIG.aws_default_region, &CONFIG.aws_url);
    let downloader = S3BucketDownloader::new(client);
    downloader
        .download(
            &config::CONFIG.aws_bucket,
            &config::CONFIG.aws_objects_prefix,
            &config::CONFIG.download_path,
        )
        .await
}
