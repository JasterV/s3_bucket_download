use anyhow::{Result, Error};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Request, S3, S3Client as Client, StreamingBody};
use rusoto_signature::Region;
use crate::errors::AppError::*;

pub struct S3Client {
    s3: Client 
}

impl S3Client {
    pub fn new(region: &str, endpoint: &str) -> Self {
        let s3_client = Client::new(Region::Custom {
            name: region.to_owned(),
            endpoint: endpoint.to_owned(),
        });
        S3Client {
            s3: s3_client
        }
    }

    pub async fn fetch_file_content(&self, key: &str, bucket: &str) -> Result<StreamingBody> {
        let result = self
            .s3
            .get_object(GetObjectRequest {
                bucket: bucket.into(),
                key: key.into(),
                ..Default::default()
            })
            .await?;
        result.body.ok_or(Error::from(CorruptedFile))
    }

    #[async_recursion]
    pub async fn list_files(
        &self,
        bucket: &str,
        prefix: &Option<String>,
        start_after: Option<String>,
    ) -> Result<Vec<String>> {
        let mut keys = self.list_objects(bucket, prefix, start_after).await?;
        if !keys.is_empty() {
            let last_key = keys.last().unwrap().to_owned();
            keys.extend(self.list_files(bucket, prefix, Some(last_key)).await?);
        }
        Ok(keys)
    }

    async fn list_objects(
        &self,
        bucket: &str,
        prefix: &Option<String>,
        start_after: Option<String>,
    ) -> Result<Vec<String>> {
        let keys = self
            .s3
            .list_objects_v2(ListObjectsV2Request {
                bucket: bucket.into(),
                prefix: prefix.to_owned(),
                start_after,
                ..Default::default()
            })
            .await?
            .contents
            .map(|contents| contents.into_iter().filter_map(|obj| obj.key).collect())
            .unwrap_or(vec![]);
        Ok(keys)
    }
}