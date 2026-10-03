# Agent-feed decision integrity contract

`source /workspace/shared/recovered-toolchains/env.sh` is a local recovery-environment convenience, not a repository dependency. Normal prerequisites are the documented Rust toolchain, PostgreSQL server tools and Redis. Use `with-owned-postgres.sh` to create disposable loopback services or provide an explicit `OHC_FEED_TEST_DATABASE_URL` with an `ohc_*_test` database plus `OHC_FEED_TEST_REDIS_URL`.

The harness imports actual production HTTP, repository, queue, worker, canonical authentication and migration code. For the unmodified baseline, the decision HTTP function is copied verbatim into a generated module; after extraction the complete production decisions module is imported by path. The source manifest records all inputs and is verified unchanged after tests.

The production action router is imported unchanged. The business action exercised here is the real `create_product` database handler. Provider handlers outside this contract are explicit panic boundaries, never fake successful delivery implementations. Cache invalidation and WebSocket publication are inert because they cannot establish database/dispatch correctness. No production database or live provider/work dispatch is permitted. This focused gate does not replace `make lint` and `make test`.

The inventory includes the existing source module's optional PostgreSQL lifecycle test, which returns early under the dummy standalone configuration. The 32 `contract::` tests require real owned PostgreSQL and do not skip. Do not count the inherited early return as additional PostgreSQL coverage. Worker side-effect fixtures use an owned privileged pool to count local inserts; the separate restricted-role/RLS case records the existing bare-pool handler failure truthfully.
