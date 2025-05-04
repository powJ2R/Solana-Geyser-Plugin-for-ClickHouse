// TODO:: make it multi-threaded
use clickhouse::Client;
use std::{fmt, sync::Arc};

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
}

// Manually implement Debug for ClickhouseConnection
impl fmt::Debug for ClickhouseConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClickhouseConnection")
            .field("client", &"<clickhouse::Client>")
            .finish()
    }
}