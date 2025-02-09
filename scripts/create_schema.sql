CREATE DATABASE IF NOT EXISTS SOLANA;

CREATE TABLE SOLANA.accounts
(
    slot UInt64,
    pubkey FixedString(44),
    owner FixedString(44),
    lamports UInt64,
    executable UInt8,
    rent_epoch UInt64,
    data String,
    updated_at DateTime64(3),
    txn_signature Nullable(FixedString(88)),
    write_version UInt64
)
ENGINE = ReplacingMergeTree(write_version)
PRIMARY KEY (pubkey, slot)
SETTINGS index_granularity = 8192;

CREATE TABLE SOLANA.transactions
(
    signature String,
    slot UInt64,
    block_time DateTime64(3),
    err Nullable(String),
    fee UInt64,
    index UInt32,
    programs Array(String),
    accounts Array(String),
    log_messages Array(String),
    data String
)
ENGINE = MergeTree
ORDER BY (slot, block_time);