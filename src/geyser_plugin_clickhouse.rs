use crate::clickhouse_client::ClickhouseConnection;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result,
};
use solana_sdk::clock::Slot;
use log::{LevelFilter, Log};

#[derive(Debug)]
struct ClickhousePlugin;

impl Default for ClickhousePlugin {
    fn default() -> Self {
        Self {}
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

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> Result<()> {
        let conn = ClickhouseConnection::new();
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> Result<()> {
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
