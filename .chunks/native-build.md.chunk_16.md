able or Cargo feature to re-enable them.
The native browser runner applies explicit SQL fixtures only to the PostgreSQL
container it just created. A private per-run proof binds fixture SQL helpers to
that running container's ID, generated run label, database and loopback port;
arbitrary `DATABASE_URL` values are rejected before any SQL connection. The runner
then authenticates its fixture owner through the real login and requires HTTP 404
for all 18 retired backend fixture routes before browser execution. Unit tests of
the isolation helper and HTTP probe are not a substitute for that real-stack gate.

Compose verification loads its explicit SQL fixture into its own project-verified
PostgreSQL container, then checks exact record IDs through the real authenticated
UI APIs. Kind verification creates and reloads a vendor through the real API in
both database modes. Neither uses a production fixture endpoint. The retired
operator seeder fails closed and never modifies an existing installation.

Historical `*.mock-contract.ts` files remain as an inventory of unproven flows;
they are not evidence of working production capabilities. This boundary repair
does not claim to implement automatic review solicitation, referral checkout,
newsletter generation, invoice drafting, or other simulated provider workflows.
Those capabilities still require real implementations and their own acceptance.

The native E2E runner starts isolated PostgreSQL/Valkey containers, the real Rust binaries and the newly built web package. It seeds only its own database and reconstructs an environment without production database or provider credentials. CI shards the complete browser-spec discovery. Per-test filters are available for local diagnosis without changing CI discovery.

Provider-boundary test servers can supply deterministic external contracts; the internal UI/API/database must remain real. Label those results as contract verification, not live provider or customer evidence. Billing/provider f