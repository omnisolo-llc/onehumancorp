---
title: "Proactive: Distributed Mutex for Teammate Mesh"
status: DONE
agent: Implementer
priority: "P0"
estimated_scope: "Medium"
---

# Problem Statement
As the OHC Swarm scales, agents executing complex workflows need to ensure mutually exclusive access to shared resources (like task DAGs and task queues). While `statemachine.Transition` handles DB locks, there isn't a robust, explicit `MutexProvider` that acts across the cloud-native environment (using `rueidis` Redis locks) and the local SQLite database. The KAIROS architectural guidelines suggest that agents must not concurrently clobber each other's states.

# Design Doc
**Architecture:**
- Create `srcs/server/orchestration/mutex.go` defining the `MutexProvider` interface and implementations:
  - `RedisMutex` (uses `github.com/redis/rueidis` to implement Redlock or simple `SET NX EX`).
  - `SQLiteMutex` (uses `database/sql` with a new `distributed_locks` table to emulate exclusive locks).

**DB Schema Changes (Standalone / Hybrid Fallback):**
```sql
CREATE TABLE IF NOT EXISTS distributed_locks (
    lock_key TEXT PRIMARY KEY,
    owner_id TEXT NOT NULL,
    expires_at DATETIME NOT NULL
);
```

**API Contracts (Go):**
```go
type Mutex interface {
    Lock(ctx context.Context, ttl time.Duration) error
    Unlock(ctx context.Context) error
}

type MutexProvider interface {
    NewMutex(key string) Mutex
}
```

# Implementation Prompt
1. Define the `Mutex` and `MutexProvider` interfaces in `srcs/server/orchestration/mutex.go`.
2. Implement `RedisMutex` using `rueidis`.
3. Implement `SQLiteMutex` using the database schema and explicit transactional locks.
4. Add unit tests for both implementations in `srcs/server/orchestration/mutex_test.go` ensuring 90%+ test coverage.
