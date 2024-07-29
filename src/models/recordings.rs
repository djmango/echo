use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Recording {
    pub id: Uuid,
    pub session_id: Uuid,
    pub s3_object_key: String,
    pub start_timestamp: DateTime<Utc>,
    pub duration: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Default for Recording {
    fn default() -> Self {
        Recording {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            s3_object_key: String::new(),
            start_timestamp: Utc::now(),
            duration: 0,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Recording {
    pub async fn new(
        pool: &PgPool,
        recording_id: Uuid,
        session_id: Uuid,
        s3_object_key: String,
        start_timestamp_nanos: i64,
        duration_ms: u64,
    ) -> Result<Self> {
        let start_timestamp = Utc.timestamp_nanos(start_timestamp_nanos);

        let recording = Recording {
            id: recording_id,
            session_id,
            s3_object_key,
            start_timestamp,
            duration: duration_ms,
            ..Default::default()
        };

        query!(
            r#"
            INSERT INTO recordings (id, session_id, s3_object_key, start_timestamp, duration, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            recording.id, recording.session_id, recording.s3_object_key, recording.start_timestamp, recording.duration as i64, recording.created_at, recording.updated_at
        )
        .execute(pool)
        .await?;

        Ok(recording)
    }
}