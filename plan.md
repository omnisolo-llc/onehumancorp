1. **Update `Store.byName` and `Store.byEmail` types to handle organization scope**
   - Change `byName map[string]*User` to `byName map[string]map[string]*User` (or composite key). Actually, looking at `Users`, the `organization_id` is crucial for cross-tenant data leakage. The `byName` should ideally map `orgID + "\x00" + username` -> `*User` or map[string]*User to keep changes minimal, or use a composite struct. Let's use `orgID + ":" + username` as composite key, or map of map. Wait, using composite key `orgID + "\x00" + username` is easiest. Let's use `orgID + ":" + username` or `orgID + "|" + username`. Even better: `type tenantKey struct { orgID, key string }`.

Let me use a `tenantKey` struct for byName, byEmail, and byOIDC.
```go
type tenantKey struct {
    orgID string
    key   string
}
```
Update `Store`:
```go
    byName  map[tenantKey]*User
    byEmail map[tenantKey]*User
    byOIDC  map[tenantKey]*User
```
Then update all places using `byName`, `byEmail`, `byOIDC`.
