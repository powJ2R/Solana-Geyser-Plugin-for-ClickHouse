use std::{sync::Arc, thread};
use tokio::sync::mpsc::{self, Sender};
use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result,
};
use solana_sdk::clock::Slot;
use log::{error, info, LevelFilter, Log};
use thiserror::Error;
use chrono::Utc;

use crate::{
    clickhouse_client::ClickhouseConnection,
    worker::{AccountUpdate, Worker},
};

const CHANNEL_CAPACITY: usize = 100_000;

#[derive(Error, Debug)]
enum PluginError{
    #[error("Replica account V0.0.1 not supported anymore")]
    ReplicaAccountV001NotSupported,

    #[error("Channel send error: {0}")]
    ChannelError(#[from] mpsc::error::TrySendError<AccountUpdate>),
}

#[derive(Debug)]
struct ClickhousePlugin {
    conn: Arc<ClickhouseConnection>,
    sender: Option<Sender<AccountUpdate>>,
}


impl Default for ClickhousePlugin {
    fn default() -> Self {
        Self {
            conn: Arc::new(ClickhouseConnection::new()),
            sender: None
        }
    }
}


impl GeyserPlugin for ClickhousePlugin {
    fn name(&self) -> &'static str {
        "Clickhouse Plugin"
    }

    #[allow(unused_variables)]
    fn setup_logger(&self, logger: &'static dyn Log, level: LevelFilter) -> Result<()> {
        log::set_max_level(level);
        if let Err(err) = log::set_logger(logger) {
            return Err(GeyserPluginError::Custom(Box::new(err)));
        }
        Ok(())
    }

    #[no_mangle]
    #[inline(never)]
    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> Result<()> {
        info!("ClickhousePlugin loaded with config file: {}", config_file);
        unsafe {
            // TODO: do we need this still?
            libc::raise(libc::SIGTRAP);
        }
        
        // std::thread::sleep(std::time::Duration::from_secs(60));
        // info!("ClickhousePlugin loaded with config file: {}", config_file);
        let conn = Arc::new(ClickhouseConnection::new());
        self.conn = conn.clone();
        info!("ClickhousePlugin connected to Clickhouse");

        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        self.sender = Some(sender);

        // Spawn worker thread to deligate
        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime");

            let mut worker = Worker::new(conn, receiver);
            runtime.block_on(async move {
                worker.run().await;
            });
        });

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Clickhouse Plugin unloaded")
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        _is_startup: bool,
    ) -> Result<()> {
        info!("update_account");

        if let ReplicaAccountInfoVersions::V0_0_3(account_info) = account {
            if let Some(sender) = &self.sender {
                let update = AccountUpdate {
                    pubkey: hex::encode(account_info.pubkey),
                    lamports: account_info.lamports,
                    owner: hex::encode(account_info.owner),
                    executable: if account_info.executable { 1 } else { 0 },
                    rent_epoch: account_info.rent_epoch,
                    data: hex::encode(account_info.data),
                    slot,
                    updated_at: Utc::now(),
                    txn_signature: None,
                    write_version: account_info.write_version,
                };

                sender
                    .try_send(update)
                    .map_err(|e| GeyserPluginError::Custom(Box::new(PluginError::ChannelError(e))))?;
            }
        } else {
            return Err(GeyserPluginError::Custom(Box::new(
                PluginError::ReplicaAccountV001NotSupported,
            )));
        }

        Ok(())
    }
    /// Check if the plugin is interested in account data
    /// Default is true -- if the plugin is not interested in
    /// account data, please return false.
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in transaction data
    /// Default is false -- if the plugin is interested in
    /// transaction data, please return true.
    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in entry data
    /// Default is false -- if the plugin is interested in
    /// entry data, return true.
    fn entry_notifications_enabled(&self) -> bool {
        true
    }
}



#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = ClickhousePlugin::default();
    let plugin = Box::new(plugin);
    Box::into_raw(plugin)
}
