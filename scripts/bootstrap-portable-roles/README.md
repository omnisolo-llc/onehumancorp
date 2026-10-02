# Bootstrap roles through portable authentication

Run `OHC_SETUP_TEST_DATABASE_URL=<owned disposable PostgreSQL database> bash scripts/bootstrap-portable-roles/run.sh`.

The harness compiles every production setup function and the real SeaORM authentication repository. Its database handle exposes the same pool/store fields consumed by setup; it does not substitute a query or role reader. PostgreSQL tables use the maintained initial migration and actual ORM identity entities, with a disposable schema and restricted reader. SQLite uses the same identity entities. This verifies setup persistence and later role reads without launching agent work, connecting a provider, or certifying the full Kind deployment.
