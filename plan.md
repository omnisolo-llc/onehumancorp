1. **Explore `rueidis` initialization and config checking**: Review `srcs/server/tools/hybridfsmcp/mcp.go` again to see how we should handle the injected dependencies for `kvmcp`.
2. **Create Database Migration (`agent_kv_store`)**:
   - `srcs/server/db/migrations/20260429000000_agent_kv_store_pg.sql`
   - `srcs/server/db/migrations/20260429000000_agent_kv_store_sqlite.sql`
3. **Create `srcs/server/tools/kvmcp/BUILD.bazel`**: To declare the bazel target.
4. **Create `srcs/server/tools/kvmcp/mcp.go`**: Implement the `KVMCP` struct, supporting `kv_get`, `kv_set`, `kv_delete`, `kv_list`.
   - Take `db.Provider` and `rueidis.Client` as parameters in the constructor.
   - Use `envBoolDefault("OHC_STANDALONE", false)` or similar logic.
   - For Cloud mode (Redis), use `tenant:{org_id}:kv:{key}`.
   - For Standalone mode (SQLite), use the `agent_kv_store` table.
5. **Create `srcs/server/tools/kvmcp/mcp_test.go`**: Implement tests to reach >90% coverage for `kvmcp` package.
6. **Integrate into `srcs/server/tools/tools.go`**: (If necessary, though `tools.go` seems just a dummy/build tag file right now based on previous output).
