# Service creation contract

This gate compiles the whole maintained create-service router, including its three existing SQLite/auth cases, against an explicit database adapter and real auth/common crates. PostgreSQL cases use the canonical service DDL from migration008, a private schema, and a real restricted LOGIN role with forced RLS. They exercise signed-bearer requests, forged tenant hints, exact-cent persistence, invalid input, and commit rejection. No provider or public service is called. Full main-server and browser execution remain separate gates.

Run `OHC_SERVICE_TEST_DATABASE_URL=<owned disposable URL> bash scripts/service-creation/run.sh`. Missing opt-in fails before connection. All harness registry dependencies must match the repository lock.
