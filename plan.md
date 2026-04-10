1.  **Create Database Migration:**
    - Execute `cat << 'EOF' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql` containing:
      ```sql
      ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
      ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
      ```
    - Execute `sed -i 's/"migrations\/031_agent_missions_updated_at.sql",/"migrations\/031_agent_missions_updated_at.sql",\n        "migrations\/032_hybrid_sync_metadata.sql",/g' srcs/server/db/BUILD.bazel` to update the BUILD file.
    - Verify with `cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql` and `cat srcs/server/db/BUILD.bazel | grep -B 2 -A 2 032_hybrid_sync_metadata.sql`.

2.  **Define Go Interface and Structs:**
    - Create `srcs/server/hub/rag_sync.go` with `cat << 'EOF' > srcs/server/hub/rag_sync.go` containing:
      ```go
      package hub

      import (
          "context"
          "time"
          "go.opentelemetry.io/otel/metric"
          "go.opentelemetry.io/otel"
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
          Vector       []float32
          SyncStatus   SyncStatus
          LastSyncAt   time.Time
      }

      type RAGSyncService interface {
          FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
          MarkSynced(ctx context.Context, ids []string) error
          ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
      }

      var (
          meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
          RagRecordsSyncedTotal metric.Int64Counter
          RagSyncErrorsTotal    metric.Int64Counter
      )

      func InitRAGSyncMetrics() error {
          var err error
          RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
          if err != nil {
              return err
          }
          RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
          return err
      }
      ```
    - Verify with `cat srcs/server/hub/rag_sync.go`.

3.  **Implement RAGSyncService (Cloud and Standalone compatibility):**
    - Create `srcs/server/hub/rag_sync_impl.go` using `cat << 'EOF' > srcs/server/hub/rag_sync_impl.go` containing:
      ```go
      package hub

      import (
          "context"
          "encoding/binary"
          "fmt"
          "math"
          "time"
          "github.com/onehumancorp/mono/srcs/server/db"
          "database/sql"
          "strings"
      )

      type ragSyncServiceImpl struct {
          provider db.Provider
      }

      func NewRAGSyncService(provider db.Provider) RAGSyncService {
          return &ragSyncServiceImpl{provider: provider}
      }

      func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
          query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
          if s.provider.IsSQLite() {
              query = `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?`
          }

          rows, err := s.provider.Query(ctx, query, limit)
          if err != nil {
              return nil, err
          }
          defer rows.Close()

          var records []RAGSyncRecord
          for rows.Next() {
              var rec RAGSyncRecord
              var vecBytes []byte
              var lastSync sql.NullString
              if err := rows.Scan(&rec.ID, &rec.Context, &vecBytes, &rec.SyncStatus, &lastSync); err != nil {
                  return nil, err
              }

              if len(vecBytes) > 0 {
                  floats := make([]float32, len(vecBytes)/4)
                  for i := 0; i < len(floats); i++ {
                      bits := binary.LittleEndian.Uint32(vecBytes[i*4 : (i+1)*4])
                      floats[i] = math.Float32frombits(bits)
                  }
                  rec.Vector = floats
              }

              if lastSync.Valid {
                  t, err := time.Parse(time.RFC3339, lastSync.String)
                  if err == nil {
                      rec.LastSyncAt = t
                  }
              }

              records = append(records, rec)
          }
          return records, rows.Err()
      }

      func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
          if len(ids) == 0 {
              return nil
          }

          placeholders := make([]string, len(ids))
          args := make([]any, len(ids))
          for i, id := range ids {
              if s.provider.IsSQLite() {
                  placeholders[i] = "?"
              } else {
                  placeholders[i] = fmt.Sprintf("$%d", i+1)
              }
              args[i] = id
          }

          query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)`, strings.Join(placeholders, ","))
          _, err := s.provider.Exec(ctx, query, args...)
          if err != nil && RagSyncErrorsTotal != nil {
              RagSyncErrorsTotal.Add(ctx, 1)
          } else if err == nil && RagRecordsSyncedTotal != nil {
              RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
          }
          return err
      }

      func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
          for _, rec := range records {
              var vecBytes []byte
              if len(rec.Vector) > 0 {
                  vecBytes = make([]byte, len(rec.Vector)*4)
                  for i, f := range rec.Vector {
                      binary.LittleEndian.PutUint32(vecBytes[i*4:(i+1)*4], math.Float32bits(f))
                  }
              }

              query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                        VALUES ($1, $2, $3, $4, $5)
                        ON CONFLICT (memory_id) DO UPDATE
                        SET context = EXCLUDED.context,
                            vector_embedding = EXCLUDED.vector_embedding,
                            sync_status = EXCLUDED.sync_status,
                            last_sync_at = EXCLUDED.last_sync_at`

              if s.provider.IsSQLite() {
                  query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                           VALUES (?, ?, ?, ?, ?)
                           ON CONFLICT (memory_id) DO UPDATE
                           SET context = EXCLUDED.context,
                               vector_embedding = EXCLUDED.vector_embedding,
                               sync_status = EXCLUDED.sync_status,
                               last_sync_at = EXCLUDED.last_sync_at`
              }

              var lastSyncAtAny any
              if rec.LastSyncAt.IsZero() {
                  lastSyncAtAny = nil
              } else {
                  lastSyncAtAny = rec.LastSyncAt.Format(time.RFC3339)
              }

              _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, vecBytes, rec.SyncStatus, lastSyncAtAny)
              if err != nil {
                  if RagSyncErrorsTotal != nil {
                      RagSyncErrorsTotal.Add(ctx, 1)
                  }
                  return err
              }
          }

          if RagRecordsSyncedTotal != nil {
              RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
          }
          return nil
      }
      ```
    - Verify with `cat srcs/server/hub/rag_sync_impl.go`.

4.  **Write Tests:**
    - Create `srcs/server/hub/rag_sync_test.go` with `cat << 'EOF' > srcs/server/hub/rag_sync_test.go` containing:
      ```go
      package hub

      import (
          "context"
          "database/sql"
          "testing"
          "time"
          "github.com/onehumancorp/mono/srcs/server/db"
          _ "modernc.org/sqlite"
      )

      func setupTestDB(t *testing.T) db.Provider {
          d, err := sql.Open("sqlite", ":memory:")
          if err != nil {
              t.Fatalf("failed to open memory db: %v", err)
          }

          provider := db.NewSqliteProvider(d)

          createTable := `
          CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
              memory_id        TEXT PRIMARY KEY,
              context          TEXT NOT NULL,
              vector_embedding BYTEA,
              source_plugin    TEXT,
              created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
              sync_status      VARCHAR(50) DEFAULT 'pending',
              last_sync_at     TIMESTAMPTZ NULL
          );`

          _, err = provider.Exec(context.Background(), createTable)
          if err != nil {
              t.Fatalf("failed to create table: %v", err)
          }

          return provider
      }

      func TestRAGSyncService(t *testing.T) {
          InitRAGSyncMetrics()
          provider := setupTestDB(t)
          defer provider.Close()

          svc := NewRAGSyncService(provider)
          ctx := context.Background()

          // Test ProcessIncomingSync
          now := time.Now().UTC()
          rec1 := RAGSyncRecord{
              ID:         "mem-1",
              Context:    "test context 1",
              Vector:     []float32{1.1, 2.2},
              SyncStatus: SyncStatusPending,
              LastSyncAt: now,
          }

          err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{rec1})
          if err != nil {
              t.Fatalf("ProcessIncomingSync failed: %v", err)
          }

          // Test FetchPendingSyncs
          pending, err := svc.FetchPendingSyncs(ctx, 10)
          if err != nil {
              t.Fatalf("FetchPendingSyncs failed: %v", err)
          }
          if len(pending) != 1 {
              t.Fatalf("expected 1 pending record, got %d", len(pending))
          }
          if pending[0].ID != "mem-1" || pending[0].Vector[0] != 1.1 {
              t.Fatalf("unexpected record content: %+v", pending[0])
          }

          // Test MarkSynced
          err = svc.MarkSynced(ctx, []string{"mem-1"})
          if err != nil {
              t.Fatalf("MarkSynced failed: %v", err)
          }

          pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
          if err != nil {
              t.Fatalf("FetchPendingSyncs failed after mark: %v", err)
          }
          if len(pendingAfter) != 0 {
              t.Fatalf("expected 0 pending records after mark, got %d", len(pendingAfter))
          }
      }
      ```
    - Verify with `cat srcs/server/hub/rag_sync_test.go`.

5.  **Run Gazelle:**
    - Run `bazelisk run //:gazelle` to generate `BUILD.bazel` for `srcs/server/hub/`.
    - Verify with `git status` and `cat srcs/server/hub/BUILD.bazel`.

6.  **Run Tests:**
    - Run `bazelisk test //...` to ensure all tests pass.

7.  **Pre-commit Steps:**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8.  **Submit Changes:**
    - Execute `sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
    - Submit the code via `submit`.
