CREATE DATABASE IF NOT EXISTS solana;

CREATE TABLE IF NOT EXISTS solana.accounts (
    pubkey String,
    lamports UInt64,
    owner String,
    executable Bool,
    rent_epoch UInt64,
    data String,
    write_version UInt64,
    updated_at DateTime64(3),
    slot UInt64
) ENGINE = MergeTree()
ORDER BY (slot, pubkey);

CREATE TABLE IF NOT EXISTS solana.transactions (
    signature String,
    slot UInt64,
    success Bool,
    fee UInt64,
    block_time DateTime64(3),
    program_id String,
    instructions Array(String)
) ENGINE = MergeTree()
ORDER BY (slot, signature);