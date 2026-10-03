# Tenant-scoped search contract

Run `OHC_SEARCH_TEST_DATABASE_URL=<owned disposable PostgreSQL database> bash scripts/tenant-search/run.sh`.

The harness compiles the entire production search module and actual bearer authentication, signed users and pool-reset helper. It derives the three table definitions and RLS policies from the maintained migrations, creates an isolated schema and restricted LOGIN role, and verifies FORCE ROW LEVEL SECURITY with one pooled connection. Fixture cleanup drops only its own schema and role. This is real PostgreSQL HTTP-handler evidence, not full application/browser or MySQL/SQLite certification. Registry dependencies must match the repository lockfile. No provider or customer data is accessed.
