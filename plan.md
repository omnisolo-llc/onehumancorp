1. **Update SQL Migrations**
   - Create migration `srcs/server/db/migrations/049_consolidated_memory.sql`:
     ```sql
     -- +goose Up
     -- +goose StatementBegin
     ALTER TABLE consolidated_memory ADD COLUMN metadata JSONB;
     -- +goose StatementEnd

     -- +goose Down
     -- +goose StatementBegin
     -- SQLite has limited ALTER TABLE support so dropping columns isn't strictly reliable here, but we will provide best-effort.
     -- Actually, we can skip Down statement for SQLite compatible mode if not strictly needed, or just keep it simple.
     ALTER TABLE consolidated_memory DROP COLUMN metadata;
     -- +goose StatementEnd
     ```
   - Update `srcs/server/db/BUILD.bazel` to include `"migrations/049_consolidated_memory.sql"`.

2. **Update `AutoDreamWorker` Implementation**
   - In `srcs/server/orchestration/autodream_worker.go`, modify `ProcessMemories` function.
   - Fetch the organization ID using `orgID := auth.OrganizationIDFromContext(ctx)`.
   - Following memory constraints: "When calling `auth.OrganizationIDFromContext(ctx)` in background workers (like `AutoDreamWorker`) that lack HTTP request contexts, the function will return an empty string. You must explicitly handle this by rejecting or failing the task (or using a dead-letter queue)—never default to 'system', as this causes privilege elevation and introduces a critical security vulnerability."
   - If `orgID == ""`, return an error instead of continuing: `return fmt.Errorf("missing organization_id in context")`.
   - Update the insert statements to insert into `consolidated_memory` instead of `autodream_memories`.
   - `consolidated_memory` columns: `id, organization_id, agent_id, content, embedding, source_type, created_at, metadata`
   - Adjust the query parameter logic accordingly. We don't need `source_mission_id`, but we can put it in `metadata` or just omit it if not mapped. Wait, the `consolidated_memory` table schema from migration `027_consolidated_memory.sql` is:
     `id, organization_id, agent_id, content, embedding, source_type, created_at`. (And we will add `metadata`).
   - Example query for PostgreSQL:
     ```go
     query = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at) VALUES ($1, $2, $3, $4, $5::vector, $6, CURRENT_TIMESTAMP)`
     ```
     For SQLite:
     ```go
     query = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)`
     ```
     With arguments: `args = []interface{}{memID, orgID, "auto-dream-worker", contentToEmbed, embStr, "background-pipeline"}`.
     Wait, I'll store the `missionID` in the `metadata` column: `metadataJSON := fmt.Sprintf("{\"source_mission_id\": %q}", missionID)`.
     Then add `metadata` to the columns.

3. **Update Tests**
   - Update `srcs/server/orchestration/autodream_worker_test.go` to insert into `consolidated_memory` table instead.
   - Create the table with `metadata` column in the test setup.
   - Run the test with a context that has the `orgID` claim injected:
     `ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})`.
   - Add a test that verifies it fails when the context is missing `organization_id`.

4. **Verify Implementation**
   - Run tests: `bazelisk test //srcs/server/orchestration/...`

5. **Pre commit & PR submission**
   - Run `pre_commit_instructions`.
   - Submit PR with the title `🗺️ Guide: [KAIROS AutoDream Memory Consolidation]`.
