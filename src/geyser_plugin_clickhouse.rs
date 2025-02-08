// This is the entery point of plugin
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;
use solana_sdk::clock::Slot;
use crate::clickhouse_client::ClickhouseConnection;
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

    fn setup_logger(
        &self,
        logger: &'static dyn log::Log,
        level: log::LevelFilter,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn on_load(
        &mut self,
        _config_file: &str,
        _is_reload: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let conn = ClickhouseConnection::new();
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &self,
        account: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = ClickhousePlugin::default();
    let plugin = Box::new(plugin);
    Box::into_raw(plugin)
}
