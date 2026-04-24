1.  **Update `shared_tasks` Postgres Migration**
    - Modify `srcs/server/db/migrations/20260413165953_kairos_phase1_pg.sql` to match the exact schema detailed in `docs/technical/features/kairos_orchestration.md` while honoring the multi-tenant architecture.
    - Change `organization_id` to `tenant_id`.
    - Enable Row Level Security (RLS) on the table.

2.  **Update `shared_tasks` SQLite Migration**
    - Modify `srcs/server/db/migrations/20260413165953_kairos_phase1_sqlite.sql` to match the equivalent schema for SQLite, using `tenant_id`.

3.  **Update TaskRecord / Go Struct Definitions**
    - Ensure `TaskRecord` in `srcs/server/db/provider.go` has fields matching the new schema (`TenantID`, `ParentPlanID`, `Title`, `Status`, `AssignedAgentID`, `CreatedAt`, `UpdatedAt`).
    - Make sure `SharedTask` in `srcs/server/models/task.go` and `srcs/server/orchestration/kairos/kairos_shared_task.go` correctly map to `TenantID` instead of `OrganizationID`.

4.  **Implement Unit Tests**
    - Ensure the logic for parsing `TaskRecord` instances and `AcquireTask` matches the new schema across both `postgres_provider.go` and `sqlite_provider.go`.
    - Add tests to `provider_test.go` checking parsing and ensuring >90% coverage for `SharedTaskRepository` and DB queries related to `shared_tasks`.

5.  **Run Tests**
    - Run `bazelisk test //...` to ensure no regressions were introduced.

6.  **Complete pre-commit steps**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
