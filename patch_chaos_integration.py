import re

with open("srcs/server/orchestration/chaos_integration_test.go", "r") as f:
    code = f.read()

# Let's fix the SQL syntax error for Postgres (pgx)
# it seems PostgresMock in the test is actually using SQLite because OHC_STANDALONE=false mock isn't perfectly mapped to actual postgres binary in these tests.
# Actually, wait, dbInstance is created using `NewSIPDB(dbPath)`.
# `NewSIPDB` creates a SQLite db if the path is not a postgres connection string!
# "Failed to insert bursting mission: SQL logic error: near "'2 days'": syntax error"
# This means it's running SQLite syntax!
# Let's just use `datetime('now', '-2 days')` and remove the `CURRENT_TIMESTAMP - INTERVAL` since NewSIPDB on a file path is always SQLite in the test environment (unless connection string provided).
# The whole test block is testing the interface with SQLite backend underneath because that's how NewSIPDB is implemented for mock file paths.

target = "INSERT INTO agent_missions (id, status, payload, created_at, organization_id) VALUES ('burst-pg', 'BURSTING', '{}', CURRENT_TIMESTAMP - INTERVAL '2 days', 'system')"
new_target = "INSERT INTO agent_missions (id, status, payload, created_at, organization_id) VALUES ('burst-pg', 'BURSTING', '{}', datetime('now', '-2 days'), 'system')"

code = code.replace(target, new_target)

with open("srcs/server/orchestration/chaos_integration_test.go", "w") as f:
    f.write(code)
