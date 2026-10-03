# Site publication contract gate

Compiles the complete maintained publication store, deterministic renderer, durable worker, read-only public page projection and builder database module with real common tenant binding. Tests require an explicit loopback PostgreSQL `ohc_*_test` database without connection override parameters. Each database fixture uses canonical table statements and a real restricted LOGIN with forced RLS. No provider, payment, DNS, CDN deployment or public customer publication is performed.

The gate exercises atomic receipt/snapshot/routing commit, exact raw-tenant ownership, immutable rendering, leased restart recovery, late-worker/version fencing, actual final-commit rejection, owner/product revocation races, and the background consumer's shutdown. Four preflight checks use a recording native-command stub to prove invalid destinations stop before any database/native work. Source fingerprints and root-lock dependency equivalence are mandatory.

The public projection reuses the immutable internal routing metadata. Completed jobs leave a nonqueued route; the route itself cannot authorize content. A partial index covers queued work independently of historical routes. Every read rechecks the current publication pointer, receipt digest, current owner and selected products under the same authority lock order as the worker. Transaction-local3s statement/1s lock timeouts fail closed on prolonged contention. The worker also bounds discovery and each item to5s and observes shutdown during storage waits; cancellation leaves ambiguous committed state for durable reconciliation. Tests cover actual revocation waits, a persistent authority-lock timeout and subsequent pool recovery, missing routes, tenant deletion, raw-tenant UUID collisions, foreign-site repoint rejection and the common unambiguous document-path contract.

The complete production router assembly runs with strict bearer middleware and real restricted-role PostgreSQL. HTTP tests cover owner admission/replay/revocation, public response headers, raw JSON admission, the actual polling worker, immutable product documents and cross-language receipt digests using pinned JavaScript `canonicalize` 5.1.0. `test_mount.py` guards application registration and PostgreSQL-only startup; it is a source guard, not full application startup proof.

Use pinned Node22 and install the tiny witness tree with `npm ci --prefix scripts/site-publication --ignore-scripts`. `verify_node_lock.py` requires the witness and maintained Next manifest/lock to agree on canonicalize5.1.0 and its registry integrity. A previously installed identical client dependency can also resolve through the Next module path. The digest witness executes JavaScript against both the submitted snapshot and the actual PostgreSQL JSONB value. It does not substitute fixture hashes for database output.

This focused gate does not certify the complete server build, authenticated publication UI, Next proxy or browser acceptance. Those remain separate acceptance gates. The implementation is application-hosted PostgreSQL publication; it does not deploy DNS, CDN, custom domains or external providers.

The focused Cargo lock is checked in so an isolated CI lane can run `bash scripts/site-publication/fetch.sh` before the offline gate. That bounded setup fetches only this harness for the host target with `--locked`, and checks every registry source/version/checksum against the root lock both before and after fetching. Tests retain `--locked --offline`; missing dependency or integrity failures remain failures. Root dependency updates must refresh and reverify this focused lock explicitly.

The main-composition regression derives the publication mount position from the
application source and runs the actual global bearer/tenant middleware around the
real publication router. It covers anonymous HTML, current owner authorization and
anonymous 404 after exact-version revocation. This catches public routes captured
by a later global authentication layer; isolated child-router tests cannot prove
that boundary. The tier layer only applies action limits to protected/autodream
paths and is not compiled into this focused boundary. Full hosted browser acceptance
remains required.

The gate also compiles the complete mutable storefront router, builder edge
renderer, production cache, and shared response-cache middleware. Its 93 cases
retain all 81 publication cases. Signed requests cannot reuse a public URI cache;
current owner and tenant checks precede private product/domain cache reads. A
deleted product cannot reappear from an old entry, and foreign or mixed-tag
invalidation requests leave both tenants' caches unchanged. Private directives
and Set-Cookie responses remain private and cannot enter the shared cache.

The runner owns a loopback HTTP denial proxy. It never forwards traffic and fails
on any non-loopback HTTP attempt, while retaining real local database and HTTP
fixtures. These tests do not send edge purge requests. Private cold rendering,
legacy SEO escaping and truthful inventory fallback have separate outstanding
regressions; the cache/privacy proof does not certify those paths. Public
owner-reviewed snapshots remain available through the immutable publication API.
