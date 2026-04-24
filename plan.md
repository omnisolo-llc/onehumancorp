1. **Add `ActiveOrganizations` to `UsageRepository` interface**
   - Update `srcs/server/billing/repository.go` to include `ActiveOrganizations(ctx context.Context) ([]string, error)` in the `UsageRepository` interface.

2. **Implement `ActiveOrganizations` in `PgUsageRepository`**
   - Update `srcs/server/billing/postgres_tracker.go` to implement `ActiveOrganizations`. The method should query PostgreSQL: `SELECT DISTINCT organization_id FROM usage_events`.

3. **Implement `ActiveOrganizations` in `SqliteUsageRepository`**
   - Update `srcs/server/billing/sqlite_tracker.go` to implement `ActiveOrganizations`. The method should query SQLite: `SELECT DISTINCT organization_id FROM usage_events`.

4. **Update `Tracker.ActiveOrganizations` to use the repository**
   - Update `srcs/server/billing/tracker.go` to use `t.repo.ActiveOrganizations(ctx)` instead of the hardcoded `[]string{"demo", "default"}` when `t.repo` is not nil. We will also log any errors returned by the repository and fallback to `[]string{"demo", "default"}` if an error occurs or the list is empty.

5. **Complete pre commit steps**
   - Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit the change**
   - Run tests and submit using `submit`.
