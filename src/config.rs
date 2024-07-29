use anyhow::anyhow;
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct AppConfig {
    pub db_connection_uri: String,
}

impl AppConfig {
    // Asynchronous factory function for creating AppConfig
    pub fn new(secret_store: &SecretStore) -> Result<Self, anyhow::Error> {
        let db_connection_string = secret_store
            .get("DB_CONNECTION_URI")
            .ok_or_else(|| anyhow!("DB_CONNECTION_URI not found"))?;

        Ok(Self {
            db_connection_uri: db_connection_string,
        })
    }
}