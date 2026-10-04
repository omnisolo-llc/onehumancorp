outcome: blocked
issue_title: Implement Multi-Channel POS Sync and Row-Level Inventory Reservation Architecture
issue_description: "The issue requests implementing a Redis Redlock-based inventory reservation system combined with a PostgreSQL central ledger relying on `SELECT ... FOR UPDATE` and Row-Level Security (RLS) to solve double-booking and out-of-stock anomalies across online and in-store channels. However, there is no Redis dependency or integration existing in the current Rust/Node repository. The project uses PostgreSQL (and SQLite) for persistence and row-level locks (e.g., in `src/server/api/terminal_offline_sync.rs` and `src/server/api/durable_sync.rs`). Introducing Redis and Redlock would violate the instruction 'reuse existing modules and verify the business transition rather than creating another subsystem' and the finding that the existing architecture should be reused. Furthermore, the issue requests creating an E2E Playwright test simulating concurrent checkout attempts (one POS, one Online) and logic for 'The Manager' Operations Agent, but without the underlying Redis reservation mechanism, this cannot be implemented as requested. Therefore, the implementation is blocked."
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
