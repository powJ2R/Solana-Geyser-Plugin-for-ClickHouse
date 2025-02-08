CREATE TABLE accounts
(
    slot UInt64,
    pubkey String,
    owner String,
    lamports UInt64,
    executable UInt8,
    rent_epoch UInt64,
    data String,
    updated_at DateTime64(3),
    txn_signature Nullable(String),
    write_version UInt64
)
ENGINE = ReplacingMergeTree(write_version)
ORDER BY (pubkey, slot);

CREATE TABLE transactions
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