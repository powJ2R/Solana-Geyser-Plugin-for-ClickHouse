use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfo, ReplicaTransactionInfo,
    Result as PluginResult,
};
use clickhouse_rs::{Pool, Options};
use std::sync::Arc;

#[derive(Default)]
pub struct ClickHouseGeyserPlugin {
    pool: Option<Arc<Pool>>,
}

impl GeyserPlugin for ClickHouseGeyserPlugin {
    fn name(&self) -> &'static str {
        "ClickHouseGeyserPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        // Initialize ClickHouse connection pool
        let options = Options::from_str("tcp://localhost:9000")?;
        self.pool = Some(Arc::new(Pool::new(options)));
        Ok(())
    }

    fn update_account(
        &mut self,
        account: &ReplicaAccountInfo,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if let Some(pool) = &self.pool {
            let mut client = pool.get_handle()?;
            
            // Insert account data into ClickHouse
            client.execute(
                "INSERT INTO solana.accounts 
                (pubkey, lamports, owner, executable, rent_epoch, data, 
                write_version, updated_at, slot) 
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                (
                    base64::encode(&account.pubkey),
                    account.lamports,
                    base64::encode(&account.owner),
                    account.executable,
                    account.rent_epoch,
                    base64::encode(&account.data),
                    account.write_version,
                    chrono::Utc::now(),
                    slot,
                ),
            )?;
        }
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: &ReplicaTransactionInfo,
        slot: u64,
    ) -> PluginResult<()> {
        if let Some(pool) = &self.pool {
            let mut client = pool.get_handle()?;
            
            // Insert transaction data into ClickHouse
            client.execute(
                "INSERT INTO solana.transactions 
                (signature, slot, success, fee, block_time, program_id, instructions) 
                VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    base64::encode(&transaction.signature),
                    slot,
                    transaction.err.is_none(),
                    transaction.fee,
                    chrono::Utc::now(),
                    transaction.program_id.map(base64::encode),
                    transaction.instructions
                        .iter()
                        .map(|i| serde_json::to_string(i))
                        .collect::<Vec<_>>(),
                ),
            )?;
        }
        Ok(())
    }
}

#[no_mangle]
pub fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = ClickHouseGeyserPlugin::default();
    Box::into_raw(Box::new(plugin))
}