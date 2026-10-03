# Verified appointment reads

This mandatory PostgreSQL gate executes the actual authenticated appointment read module with the real Store middleware and source-derived canonical migrations001,162 and222. It creates an owned isolated schema and a login role without superuser or RLS-bypass rights, then forces RLS on all read tables. A one-connection pool proves tenant context does not survive reads. Additional table-owner reads exercise explicit join/query guards without relying on RLS.

Set OHC_APPOINTMENTS_TEST_DATABASE_URL to a dedicated loopback ohc_*_test database. The runner rejects unsafe targets before Cargo or database use. Run through scripts/focused_ci_gate.py operations-appointments. Missing database configuration is an error, never a skipped test. The lock must match the repository dependency graph, and source fingerprints bind the actual query/module, migrations and auth code to the result.

This proves read behavior and isolation only. Appointment writes, messaging, route optimization and reminder/provider dispatch have separate authority and outcome contracts. The operations page displays recorded status and browser-local times; neither establishes payment, AI analysis or a scheduled notification. Actual browser acceptance uses a uniquely owned database fixture and retains real HTTP receipts.
