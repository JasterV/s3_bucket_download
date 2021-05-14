use crate::{errors::AppError::*, models::s3_client::S3Client};
use anyhow::Result;
use rusoto_s3::StreamingBody;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct S3BucketDownloader {
    client: S3Client,
}

impl S3BucketDownloader {
    pub fn new(client: S3Client) -> Self {
        S3BucketDownloader { client }
    }

    pub async fn download(
        &self,
        bucket_name: &str,
        prefix: &Option<String>,
        out_path: &str,
    ) -> Result<()> {
        let files = self.client.list_files(bucket_name, prefix, None).await?;
        println!("Total files: {}", files.len());
        self.download_files(&files, bucket_name, prefix, out_path)
            .await
    }

    async fn download_files(
        &self,
        files: &Vec<String>,
        bucket: &str,
        prefix: &Option<String>,
        out_path: &str,
    ) -> Result<()> {
        for file in files.iter() {
            if !Path::new(out_path).join(file).exists() {
                self.download_file(file, bucket, prefix, out_path).await?;
            }
        }
        Ok(())
    }

    async fn download_file(
        &self,
        file: &str,
        bucket: &str,
        prefix: &Option<String>,
        out_path: &str,
    ) -> Result<()> {
        let content = self.client.fetch_file_content(file, bucket).await?;
        self.write_file(content, file, prefix, out_path).await
    }

    async fn write_file(
        &self,
        content: StreamingBody,
        key: &str,
        prefix: &Option<String>,
        folder_path: &str,
    ) -> Result<()> {
        let key = prefix
            .as_ref()
            .map_or(key, |prefix| key.trim_start_matches(prefix))
            .trim_matches('/');
        let body = self.read_content(content).await?;
        let file_path = Path::new(folder_path).join(key);
        Ok(self.write_content(&body, &file_path).await?)
    }

    async fn read_content(&self, content: StreamingBody) -> Result<Vec<u8>> {
        let mut stream = content.into_async_read();
        let mut body = Vec::new();
        stream.read_to_end(&mut body).await?;
        Ok(body)
    }

    async fn write_content(&self, body: &[u8], file_path: &PathBuf) -> Result<()> {
        let str_file_path = file_path.to_string_lossy().to_string();

        let mut file = File::create(file_path).await.map_err(|err| IOError {
            path: str_file_path.clone(),
            source: err,
        })?;

        Ok(file.write_all(&body).await.map_err(|err| IOError {
            path: str_file_path.clone(),
            source: err,
        })?)
    }
}
