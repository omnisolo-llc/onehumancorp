# Onboarding durability regression harness

Run `scripts/onboarding-durability/run.sh` with `OHC_SYNC_TEST_DATABASE_URL`
pointing to an owned disposable PostgreSQL database. Never point this at customer
data. Tests create isolated schemas and roles and do not invoke provider APIs.

The harness compiles exact preparation helper and mounted onboarding API source,
extracts unchanged method bodies and persona definitions from OnboardingAgent,
and uses the real server_auth crate, signed tokens, revocation reads, tenant SQL
context and PostgreSQL transactions. All existing onboarding API unit tests are
included. Hub publication is recorded in memory; provider intake and environment
provisioning are disabled. This does not verify provider execution, downstream
notification delivery, SQLite onboarding, the full server suite or application E2E.

The dependency lock is derived from repository Cargo.lock; registry versions and
checksums must match. Generated source paths are checkout-local and never
committed. The runner verifies a before/after source manifest.
