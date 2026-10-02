# Source-bound builder generation contract

Run `OHC_BUILDER_GENERATION_TEST_DATABASE_URL=postgres://.../ohc_builder_generation_test bash scripts/builder-generation-contract/run.sh`.

The loopback disposable database is mandatory. Tests use actual configured
OpenAI-compatible HTTP transport, actual signed authority and revocation checks,
production JSON validation and actual Postgres storage under a restricted
RLS-enforced role. Fixtures contain synthetic test records only; production
sources contain no successful fake generator. `prepare.py` imports production
files and copies the exact production inference adapter, checks dependencies
against the repository Cargo.lock and fingerprints all covered sources.

These are focused local contract tests, not live-provider or full-server
certification. Full Cargo/Next/desktop and hosted CI gates remain required.

The combined gate preserves the original 32 generation/admission cases, 15 imported durable receipt lifecycle cases and three funded admission cases. Its mandatory CI floor is 50. Persistence modules, receipt sources and all included SQL are source-fingerprinted.
