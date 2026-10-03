# Staff read truthfulness contract

This source-bound harness compiles the exact three production staff/task/summary read handlers and DTOs, the actual signed-tenant extractor and genuine SQLx pools. The small DB container and Claims insertion are test setup boundaries; this is not the full authentication middleware/server stack.

SQLite and disposable restricted-role PostgreSQL tests distinguish confirmed empty results, real tenant records, missing storage and invalid tenant claims. The PostgreSQL fixture applies the actual additive native staff migration and forces owner RLS. No provider calls or live data are involved. Complete server, browser and all other staff mutation contracts remain separate acceptance requirements.
