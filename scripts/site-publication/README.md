# Site publication contract gate

Compiles the complete maintained publication store, deterministic renderer, durable worker and builder database module with real common tenant binding. Tests require an explicit loopback PostgreSQL `ohc_*_test` database without connection override parameters. Each database fixture uses canonical table statements and a real restricted LOGIN with forced RLS. No provider, payment, DNS, CDN deployment or public customer publication is performed.

The gate exercises atomic receipt/snapshot/routing commit, exact raw-tenant ownership, immutable rendering, leased restart recovery, late-worker/version fencing, actual final-commit rejection, owner/product revocation races, and the background consumer's shutdown. Four preflight checks use a recording native-command stub to prove invalid destinations stop before any database/native work. Source fingerprints and root-lock dependency equivalence are mandatory.

This standalone gate does not certify application startup mounting, public HTTP/cache delivery, authenticated publication UI or browser acceptance. Those remain separate stages in the reviewed publication plan.
