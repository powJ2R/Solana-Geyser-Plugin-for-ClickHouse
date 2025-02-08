## Basic Database CMD

### Show database

```
SHOW DATABASES;
```
- If you want to show the current database, you can use the currentDatabase() function:
```
SELECT currentDatabase();
```

- If you want to use the current database, use 
```
USE  mydatabase
```

### Show tables

```
SHOW TABLES;
or
SHOW FULL TABLES
```

- If  you want table from any specific database
```
SHOW TABLES FROM mydatabse;
```

### Create table
1. Create Local Table
```
CREATE TABLE local_table (
    id UInt32,
    name String,
    value Float64
) ENGINE = MergeTree() ORDER BY (id);
```
This creates a local table local_table with three columns: id (a 32-bit unsigned integer), name (a string), and value (a 64-bit floating-point number). The table uses the MergeTree engine and is ordered by the id column.


2. Create Distributed Table
```
CREATE TABLE distributed_table (
    id UInt32,
    name String,
    value Float64
) ENGINE = ReplicatedMergeTree('/clickhouse/tables/distributed_table', 'replica1') PARTITION BY id ORDER BY (id);
```
This creates a distributed table distributed_table with the same columns as before. The table uses the ReplicatedMergeTree engine and is replicated across multiple nodes in the cluster. The partitioning is done by the id column, and the table is ordered by the same column.