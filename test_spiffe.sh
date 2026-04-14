#!/bin/bash
cat << 'INNER_EOF' > /tmp/plan.md
1. **Create Migration:**
   - Create `srcs/server/db/migrations/049_telemetry_mesh.sql` with:
     ```sql
     -- +goose Up
     CREATE TABLE IF NOT EXISTS telemetry_buffer (
         id SERIAL PRIMARY KEY,
         metric_name TEXT NOT NULL,
         value REAL NOT NULL,
         labels_json TEXT,
         timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
         sync_status TEXT DEFAULT 'pending'
     );

     -- +goose Down
     DROP TABLE IF EXISTS telemetry_buffer;
     ```
2. **Implement Worker:**
   - Create `srcs/server/telemetry/mcp_sync_worker.go` that implements the worker, interacting with the DB provider and a stubbed HTTP call using SPIFFE IDs.

3. **Implement Tests:**
   - Create `srcs/server/telemetry/mcp_sync_worker_test.go` to test the sync worker using `db.NewTestProvider`.

4. **Update BUILD.bazel**: Add `mcp_sync_worker.go` and `mcp_sync_worker_test.go` to `srcs/server/telemetry/BUILD.bazel`.

5. **Pre-commit Steps**: Ensure `pre_commit_instructions` are followed.

6. **Mark Mission COMPLETE**: Mark mission file as done.
INNER_EOF
