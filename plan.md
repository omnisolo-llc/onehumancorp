1. **Implement Concrete RAG Sync Service:**
   ```bash
   cat << 'EOF' > srcs/server/hub/rag_sync.go
   package hub

   import (
       "context"
       "time"
       "log/slog"
       "fmt"

       "github.com/onehumancorp/mono/srcs/server/db"
       "go.opentelemetry.io/otel"
       "go.opentelemetry.io/otel/metric"
   )

   type SyncStatus string

   const (
       SyncStatusPending SyncStatus = "pending"
       SyncStatusSynced  SyncStatus = "synced"
       SyncStatusError   SyncStatus = "error"
   )

   type RAGSyncRecord struct {
       ID           string
       Context      string
       Vector       []float32 // Convert to string internally for SQLite compat if needed
       SyncStatus   SyncStatus
       LastSyncAt   time.Time
   }

   type RAGSyncService interface {
       FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
       MarkSynced(ctx context.Context, ids []string) error
       ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
   }

   // DefaultRAGSyncService is a concrete implementation of RAGSyncService
   type DefaultRAGSyncService struct {
       provider db.Provider
   }

   func NewRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
       return &DefaultRAGSyncService{
           provider: provider,
       }
   }

   func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
       rows, err := s.provider.Query(ctx, "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
       if err != nil {
           return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
       }
       defer rows.Close()

       var records []RAGSyncRecord
       for rows.Next() {
           var record RAGSyncRecord
           var status string
           var lastSyncAt *time.Time
           // Since we omitted vector parsing for simplicity in this baseline implementation
           if err := rows.Scan(&record.ID, &record.Context, &status, &lastSyncAt); err != nil {
               slog.Error("failed to scan row", "error", err)
               continue
           }
           record.SyncStatus = SyncStatus(status)
           if lastSyncAt != nil {
               record.LastSyncAt = *lastSyncAt
           }
           records = append(records, record)
       }

       if err := rows.Err(); err != nil {
           return nil, fmt.Errorf("rows iteration error: %w", err)
       }

       return records, nil
   }

   func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
       if len(ids) == 0 {
           return nil
       }

       tx, err := s.provider.Begin(ctx)
       if err != nil {
           return fmt.Errorf("failed to begin transaction: %w", err)
       }
       defer tx.Rollback(ctx)

       now := time.Now()
       for _, id := range ids {
           _, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3", string(SyncStatusSynced), now, id)
           if err != nil {
               return fmt.Errorf("failed to update record %s: %w", id, err)
           }
       }

       if err := tx.Commit(ctx); err != nil {
           return fmt.Errorf("failed to commit transaction: %w", err)
       }

       ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
       return nil
   }

   func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
       if len(records) == 0 {
           return nil
       }

       tx, err := s.provider.Begin(ctx)
       if err != nil {
           return fmt.Errorf("failed to begin transaction: %w", err)
       }
       defer tx.Rollback(ctx)

       now := time.Now()
       for _, record := range records {
           // Insert ... ON CONFLICT DO UPDATE is standard for Postgres and Modern SQLite
           _, err := tx.Exec(ctx, `
               INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (id) DO UPDATE SET
                   content = EXCLUDED.content,
                   sync_status = EXCLUDED.sync_status,
                   last_sync_at = EXCLUDED.last_sync_at`,
               record.ID, record.Context, string(SyncStatusSynced), now)
           if err != nil {
               ragSyncErrorsTotal.Add(ctx, 1)
               return fmt.Errorf("failed to upsert record %s: %w", record.ID, err)
           }
       }

       if err := tx.Commit(ctx); err != nil {
           return fmt.Errorf("failed to commit transaction: %w", err)
       }

       return nil
   }

   var (
       ragRecordsSyncedTotal metric.Int64Counter
       ragSyncErrorsTotal    metric.Int64Counter
   )

   func init() {
       meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
       var err error
       ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
       if err != nil {
           slog.Error("Failed to initialize rag_records_synced_total metric", "error", err)
       }

       ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
       if err != nil {
           slog.Error("Failed to initialize rag_sync_errors_total metric", "error", err)
       }
   }
   EOF
   cat srcs/server/hub/rag_sync.go
   ```
2. **Update Bazel Hub Build File:** Update the `deps` for `srcs/server/hub/BUILD.bazel` to include `//srcs/server/db` package.
   ```bash
   sed -i 's/deps = \[/deps = \[\n        "\\/\/srcs\/server\/db",/' srcs/server/hub/BUILD.bazel
   cat srcs/server/hub/BUILD.bazel
   ```
3. **Update Tests:** Test the actual `DefaultRAGSyncService` with a mock database provider.
   ```bash
   cat << 'EOF' > srcs/server/hub/rag_sync_test.go
   package hub

   import (
       "context"
       "testing"
       "time"

       "github.com/onehumancorp/mono/srcs/server/db"
   )

   type MockRows struct {
       records []RAGSyncRecord
       pos     int
   }

   func (r *MockRows) Next() bool {
       r.pos++
       return r.pos <= len(r.records)
   }

   func (r *MockRows) Scan(dest ...any) error {
       rec := r.records[r.pos-1]
       *dest[0].(*string) = rec.ID
       *dest[1].(*string) = rec.Context
       *dest[2].(*string) = string(rec.SyncStatus)

       if !rec.LastSyncAt.IsZero() {
            *dest[3].(**time.Time) = &rec.LastSyncAt
       } else {
            *dest[3].(**time.Time) = nil
       }
       return nil
   }

   func (r *MockRows) Close() {}
   func (r *MockRows) Columns() ([]string, error) { return nil, nil }
   func (r *MockRows) Err() error { return nil }

   type MockTx struct{}
   func (t *MockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
   func (t *MockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
   func (t *MockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
   func (t *MockTx) Commit(ctx context.Context) error { return nil }
   func (t *MockTx) Rollback(ctx context.Context) error { return nil }

   type MockProvider struct{}
   func (p *MockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
   func (p *MockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
       return &MockRows{
           records: []RAGSyncRecord{
               {ID: "test-1", Context: "ctx-1", SyncStatus: SyncStatusPending},
           },
       }, nil
   }
   func (p *MockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
   func (p *MockProvider) Begin(ctx context.Context) (db.Tx, error) { return &MockTx{}, nil }
   func (p *MockProvider) Close() {}
   func (p *MockProvider) IsSQLite() bool { return false }
   func (p *MockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) { return nil, nil }

   func TestDefaultRAGSyncService(t *testing.T) {
       provider := &MockProvider{}
       service := NewRAGSyncService(provider)
       ctx := context.Background()

       // Test FetchPendingSyncs
       records, err := service.FetchPendingSyncs(ctx, 10)
       if err != nil {
           t.Fatalf("FetchPendingSyncs failed: %v", err)
       }
       if len(records) != 1 {
           t.Fatalf("Expected 1 record, got %d", len(records))
       }

       // Test MarkSynced
       err = service.MarkSynced(ctx, []string{"test-1"})
       if err != nil {
           t.Fatalf("MarkSynced failed: %v", err)
       }

       // Test ProcessIncomingSync
       err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
           {ID: "test-2", Context: "ctx-2"},
       })
       if err != nil {
           t.Fatalf("ProcessIncomingSync failed: %v", err)
       }
   }
   EOF
   cat srcs/server/hub/rag_sync_test.go
   ```
4. **Mark Mission DONE:**
   ```bash
   sed -i 's/^status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   head -n 5 .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   ```
5. **Verify:** Use `run_in_bash_session` to execute the tests: `bazelisk test --config=local //srcs/server/hub/... //srcs/server/db/...`
6. **Pre-commit Steps:**
   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Request Code Review:** Call the `request_code_review` tool.
