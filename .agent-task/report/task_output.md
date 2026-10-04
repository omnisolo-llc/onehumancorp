outcome: no_work
issue_title: Implement Multi-Channel POS Sync and Row-Level Inventory Reservation Architecture
issue_description: "The requested Redis Redlock-based inventory reservation system and PostgreSQL central ledger with `SELECT ... FOR UPDATE` and RLS are already fully implemented in `src/server/services/inventory/service.rs` and `src/server/api/terminal_offline_sync.rs`. The required E2E tests, including concurrent checkout simulation and the Operations Agent push notification, already exist and pass in `src/e2e/inventory_conflict.mock-contract.ts`."
issue_priority: P0
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
