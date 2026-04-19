1. **Update `Store` struct to support multi-tenant isolation in `byName`, `byEmail`, and `byOIDC` maps.**
   - Change `byName map[string]*User` to `byName map[tenantKey]*User`.
   - Change `byEmail map[string]*User` to `byEmail map[tenantKey]*User`.
   - Change `byOIDC map[string]*User` to `byOIDC map[tenantKey]*User`.
   - Add `type tenantKey struct { orgID, key string }` to `store.go`.
2. **Update initialization and operations in `store.go`.**
   - In `newStore`, initialize maps with `tenantKey`.
   - Update `CreateUser`, `Authenticate`, `UpdateUser`, `DeleteUser`, `GetOrCreateOIDCUser` to use `tenantKey{orgID: orgID, key: ...}`.
   - For `sys` org or empty org, we need to handle it. Actually, `admin` is created with empty `""` org. So `tenantKey{orgID: "", key: adminUser}` is fine.
   - Update `UpdateUser`: ensure the email uniqueness check uses the correct org ID. `if _, exists := s.byEmail[tenantKey{orgID: u.OrganizationID, key: *emailPtr}]; exists { ... }`
3. **Run tests and verify.**
   - Run `bazelisk test //srcs/server/auth/...`.
   - Run `bazelisk test //...`
   - Complete pre-commit steps.
