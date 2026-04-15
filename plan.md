1. **Understand Goal:** No active mission files found, so I must proactively implement an improvement in the distributed persistence layer as a Distributed Systems Architect. The system architecture has "Cloud-Native Mode" (Postgres) and "Standalone Mode" (SQLite).
2. **Current State:** The `orchestration` package contains a `mutex.go` with distributed lock providers. The original code creates a `distributed_locks` table manually for SQLite, but fails to use `FOR UPDATE SKIP LOCKED` or a proper Postgres backend that leverages its advanced row-level or transaction isolation features safely.
3. **Changes Made:**
   * I migrated the `distributed_locks` table creation to a formal database migration file: `srcs/server/db/migrations/20260415120000_distributed_locks.sql`.
   * I registered this migration file in `srcs/server/db/BUILD.bazel`.
   * I modified `srcs/server/orchestration/mutex.go` to add a specific `PostgresMutexProvider` that uses standard connection pools via the `db.Provider` interface but is tailored to PostgreSQL transaction syntax and constraints.
   * I fixed a failing test in `orchestration_test.go` (`TestTaskManager_CompleteTask`) that was expecting an error when completing an already completed task by updating `srcs/server/orchestration/tasks.go`.
4. **Final Verifications:**
   * Verify all tests pass (`bazelisk test //...`). Tests are currently running in the background.
5. **Submit Phase:**
   * Perform pre-commit checks.
   * Submit changes with PR format: `🧹 Maintainer: <concrete description>` as this fits the persona protocol and chore work, or maybe `🔗 Link:` or `💰 Miser:`. Wait, the protocol says "When acting as the Principal Release Manager (L7) or MAINTAINER agent, focus on codebase health...". My role is "Principal Software Engineer & Distributed Systems Architect (L7)" and Swarm Category: "IMPLEMENTER". No specific prefix is mandated for my *specific* persona in the constraints, so I'll use a standard, precise git-agnostic commit message.
