// TODO:: make it multi-threaded
use clickhouse::Client;
use hex::encode;
use std::{fmt, sync::Arc, error::Error};

#[derive(Clone)]
pub struct ClickhouseConnection {
    pub client: Arc<Client>
}

impl ClickhouseConnection {
    pub fn new()-> Self {
        let client: Arc<Client>= Arc::new(
            Client::default().with_url("http://localhost:8123").with_database("SOLANA")
        );
     Self { client }   
    }

    pub async fn create_db(&self, db_name: &str)-> Result<(), Box<dyn std::error::Error>> {
        self.client.query(&format!("CREATE DATABASE IF NOT EXISTS {}", db_name)).execute().await?;
        Ok(())
    }

    pub async  fn insert_account_data(
        &self,
        pubkey: &str,
        lamports: u64,
        owner: &str,
        executable: bool,
        rent_epoch: u64,
        data: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        let query = format!(
            "INSERT INTO accounts (pubkey, lamports, owner, executable, rent_epoch, data) VALUES ('{}', {}, '{}', {}, {}, '{}')",
            pubkey, lamports, owner, executable, rent_epoch, encode(data)
        );
        self.client.query(&query).execute().await?;
        Ok(())
    }
}

// Manually implement Debug for ClickhouseConnection
impl fmt::Debug for ClickhouseConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClickhouseConnection")
            .field("client", &"<clickhouse::Client>")
            .finish()
    }
}