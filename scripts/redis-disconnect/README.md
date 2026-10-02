# Legacy Redis disconnect regression probe

This small crate imports the complete existing sync, Redis-pool, compression and batching modules, including every original test. It uses the actual server_config and server_auth crates and byte-exact is_standalone_runtime/check_spiffe_auth function slices. Registry dependencies must match the repository lockfile. Generated paths and locks are ignored and regenerated locally.

Supply only an owned disposable loopback Redis process in OHC_TEST_REDIS_URL and REDIS_URL. The disconnect test runs CLIENT KILL TYPE pubsub, so never use a shared or production service. Supply the validated disposable PostgreSQL URL in OMNISOLO_DATABASE_URL to select the real hosted configuration; the Redis test does not query that database. Do not fake TEST_WORKSPACE or disable the production standalone policy.

The tested legacy api::sync handler is not mounted in the application; /api/v1/sync/ws mounts sync_gateway. Its original test-only x-mock-auth route is retained. These tests establish Redis disconnect/framing behavior and configuration prerequisites, not mounted production authorization. The required native ignored-test gate remains authoritative for the original full-crate selector.
