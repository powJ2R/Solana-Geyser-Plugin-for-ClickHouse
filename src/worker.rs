// till now my understanding we will use channel to send data from geyser plugin to worker
// and then worker will send data to clickhouse

use std::sync::Arc;
use tokio::{
    sync::mpsc::Receiver,
    time::{Duration, sleep, timeout}
};
use log::{error, info};
use chrono::{DateTime, Utc};

use crate::clickhouse_client::ClickhouseConnection;

const BATCH_SIZE: usize = 1000;
const BATCH_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RETRIES: u32 = 3;

#[derive(Debug)]
pub struct AccountUpdate {
    pub slot: u64,
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
    pub executable: u8,
    pub rent_epoch: u64,
    pub data: String,
    pub updated_at: DateTime<Utc>,
    pub txn_signature: Option<String>,
    pub write_version: u64,
}

pub struct Worker {
    conn: Arc<ClickhouseConnection>,
    receiver: Receiver<AccountUpdate>,
}

impl Worker {
    pub fn new(conn: Arc<ClickhouseConnection>, receiver: Receiver<AccountUpdate>) -> Self {
        Self { conn, receiver }
    }

    pub async fn run(&mut self) {
        let mut batch = Vec::with_capacity(BATCH_SIZE);

        loop {
            match timeout(BATCH_TIMEOUT, self.receiver.recv()).await {
                Ok(Some(update)) => {
                    batch.push(update);
                    if batch.len() >= BATCH_SIZE {
                        self.process_batch(std::mem::take(&mut batch)).await;
                    }
                }
                Ok(None) => {
                    // Channel closed, process remaining items and exit
                    if !batch.is_empty() {
                        self.process_batch(std::mem::take(&mut batch)).await;
                    }
                    info!("Channel closed, worker shutting down");
                    break;
                }
                Err(_) => {
                    // Timeout reached, process current batch if any
                    if !batch.is_empty() {
                        self.process_batch(std::mem::take(&mut batch)).await;
                    }
                }
            }
        }
    }

    async fn process_batch(&self, batch: Vec<AccountUpdate>) {
        let mut retries = 0;
        while retries < MAX_RETRIES {
            match self.insert_batch(&batch).await {
                Ok(_) => {
                    info!("Successfully inserted batch of {} records", batch.len());
                    break;
                }
                Err(e) => {
                    retries += 1;
                    error!(
                        "Batch insert failed (attempt {}/{}): {}",
                        retries, MAX_RETRIES, e
                    );
                    if retries < MAX_RETRIES {
                        sleep(Duration::from_secs(1 << retries)).await;
                    }
                }
            }
        }
    }

    async fn insert_batch(&self, batch: &[AccountUpdate]) -> Result<(), Box<dyn std::error::Error>> {
        let values: Vec<String> = batch
            .iter()
            .map(|update| {
                format!(
                    "({}, '{}', '{}', {}, {}, {}, '{}', '{}', {}, {})",
                    update.slot,
                    update.pubkey,
                    update.owner,
                    update.lamports,
                    update.executable,
                    update.rent_epoch,
                    update.data,
                    update.updated_at.format("%Y-%m-%d %H:%M:%S%.3f"),
                    update.txn_signature
                        .as_ref()
                        .map_or("NULL".to_string(), |s| format!("'{}'", s)),
                    update.write_version
                )
            })
            .collect();

        let query = format!(
            "INSERT INTO accounts (slot, pubkey, owner, lamports, executable, rent_epoch, data, updated_at, txn_signature, write_version) VALUES {}",
            values.join(",")
        );

        self.conn.client.query(&query).execute().await?;
        Ok(())
    }
}