1. **Fix Metrics Drift & Compatibility:** Update `srcs/server/hub/rag_sync_impl.go` using a Python script to fix metrics updating inside the loop and `[]byte` PostgreSQL incompatibility.
   ```bash
   cat << 'EOF' > patch.py
   with open('srcs/server/hub/rag_sync_impl.go', 'r') as f:
       content = f.read()

   old_mark = """
       query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
       for _, id := range ids {
           if _, err := tx.ExecContext(ctx, query, id); err != nil {
               RagSyncErrorsTotal.Add(ctx, 1)
               return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
           }
           RagRecordsSyncedTotal.Add(ctx, 1)
       }

       return tx.Commit()
   """

   new_mark = """
       query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
       var successCount int64
       for _, id := range ids {
           if _, err := tx.ExecContext(ctx, query, id); err != nil {
               RagSyncErrorsTotal.Add(ctx, 1)
               return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
           }
           successCount++
       }

       if err := tx.Commit(); err != nil {
           RagSyncErrorsTotal.Add(ctx, 1)
           return err
       }
       RagRecordsSyncedTotal.Add(ctx, successCount)
       return nil
   """
   content = content.replace(old_mark.strip(), new_mark.strip())

   old_upsert = """
       for _, r := range records {
           vectorJSON, _ := json.Marshal(r.Vector)
           if _, err := tx.ExecContext(ctx, query, r.ID, r.Context, vectorJSON, r.SyncStatus); err != nil {
               return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
           }
       }
   """

   new_upsert = """
       for _, r := range records {
           vectorJSON, _ := json.Marshal(r.Vector)
           if _, err := tx.ExecContext(ctx, query, r.ID, r.Context, string(vectorJSON), r.SyncStatus); err != nil {
               return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
           }
       }
   """
   content = content.replace(old_upsert.strip(), new_upsert.strip())

   with open('srcs/server/hub/rag_sync_impl.go', 'w') as f:
       f.write(content)
   EOF
   python3 patch.py
   rm patch.py
   gofmt -w srcs/server/hub/rag_sync_impl.go
   cat srcs/server/hub/rag_sync_impl.go
   ```
2. **Implement Full Test Coverage:** Expand `srcs/server/hub/rag_sync_impl_test.go` to achieve >90% test coverage.
   ```bash
   cat << 'EOF' > srcs/server/hub/rag_sync_impl_test.go
   package hub

   import (
       "context"
       "database/sql"
       "encoding/json"
       "testing"
       "time"
   )

   // mockDBProvider implements DBProvider for testing
   type mockDBProvider struct {
       queryErr error
       execErr  error
       txErr    error
       rows     *mockRows
   }

   type mockTx struct {
       execErr   error
       commitErr error
   }

   func (t *mockTx) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
       return nil, t.execErr
   }
   func (t *mockTx) Commit() error {
       return t.commitErr
   }
   func (t *mockTx) Rollback() error {
       return nil
   }

   // mockRows implements database/sql.Rows interface methods needed
   type mockRows struct {
       records []RAGSyncRecord
       index   int
       err     error
   }

   func (m *mockRows) Next() bool {
       if m.index < len(m.records) {
           return true
       }
       return false
   }

   func (m *mockRows) Scan(dest ...any) error {
       r := m.records[m.index]
       *dest[0].(*string) = r.ID
       *dest[1].(*string) = r.Context

       vectorJSON, _ := json.Marshal(r.Vector)
       if dest[2] != nil {
           b := dest[2].(*[]byte)
           *b = vectorJSON
       }

       *dest[3].(*SyncStatus) = r.SyncStatus

       ns := dest[4].(*sql.NullTime)
       ns.Valid = true
       ns.Time = r.LastSyncAt

       m.index++
       return nil
   }

   func (m *mockRows) Close() error {
       return nil
   }

   func (m *mockRows) Columns() ([]string, error) {
       return []string{"memory_id", "context", "vector_embedding", "sync_status", "last_sync_at"}, nil
   }

   func (m *mockRows) Err() error {
       return m.err
   }

   // We need to implement sql.Tx for our mock to work cleanly or change the interface
   // To keep it simple, we will adapt DBProvider to return an interface that mockTx implements

   // Since the DBProvider returns *sql.Tx directly, mocking it perfectly is hard without wrapping the standard library.
   // Given time constraints and strict coverage rules, we'll rewrite rag_sync_impl_test.go with a wrapper pattern if we had one.
   // Instead we'll use a pragmatic approach for unit testing database-dependent code in Go.
   EOF
   cat << 'EOF' > patch_test.go
   package hub
   // We will implement a simplified mock strategy in patch_test.go and then merge.
   // Given time constraints, I will instead provide a full rewrite of rag_sync_impl.go and rag_sync_impl_test.go
   // to use interfaces for sql.Rows and sql.Tx
   EOF
   ```
   *Actually, I need to modify `rag_sync_impl.go` to use an interface for `Tx` and `Rows` to achieve high coverage.*
   ```bash
   cat << 'EOF' > srcs/server/hub/rag_sync_impl.go
   package hub

   import (
       "context"
       "database/sql"
       "encoding/json"
       "fmt"
       "time"
   )

   type Rows interface {
       Next() bool
       Scan(dest ...any) error
       Close() error
       Err() error
       Columns() ([]string, error)
   }

   type Tx interface {
       ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
       Commit() error
       Rollback() error
   }

   type DBProvider interface {
       QueryContext(ctx context.Context, query string, args ...any) (Rows, error)
       ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
       BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error)
   }

   type ragSyncService struct {
       db DBProvider
   }

   func NewRAGSyncService(db DBProvider) RAGSyncService {
       return &ragSyncService{db: db}
   }

   func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
       query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
                 FROM swarm_memory_embeddings
                 WHERE sync_status = 'pending'
                 LIMIT $1`

       rows, err := s.db.QueryContext(ctx, query, limit)
       if err != nil {
           return nil, fmt.Errorf("failed to query pending syncs: %w", err)
       }
       defer rows.Close()

       var records []RAGSyncRecord
       for rows.Next() {
           var r RAGSyncRecord
           var vectorJSON []byte
           var lastSyncAt sql.NullTime

           if err := rows.Scan(&r.ID, &r.Context, &vectorJSON, &r.SyncStatus, &lastSyncAt); err != nil {
               return nil, fmt.Errorf("failed to scan record: %w", err)
           }

           if vectorJSON != nil {
               if err := json.Unmarshal(vectorJSON, &r.Vector); err != nil {
                   // Ignore error to keep processing other rows
               }
           }
           if lastSyncAt.Valid {
               r.LastSyncAt = lastSyncAt.Time
           }

           records = append(records, r)
       }
       if err := rows.Err(); err != nil {
           return nil, fmt.Errorf("rows error: %w", err)
       }
       return records, nil
   }

   func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
       if len(ids) == 0 {
           return nil
       }

       tx, err := s.db.BeginTx(ctx, nil)
       if err != nil {
           return err
       }
       defer tx.Rollback()

       query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
       var successCount int64
       for _, id := range ids {
           if _, err := tx.ExecContext(ctx, query, id); err != nil {
               RagSyncErrorsTotal.Add(ctx, 1)
               return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
           }
           successCount++
       }

       if err := tx.Commit(); err != nil {
           RagSyncErrorsTotal.Add(ctx, 1)
           return err
       }
       RagRecordsSyncedTotal.Add(ctx, successCount)
       return nil
   }

   func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
       if len(records) == 0 {
           return nil
       }

       tx, err := s.db.BeginTx(ctx, nil)
       if err != nil {
           return err
       }
       defer tx.Rollback()

       query := `
           INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
           VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
           ON CONFLICT(memory_id) DO UPDATE SET
               context = excluded.context,
               vector_embedding = excluded.vector_embedding,
               sync_status = excluded.sync_status,
               last_sync_at = excluded.last_sync_at
       `

       for _, r := range records {
           vectorJSON, _ := json.Marshal(r.Vector)
           if _, err := tx.ExecContext(ctx, query, r.ID, r.Context, string(vectorJSON), r.SyncStatus); err != nil {
               return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
           }
       }

       return tx.Commit()
   }
   EOF
   gofmt -w srcs/server/hub/rag_sync_impl.go
   cat srcs/server/hub/rag_sync_impl.go

   cat << 'EOF' > srcs/server/hub/rag_sync_impl_test.go
   package hub

   import (
       "context"
       "database/sql"
       "encoding/json"
       "errors"
       "testing"
       "time"
   )

   type mockRows struct {
       records []RAGSyncRecord
       index   int
       err     error
   }

   func (m *mockRows) Next() bool {
       if m.index < len(m.records) {
           return true
       }
       return false
   }

   func (m *mockRows) Scan(dest ...any) error {
       r := m.records[m.index]
       *dest[0].(*string) = r.ID
       *dest[1].(*string) = r.Context

       vectorJSON, _ := json.Marshal(r.Vector)
       b := dest[2].(*[]byte)
       *b = vectorJSON

       *dest[3].(*SyncStatus) = r.SyncStatus

       ns := dest[4].(*sql.NullTime)
       ns.Valid = true
       ns.Time = r.LastSyncAt

       m.index++
       return nil
   }

   func (m *mockRows) Close() error { return nil }
   func (m *mockRows) Columns() ([]string, error) { return []string{"memory_id", "context", "vector_embedding", "sync_status", "last_sync_at"}, nil }
   func (m *mockRows) Err() error { return m.err }

   type mockTx struct {
       execErr   error
       commitErr error
   }

   func (t *mockTx) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
       return nil, t.execErr
   }
   func (t *mockTx) Commit() error {
       return t.commitErr
   }
   func (t *mockTx) Rollback() error {
       return nil
   }

   type mockDBProvider struct {
       queryErr error
       execErr  error
       txErr    error
       rows     Rows
       tx       Tx
   }

   func (m *mockDBProvider) QueryContext(ctx context.Context, query string, args ...any) (Rows, error) {
       if m.queryErr != nil {
           return nil, m.queryErr
       }
       return m.rows, nil
   }

   func (m *mockDBProvider) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
       return nil, m.execErr
   }

   func (m *mockDBProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error) {
       if m.txErr != nil {
           return nil, m.txErr
       }
       return m.tx, nil
   }

   func TestFetchPendingSyncsSuccess(t *testing.T) {
       rows := &mockRows{
           records: []RAGSyncRecord{
               {ID: "1", Context: "ctx", Vector: []float32{1.0}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
           },
       }
       db := &mockDBProvider{rows: rows}
       svc := NewRAGSyncService(db)

       records, err := svc.FetchPendingSyncs(context.Background(), 1)
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       if len(records) != 1 {
           t.Fatalf("expected 1 record, got %d", len(records))
       }
       if records[0].ID != "1" {
           t.Errorf("expected ID 1, got %s", records[0].ID)
       }
   }

   func TestFetchPendingSyncsQueryError(t *testing.T) {
       db := &mockDBProvider{queryErr: errors.New("query error")}
       svc := NewRAGSyncService(db)

       _, err := svc.FetchPendingSyncs(context.Background(), 10)
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestFetchPendingSyncsRowsError(t *testing.T) {
       rows := &mockRows{
           records: []RAGSyncRecord{{ID: "1"}},
           err:     errors.New("rows error"),
       }
       db := &mockDBProvider{rows: rows}
       svc := NewRAGSyncService(db)

       _, err := svc.FetchPendingSyncs(context.Background(), 10)
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestMarkSyncedSuccess(t *testing.T) {
       tx := &mockTx{}
       db := &mockDBProvider{tx: tx}
       svc := NewRAGSyncService(db)

       err := svc.MarkSynced(context.Background(), []string{"1", "2"})
       if err != nil {
           t.Errorf("unexpected error: %v", err)
       }
   }

   func TestMarkSyncedEmpty(t *testing.T) {
       db := &mockDBProvider{}
       svc := NewRAGSyncService(db)

       err := svc.MarkSynced(context.Background(), []string{})
       if err != nil {
           t.Errorf("unexpected error: %v", err)
       }
   }

   func TestMarkSyncedTxError(t *testing.T) {
       db := &mockDBProvider{txErr: errors.New("tx error")}
       svc := NewRAGSyncService(db)

       err := svc.MarkSynced(context.Background(), []string{"1"})
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestMarkSyncedExecError(t *testing.T) {
       tx := &mockTx{execErr: errors.New("exec error")}
       db := &mockDBProvider{tx: tx}
       svc := NewRAGSyncService(db)

       err := svc.MarkSynced(context.Background(), []string{"1"})
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestMarkSyncedCommitError(t *testing.T) {
       tx := &mockTx{commitErr: errors.New("commit error")}
       db := &mockDBProvider{tx: tx}
       svc := NewRAGSyncService(db)

       err := svc.MarkSynced(context.Background(), []string{"1"})
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestProcessIncomingSyncSuccess(t *testing.T) {
       tx := &mockTx{}
       db := &mockDBProvider{tx: tx}
       svc := NewRAGSyncService(db)

       err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
           {ID: "1", Vector: []float32{1.0}},
       })
       if err != nil {
           t.Errorf("unexpected error: %v", err)
       }
   }

   func TestProcessIncomingSyncEmpty(t *testing.T) {
       db := &mockDBProvider{}
       svc := NewRAGSyncService(db)

       err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
       if err != nil {
           t.Errorf("unexpected error: %v", err)
       }
   }

   func TestProcessIncomingSyncTxError(t *testing.T) {
       db := &mockDBProvider{txErr: errors.New("tx error")}
       svc := NewRAGSyncService(db)

       err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
       if err == nil {
           t.Error("expected error, got nil")
       }
   }

   func TestProcessIncomingSyncExecError(t *testing.T) {
       tx := &mockTx{execErr: errors.New("exec error")}
       db := &mockDBProvider{tx: tx}
       svc := NewRAGSyncService(db)

       err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
       if err == nil {
           t.Error("expected error, got nil")
       }
   }
   EOF
   gofmt -w srcs/server/hub/rag_sync_impl_test.go
   cat srcs/server/hub/rag_sync_impl_test.go
   ```
3. **Run Tests with Coverage:** Run `~/go/bin/bazelisk test //srcs/server/hub/... //srcs/server/db/... --test_output=errors --jobs=4 --local_test_jobs=1` to ensure tests pass and code is correctly formatted.
4. **Pre-commit Steps:** Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
5. **Finalize:** Submit changes with a PR named `💰 Miser: [new cost feature]`.
