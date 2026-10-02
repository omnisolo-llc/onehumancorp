# Onboarding durability regression harness

Run `scripts/onboarding-durability/run.sh` with `OHC_SYNC_TEST_DATABASE_URL`
pointing to an owned disposable PostgreSQL database. Never point this at customer
data. Tests create isolated schemas and roles and do not invoke provider APIs.

The harness compiles exact preparation helper and mounted onboarding API source,
extracts unchanged method bodies and persona definitions from OnboardingAgent,
and uses the real server_auth crate, signed tokens, revocation reads, tenant SQL
context and PostgreSQL transactions. All existing onboarding API unit tests are
included. Hub publication is recorded in memory. The complete production intake/chat
methods run with no configured model; any configured-provider request hits a
panic-only test boundary. Environment provisioning is disabled. This does not verify provider execution, downstream
notification delivery, SQLite onboarding, the full server suite or application E2E.

The dependency lock is derived from repository Cargo.lock; registry versions and
checksums must match. Generated source paths are checkout-local and never
committed. The runner verifies a before/after source manifest.

The extended harness also includes every existing onboarding service test body,
using an isolated authenticated PostgreSQL fixture instead of the main DB migration
bootstrap. Configured-provider generation is outside this gate: those tests deserialize
reviewed JSON with the production input type and persist the real catalog/variants.
The old stale-cache expectation now checks the latest committed state.

Five session-identity route tests run through the actual public server_auth HTTP
router and signed bearer validation. Their fixture uses the real PostgreSQL user
repository; test-only secret access and inactive-user mutation are adapted to the
isolated test secret and public repository update. No authorization assertions or
production route bodies are replaced. This does not certify the complete auth test
suite, outer application/native CORS behavior, or browser credential transport.

`GET /api/v1/auth/session-identity` returns only `userId`, `tenantId`, and Unix
millisecond `expiresAt`, with `Cache-Control: private, no-store`. It requires the
existing verified bearer transport; it creates no credentials and offers no
unauthenticated legacy fallback. Onboarding requests may send paired
`x-ohc-expected-user`/`x-ohc-expected-tenant` preconditions. If either is supplied,
both must be singular and exactly match the authenticated principal, otherwise
HTTP409 `session_identity_changed` is returned before any handler effects.

The three direct catalog regression cases use `catalog_test_adapter.rs` to wrap the real catalog conversion and transactional save methods. This adapter exists only in the focused test crate; production no longer carries an unused test helper. The existing invalid-money, exact-cents and variant-rollback assertions are unchanged.


The missing-provider cases compile the complete production intake and chat methods,
not substitutes returning a test-only error. They prove that no fabricated business,
product, price, customer or completion result is returned when the model is absent.
The three mounted authenticated endpoints return HTTP 503 with
`error: onboarding_ai_unconfigured` before preparation or catalog mutation.
The manual `/start` path still persists the exact reviewed user-supplied catalog
and business identity without a model, and does not automatically launch it.
All 64 prior cases remain, alongside six provider/manual regressions (70 total).
