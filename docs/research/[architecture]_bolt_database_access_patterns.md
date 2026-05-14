# Database Access Patterns for Maximum Throughput

## Introduction
The database is often the ultimate bottleneck in high-load agentic systems. The Bolt standard mandates specific access patterns to ensure our Postgres and SQLite implementations scale elegantly.

## 1. Zero-Allocation Row Mapping
Using `sqlx`, we prefer mapping rows directly to pre-allocated structs using `try_get` or macro-driven mapping.
- **Goal**: Minimize the number of intermediate `String` or `Vec` allocations during high-volume queries.

## 2. Connection Pool Tuning
- **Standalone**: Max connections = 1. SQLite performs best with a single writer. Our `execute_with_retry` logic handles the busy-wait state.
- **Cloud**: Max connections = 20 (per pod). We use `statement_cache_capacity=0` for specific cloud providers that utilize PgBouncer in transaction mode.

## 3. Query Indexing Strategy
Every `WHERE` clause must be backed by an index.
- **Mandatory Indexes**: `tenant_id`, `organization_id`, `status`, `updated_at`.
- **Compound Indexes**: For workers polling for PENDING tasks, a compound index on `(status, department, created_at)` is mandatory to prevent full table scans.

## 4. Prepared Statement Re-use
Whenever possible, use prepared statements for high-frequency updates (e.g., heartbeat or status logging).
- **Latency Impact**: Reduces query planning time by 2-5ms per execution.

## 5. Summary of DB Micro-Latencies
| Environment | Query Type | Bolt Latency |
|-------------|------------|--------------|
| Standalone (SQLite) | Point Read (ID) | < 100us |
| Standalone (SQLite) | Range Scan (100 rows) | < 1.5ms |
| Cloud (Postgres) | Point Read (ID) | < 500us |
| Cloud (Postgres) | Range Scan (100 rows) | < 5.0ms |

## Conclusion
A performant database layer is the bedrock of OHC. By following these patterns, we ensure that as organizations grow from 1 agent to 1,000, the data remains accessible in constant time.
