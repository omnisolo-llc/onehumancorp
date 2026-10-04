outcome: no_work
issue_title: "[Feature] Mobile Tap-to-Pay Omnichannel Inventory Synchronization"
issue_description: |
  Mobile Tap-to-Pay Omnichannel Inventory Synchronization is already implemented and verified in the codebase.
  The files `src/server/api/terminal_offline_sync.rs` and `src/server/workers/pos_sync_worker.rs` already implement the synchronization, conflicts handling, and stock deduction logic. `src/server/services/inventory_sync.rs` also exposes gRPC interfaces to handle reservations. End-to-end tests exist in `src/e2e/inventory_sync.spec.ts`.
