use anyhow::anyhow;
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct AppConfig {
    pub db_connection_uri: String,
    pub jwt_secret: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_endpoint_url: String,
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

        let r2_access_key_id = secret_store
            .get("R2_ACCESS_KEY_ID")
            .ok_or_else(|| anyhow!("R2_ACCESS_KEY_ID not found"))?;

        let r2_secret_access_key = secret_store
            .get("R2_SECRET_ACCESS_KEY")
            .ok_or_else(|| anyhow!("R2_SECRET_ACCESS_KEY not found"))?;

        let r2_endpoint_url = secret_store
            .get("R2_ENDPOINT_URL")
            .ok_or_else(|| anyhow!("R2_ENDPOINT_URL not found"))?;

        let workos_api_key = secret_store
            .get("WORKOS_API_KEY")
            .ok_or_else(|| anyhow!("WORKOS_API_KEY not found"))?;

        let workos_client_id = secret_store
            .get("WORKOS_CLIENT_ID")
            .ok_or_else(|| anyhow!("WORKOS_CLIENT_ID not found"))?;

        Ok(Self {
            db_connection_uri: db_connection_string,
            jwt_secret,
            r2_access_key_id,
            r2_secret_access_key,
            r2_endpoint_url,
        })
    }
}
