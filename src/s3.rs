use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use aws_sdk_s3::Client;
        use std::env;
        use tokio::sync::watch;
        use once_cell::sync::Lazy;

        pub static S3_STATUS: Lazy<(watch::Sender<Option<bool>>, watch::Receiver<Option<bool>>)> =
            Lazy::new(|| watch::channel(None));

        pub async fn init_s3_poller() {
            let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "https://s3.unghotui.vn".to_string());
            
            tokio::spawn(async move {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(3))
                    .build()
                    .unwrap_or_default();
                loop {
                    let is_up = match client.head(&endpoint).send().await {
                        Ok(resp) => {
                            let status = resp.status();
                            // If it's a 4xx client error (like 400/403) or success, it means MinIO is responding
                            status.is_client_error() || status.is_success()
                        }
                        Err(_) => false,
                    };
                    let _ = S3_STATUS.0.send(Some(is_up));
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            });
        }

        pub async fn get_s3_client() -> Result<(Client, String), String> {
            let endpoint = env::var("S3_ENDPOINT").map_err(|_| "S3_ENDPOINT not set".to_string())?;
            let access_key = env::var("S3_ACCESS_KEY").map_err(|_| "S3_ACCESS_KEY not set".to_string())?;
            let secret_key = env::var("S3_SECRET_KEY").map_err(|_| "S3_SECRET_KEY not set".to_string())?;
            let region_name = env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());
            let bucket = env::var("S3_BUCKET").map_err(|_| "S3_BUCKET not set".to_string())?;

            let credentials = aws_sdk_s3::config::Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "manual",
            );

            let config = aws_sdk_s3::Config::builder()
                .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
                .credentials_provider(credentials)
                .region(aws_sdk_s3::config::Region::new(region_name))
                .endpoint_url(&endpoint)
                .force_path_style(true) // Required for MinIO
                .build();

            Ok((Client::from_conf(config), bucket))
        }

        pub async fn upload_file(file_name: &str, data: bytes::Bytes, content_type: &str) -> Result<String, String> {
            let (client, bucket) = get_s3_client().await?;
            let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "https://s3.unghotui.vn".to_string());
            
            client
                .put_object()
                .bucket(&bucket)
                .key(file_name)
                .body(data.into())
                .content_type(content_type)
                .send()
                .await
                .map_err(|e| format!("Failed to upload to S3: {}", e))?;
                
            let url = format!("{}/{}/{}", endpoint, bucket, file_name);
            Ok(url)
        }

        pub async fn generate_presigned_url(file_name: &str, content_type: &str) -> Result<(String, String), String> {
            let (client, bucket) = get_s3_client().await?;
            let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "https://s3.unghotui.vn".to_string());
            
            let expires_in = std::time::Duration::from_secs(15 * 60);
            let presigned_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in)
                .map_err(|e| format!("Failed to create presigned config: {}", e))?;
                
            let presigned_req = client
                .put_object()
                .bucket(&bucket)
                .key(file_name)
                .content_type(content_type)
                .presigned(presigned_config)
                .await
                .map_err(|e| format!("Failed to generate presigned url: {}", e))?;
                
            let upload_url = presigned_req.uri().to_string();
            let public_url = format!("{}/{}/{}", endpoint, bucket, file_name);
            
            Ok((public_url, upload_url))
        }
    }
}
