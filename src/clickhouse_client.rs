// TODO
use clickhouse::Client;
use std::sync::Arc;

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

    pub async fn create_table(&self, db_name: &str) -> Result<(), Box<dyn std::error::Error>> {  
        println!("Creating table accounts...");
        self.client.query(&format!(r#"
            CREATE TABLE IF NOT EXISTS {db_name}.accounts (
                pubkey String,
                lamports UInt64,
                owner String,
                executable Bool,
                rent_epoch UInt64,
                data String,
                write_version UInt64,
                updated_at DateTime64(3),
                slot UInt64
            ) ENGINE = MergeTree()
            ORDER BY (slot, pubkey);
        "#)).execute().await?;

        println!("Creating table transaction...");
        self.client.query(&format!(r#"
            CREATE TABLE IF NOT EXISTS {db_name}.transactions (
               signature String,
               slot UInt64,
               success Bool,
               fee UInt64,
               block_time DateTime64(3),
               program_id String,
               instructions Array(String) 
            ) ENGINE = MergeTree()
            ORDER BY (slot, signature);
        "#)).execute().await?;
        Ok(())
    }
}