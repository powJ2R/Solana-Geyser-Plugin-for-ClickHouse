# Solana Geyser Plugin for ClickHouse
### Prerequisites
- Rust toolchain (latest stable version)
- Docker (for running ClickHouse locally)
- Git
- CMake (for building Solana)

## Set Up Local Solana Development Environment
- Install Solana CLI
```
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

## Configure Local Testnet
```
solana-test-validator --geyser-plugin-config ./config.json
```

## Set Up ClickHouse
Start ClickHouse using Docker
```
docker run -d --name clickhouse-server \
    -p 8123:8123 \
    -p 9000:9000 \
    --ulimit nofile=262144:262144 \
    clickhouse/clickhouse-server
```

## Build and Test
Build the Plugin
```
cargo build --release
```
Start Solana Validator with Plugin
```
solana-test-validator \
    --geyser-plugin-config config.json
```
Test Data Flow

Send test transactions:

```
solana transfer <RECIPIENT_ADDRESS> 1 --url http://localhost:8899
````

Query ClickHouse to verify data:

```
SELECT * FROM solana.transactions ORDER BY slot DESC LIMIT 5;
```

## cloud

```aiignore
curl --user 'default:5Q5BsFd_QScmr' \
  --data-binary 'SELECT 1' \
  https://xo610muboe.asia-southeast1.gcp.clickhouse.cloud:8443
```
```aiignore
jdbc:clickhouse://xo610muboe.asia-southeast1.gcp.clickhouse.cloud:8443?user=default&password=5Q5BsFd_QScmr&ssl=true
```

# debugging

To debug the plugin:

1. modify your `config.json` to use full path to the .so e.g.
```aiignore
{
    "libpath": "/media/jerry/biggie/DEV/jv/Solana-Geyser-Plugin-for-ClickHouse/target/debug/libsolana_geyser_clickhouse.so",
    ...
```
2. run `Run test-validator`
3. the SIGTRAP will fire
4. breakpoints can be set in the lldb console (they don't work in RustRover)
