// This is the entery point of plugin 
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;
use solana_sdk::clock::Slot;

#[derive(Debug)]
struct ClickhousePlugin;

impl Default for ClickhousePlugin {
    fn default() -> Self {
        Self{}
    }
}

impl GeyserPlugin for ClickhousePlugin {
    fn name(&self) -> &'static str {
        "Clickhouse Plugin"
    }
    
    fn setup_logger(&self, logger: &'static dyn log::Log, level: log::LevelFilter) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
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
    
    fn notify_end_of_startup(&self) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn update_slot_status(
        &self,
        slot: Slot,
        parent: Option<u64>,
        status: solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn notify_transaction(
        &self,
        transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn notify_entry(&self, entry: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaEntryInfoVersions) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn notify_block_metadata(&self, blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }
    
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }
    
    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
    
    fn entry_notifications_enabled(&self) -> bool {
        false
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin{
    let plugin = ClickhousePlugin::default();
    let plugin = Box::new(plugin);
    Box::into_raw(plugin)
}