1. **Fix PostgreSQL UpsertMission Parity Issue:**
   - The method `UpsertMission` in `srcs/server/orchestration/sip.go` contains a Postgres-specific block that uses `SELECT ... FOR UPDATE SKIP LOCKED`.
   - The `Memory` constraints state: "To maintain absolute mode parity between PostgreSQL and SQLite in the OHC Hybrid Architecture, avoid divergent conditional code paths (like `if s.db.IsSQLite()`) and Postgres-specific concurrency clauses like `FOR UPDATE SKIP LOCKED` for upserts. Use the standard `INSERT ... ON CONFLICT (id) DO UPDATE` syntax, which is natively supported by both databases."
   - I will refactor `UpsertMission` to use the standard `INSERT ... ON CONFLICT` statement for both SQLite and Postgres.
2. **Remove Unused Imports / Refactor Chaos Test:**
   - Review and ensure that any references to `IsSQLite` in `UpsertMission` are removed, simplifying the codebase.
3. **Run Verification and Pre-commit Steps:**
   - I will run `bazelisk test //srcs/server/orchestration/...` to confirm tests pass.
   - I will execute pre-commit steps and instructions as required.
4. **Submit PR:**
   - Submit the PR correctly with the required `MAINTAINER` focus.
