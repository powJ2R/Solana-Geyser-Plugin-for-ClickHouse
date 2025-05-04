# runs CH in docker without user default and no password
docker run --rm -d --name clickhouse-server \
  -e CLICKHOUSE_DB=solana -e CLICKHOUSE_SKIP_USER_SETUP=1 \
  -p 8123:8123 -p 9000:9000/tcp --ulimit nofile=262144:262144 clickhouse/clickhouse-server

# -e CLICKHOUSE_DB=solana -e CLICKHOUSE_USER=default -e CLICKHOUSE_DEFAULT_ACCESS_MANAGEMENT=1 -e CLICKHOUSE_PASSWORD=password \
# clickhouse-client
