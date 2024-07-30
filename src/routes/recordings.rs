use actix_web::{post, web};
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, Region};
use aws_sdk_s3::{config::Credentials, presigning::PresigningConfig, Client};
use std::{sync::Arc, time::Duration};
use tracing::error;

use crate::models::Recording;
use crate::types::SaveRecordingRequest;
use crate::{config::AppConfig, middleware::auth::AuthenticatedUser, AppState};

#[post("/fetch_save_url")]
async fn fetch_save_url(
    app_state: web::Data<Arc<AppState>>,
    app_config: web::Data<Arc<AppConfig>>,
    _authenticated_user: AuthenticatedUser,
    req_body: web::Json<SaveRecordingRequest>,
) -> Result<String, actix_web::Error> {
    let recording_id = req_body.recording_id;
    let session_id = req_body.session_id;
    let start_timestamp = req_body.start_timestamp_nanos;
    let duration_ms = req_body.duration_ms;

    let r2_object_key = format!("{}/{}.mp4", session_id, start_timestamp);

    Recording::new(
        &app_state.pool.clone(),
        recording_id,
        session_id,
        r2_object_key.clone(),
        start_timestamp,
        duration_ms,
    )
    .await
    .map_err(|e| {
        error!("Error saving recording row: {:?}", e);
        actix_web::error::ErrorInternalServerError(e.to_string())
    })?;

    let presigned_url = generate_presigned_url(app_config, r2_object_key.clone())
        .await
        .map_err(|e| {
            error!("Error getting presigned url: {:?}", e);
            actix_web::error::ErrorInternalServerError(e.to_string())
        })?;

    Ok(presigned_url)
}

async fn generate_presigned_url(
    app_config: web::Data<Arc<AppConfig>>,
    object_key: String,
) -> Result<String> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("auto"));
    let credentials = Credentials::new(
        app_config.r2_access_key_id.clone(),
        app_config.r2_secret_access_key.clone(),
        None,
        None,
        "env-credentials",
    );

    let config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(credentials)
        .endpoint_url(app_config.r2_endpoint_url.clone())
        .load()
        .await;

    let client = Client::new(&config);

    let bucket_name = "ghost-videos";

    let presigned_request = client
        .put_object()
        .bucket(bucket_name)
        .key(object_key)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(3000))?)
        .await?;

    Ok(presigned_request.uri().to_string())
}
