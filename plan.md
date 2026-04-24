1. **Remove `"sys"` and `""` empty org bypass in `postgres_store.go`**
   - In `PgUserRepository`, replace `orgID == "" || orgID == "sys"` checks with exact matching where `organization_id = $1`.
   - Update `CreateUser`, `GetByID`, `GetByUsername`, `GetByEmail`, `GetByOIDCSubject`, `ListUsers`, `UpdateUser`, `DeleteUser`.
   - Remove branches that exclude `organization_id` from WHERE clauses when `orgID` is empty or `"sys"`. This will force all operations to explicitly scope by `orgID`.

2. **Remove `"sys"` and `""` empty org bypass in `store.go`**
   - Similar to `postgres_store.go`, remove conditions like `orgID == "sys" || orgID == ""` which fallback to searching global state or ignore the `OrganizationID`.
   - Update `Authenticate`, `GetUser`, `ListUsers`, `UpdateUser`, `DeleteUser` in the fallback in-memory logic.

3. **Verify and Run Tests**
   - Run `bazelisk test //srcs/server/auth/...` to ensure `Store` and `PgUserRepository` tests pass with proper tenant scoping.
   - Run complete pre-commit steps to ensure no breakages in CI.

4. **Submit changes**
   - Use the submit tool to finalize changes.
