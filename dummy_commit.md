Severity: CRITICAL
Vulnerability:
1. Thin Client SPIFFE Authentication Context Drop: The `spiffe_interceptor` in `src/server/lib.rs` successfully parsed `x-spiffe-id` but failed to inject the resulting `AuthInfo` into the gRPC request extensions.
2. Global Pool Prepared Statement Poisoning: The global Postgres connection pool (`GLOBAL_POOL`) instantiated in `src/server/db.rs` via `get_pool()` omitted `statement_cache_capacity=0`. Under PgBouncer/multitenant execution, prepared statement identifiers overlap, enabling cross-tenant SQL injection.
3. "System" Tenant IDOR Leakage: The application allowed users to explicitly register and authenticate with `organization_id == "system"`, bypassing tenant isolation in Cloud mode.

Impact:
1. Complete denial of service for Thin Client operations (AuthInfo missing downstream).
2. Catastrophic data leakage or execution across tenants if PgBouncer routes a prepared statement from one tenant's request to another.
3. IDOR capability to read system-level configurations and bypass multitenant restrictions.

Fix details:
1. Re-wrote `spiffe_interceptor` to properly map `AuthInfo` back to `req.extensions_mut()`.
2. Intercepted `get_pool()` creation to detect Postgres URLs and append `statement_cache_capacity=0` unconditionally.
3. Added robust checks across both the HTTP/gRPC routing (`mod.rs`) and storage layer (`postgres_store.rs`) to reject any explicit usage of the "system" `organization_id` when `multitenant` is enabled.
