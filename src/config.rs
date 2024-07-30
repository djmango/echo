use anyhow::anyhow;
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct AppConfig {
    pub db_connection_uri: String,
    pub jwt_secret: String,
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
}

impl AppConfig {
    // Asynchronous factory function for creating AppConfig
    pub fn new(secret_store: &SecretStore) -> Result<Self, anyhow::Error> {
        let jwt_secret = secret_store
            .get("JWT_SECRET")
            .ok_or_else(|| anyhow!("JWT_SECRET not found"))?;

        let db_connection_string = secret_store
            .get("DB_CONNECTION_URI")
            .ok_or_else(|| anyhow!("DB_CONNECTION_URI not found"))?;

        let aws_region = secret_store
            .get("AWS_REGION")
            .ok_or_else(|| anyhow!("AWS_REGION not found"))?;

        let aws_access_key_id = secret_store
            .get("AWS_ACCESS_KEY_ID")
            .ok_or_else(|| anyhow!("AWS_ACCESS_KEY_ID not found"))?;

        let aws_secret_access_key = secret_store
            .get("AWS_SECRET_ACCESS_KEY")
            .ok_or_else(|| anyhow!("AWS_SECRET_ACCESS_KEY not found"))?;

        Ok(Self {
            db_connection_uri: db_connection_string,
            jwt_secret,
            aws_region,
            aws_access_key_id,
            aws_secret_access_key,
        })
    }
}