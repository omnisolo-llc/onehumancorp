# Existing campaign test discovery

The actual inline `services` module in `src/server/lib.rs` includes campaign only
under `cfg(test)`. This restores the six existing ignored PostgreSQL cases and
three ordinary cases without mounting or enabling the dormant service.

The native gate is authoritative for compilation with the real integrations
registry. Select `services::campaign::service::tests::` and run with
`--include-ignored`; exactly nine cases must be discovered and completed.
Set `OHC_CAMPAIGN_TEST_DATABASE_URL` to a disposable PostgreSQL database. The
fixture creates UUID-named schemas and restricted LOGIN roles, uses the exact
canonical tenant DDL and campaign migration, and forces RLS. It explicitly checks
both session and current identity and cleans up successful fixtures. Retire the
whole disposable PostgreSQL instance after failed tests, which can leave test
schemas/roles. Never point the variable at an application/customer database.

For a small source-bound local check:

```sh
OHC_CAMPAIGN_TEST_DATABASE_URL=... bash scripts/campaign-durability/run.sh
```

This harness imports the whole unchanged service, models and repositories, with
all their original tests. It also imports the real authentication/protobuf crates.
Its only service adapter is a negative provider sentinel: every attempted
SendGrid/Twilio/Meta operation panics. The existing successful activation case uses
the service's recording dispatcher and asserts both its exact invocation and the
real PostgreSQL channel record. The missing-configuration case uses explicit
empty configuration and the real registry dispatcher logic, never ambient provider
credentials. This does not prove provider delivery, mounted gRPC authentication,
production campaign authorization, or a full-server build.

The runner verifies all registry dependencies against the repository lock and
rejects source changes during execution. Native CI must retain all original tests;
this focused harness is additional evidence, not a replacement for full gates.
