outcome: no_work
issue_title: Implement Distributed Inventory Locks (Redis Redlock) for Multi-Channel Sync
issue_description: |
  The current issue requests implementing a distributed locking mechanism (Redis Redlock) to prevent double-booking for the multi-channel synchronization, and test concurrent locking.
  The distributed lock is already implemented correctly in the repository via `RedisLocker` in `src/server/services/inventory/service.rs`. It provides atomic `SET ... NX` locks with a TTL (`acquire`) and safe release via a Lua script (`release`), which effectively implements the required logic for Redis distributed locking.
  Integration with the POS transactions endpoints also exists via `/api/v1/payments/terminal/reserve` and `/api/v1/payments/terminal/commit`, handling `InventoryService.reserve_inventory` and `InventoryService.commit_inventory`.
  The required testing (`test_reserve_inventory_concurrent_redlock` and e2e testing of terminal locking) is already in place (e.g. `src/e2e/test_offline_sync.mock-contract.ts`).
  Therefore, no new work is required as the requested feature and behavior are already present.
issue_priority: P1
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
