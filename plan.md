1. **Create Advanced AutoDream Implementation:**
   - Execute the following command to create `srcs/server/orchestration/autodream_advanced.go`:
   ```bash
   cat << 'EOF' > srcs/server/orchestration/autodream_advanced.go
   package orchestration

   import (
       "context"
       "fmt"
       "log/slog"
       "time"

       "github.com/google/uuid"
       "github.com/onehumancorp/mono/srcs/server/db"
   )

   type AutoDreamAdvanced struct {
       pool   db.Provider
       client MinimaxClient
   }

   func NewAutoDreamAdvanced(pool db.Provider, client MinimaxClient) *AutoDreamAdvanced {
       return &AutoDreamAdvanced{pool: pool, client: client}
   }

   func (a *AutoDreamAdvanced) PruneStaleAgentSessions(ctx context.Context) error {
       var query string
       if a.pool.IsSQLite() {
           query = "DELETE FROM agent_session_data WHERE last_accessed < datetime('now', '-30 days')"
       } else {
           query = "DELETE FROM agent_session_data WHERE last_accessed < CURRENT_TIMESTAMP - INTERVAL '30 days'"
       }
       res, err := a.pool.Exec(ctx, query)
       if err != nil {
           return fmt.Errorf("failed to prune stale agent sessions: %w", err)
       }
       slog.Info("AutoDreamAdvanced: Pruned stale agent sessions", "deleted_rows", res)
       return nil
   }

   func (a *AutoDreamAdvanced) ResolveConflicts(ctx context.Context, memoryID string) error {
       // Query conflicts using pgvector
       var query string
       if a.pool.IsSQLite() {
           query = `SELECT id, content FROM autodream_memories WHERE id != $1 LIMIT 5`
       } else {
           query = `SELECT id, content FROM autodream_memories WHERE id != $1 ORDER BY embedding <-> (SELECT embedding FROM autodream_memories WHERE id = $1) ASC LIMIT 5`
       }

       rows, err := a.pool.Query(ctx, query, memoryID)
       if err != nil {
           return err
       }
       defer rows.Close()

       var conflictIDs []string
       for rows.Next() {
           var id, content string
           if err := rows.Scan(&id, &content); err == nil {
               conflictIDs = append(conflictIDs, id)
           }
       }

       if len(conflictIDs) > 0 {
           // Delete conflicting records to resolve
           for _, id := range conflictIDs {
               _, _ = a.pool.Exec(ctx, "DELETE FROM autodream_memories WHERE id = $1", id)
           }
           slog.Info("AutoDreamAdvanced: Resolved conflicts", "deleted_ids", conflictIDs)
       }
       return nil
   }

   func (a *AutoDreamAdvanced) InjectTruth(ctx context.Context, orgID, agentID, content string) error {
       embedding := make([]float32, 1536)
       if a.client != nil {
           ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
           resp, err := a.client.GenerateEmbedding(ctxTimeout, content)
           cancel()
           if err == nil && len(resp) == 1536 {
               embedding = resp
           }
       }
       embStr := formatFloat32SliceForVector(embedding)
       id := uuid.New().String()

       var query string
       if a.pool.IsSQLite() {
           query = `INSERT INTO autodream_memories (id, content, embedding, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, 'truth', CURRENT_TIMESTAMP)`
       } else {
           query = `INSERT INTO autodream_memories (id, content, embedding, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, 'truth', CURRENT_TIMESTAMP)`
       }
       _, err := a.pool.Exec(ctx, query, id, content, embStr, orgID, agentID)
       if err != nil {
           return fmt.Errorf("failed to inject truth: %w", err)
       }
       slog.Info("AutoDreamAdvanced: Injected truth", "id", id)
       return nil
   }

   func (a *AutoDreamAdvanced) SearchTruth(ctx context.Context, orgID, queryText string, limit int) ([]string, error) {
       embedding := make([]float32, 1536)
       if a.client != nil {
           ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
           resp, err := a.client.GenerateEmbedding(ctxTimeout, queryText)
           cancel()
           if err == nil && len(resp) == 1536 {
               embedding = resp
           }
       }
       embStr := formatFloat32SliceForVector(embedding)

       var query string
       if a.pool.IsSQLite() {
           query = `SELECT content FROM autodream_memories WHERE organization_id = $1 AND source_type = 'truth' LIMIT $2`
       } else {
           query = `SELECT content FROM autodream_memories WHERE organization_id = $1 AND source_type = 'truth' ORDER BY embedding <-> $2::vector ASC LIMIT $3`
       }

       var rows db.Rows
       var err error
       if a.pool.IsSQLite() {
           rows, err = a.pool.Query(ctx, query, orgID, limit)
       } else {
           rows, err = a.pool.Query(ctx, query, orgID, embStr, limit)
       }
       if err != nil {
           return nil, err
       }
       defer rows.Close()

       var results []string
       for rows.Next() {
           var content string
           if err := rows.Scan(&content); err == nil {
               results = append(results, content)
           }
       }
       return results, nil
   }
   EOF
   cat srcs/server/orchestration/autodream_advanced.go
   ```

2. **Add Missing Tests:**
   - Execute the following command to create `srcs/server/orchestration/autodream_advanced_test.go`:
   ```bash
   cat << 'EOF' > srcs/server/orchestration/autodream_advanced_test.go
   package orchestration

   import (
       "context"
       "testing"
       "time"
   )

   func TestAutoDreamAdvanced_PruneStaleAgentSessions(t *testing.T) {
       provider := setupTestDB(t)
       advanced := NewAutoDreamAdvanced(provider, nil)

       _, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS agent_session_data (
           session_id TEXT PRIMARY KEY,
           agent_id TEXT NOT NULL,
           context_data TEXT NOT NULL,
           created_at TEXT DEFAULT CURRENT_TIMESTAMP,
           last_accessed TEXT DEFAULT CURRENT_TIMESTAMP
       )`)
       if err != nil {
           t.Fatalf("failed to create agent_session_data: %v", err)
       }

       _, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('sess-1', 'agent-1', 'data', datetime('now', '-40 days'))")
       if err != nil {
           t.Fatalf("failed to insert stale session: %v", err)
       }

       err = advanced.PruneStaleAgentSessions(context.Background())
       if err != nil {
           t.Fatalf("failed to prune: %v", err)
       }

       var count int
       err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
       if err != nil {
           t.Fatalf("failed to count: %v", err)
       }
       if count != 0 {
           t.Errorf("expected 0 sessions, got %d", count)
       }
   }

   func TestAutoDreamAdvanced_InjectTruth(t *testing.T) {
       provider := setupTestDB(t)
       advanced := NewAutoDreamAdvanced(provider, nil)

       err := advanced.InjectTruth(context.Background(), "org-1", "agent-1", "truth content")
       if err != nil {
           t.Fatalf("failed to inject truth: %v", err)
       }
   }
   EOF
   cat srcs/server/orchestration/autodream_advanced_test.go
   ```

3. **Update BUILD.bazel:**
   - Run `sed -i 's|"autodream_worker.go",|"autodream_worker.go",\n        "autodream_advanced.go",|g' srcs/server/orchestration/BUILD.bazel`
   - Run `sed -i 's|"autodream_worker_test.go",|"autodream_worker_test.go",\n        "autodream_advanced_test.go",|g' srcs/server/orchestration/BUILD.bazel`
   - Run `cat srcs/server/orchestration/BUILD.bazel` to verify.

4. **Verify Implementation:**
   - Run `bazelisk test //srcs/server/orchestration/...` to verify the changes and ensure no regressions were introduced.

5. **Pre-commit Instructions:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
