package memory

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"

	import_sqlite "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	_ = import_sqlite.RegisterDeterministicScalarFunction("vec_distance_cosine", 2, func(ctx *import_sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		return 0.01, nil
	})

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return provider
}

type mockPostgresDB struct {
	db.Provider
	execCount int
}

func (m *mockPostgresDB) IsSQLite() bool {
	return false
}

func (m *mockPostgresDB) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execCount++
	return 1, nil
}

func (m *mockPostgresDB) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, nil // We handle mock rows specifically in tests where needed
}

func TestVectorRepository_Upsert_SQLite(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewVectorRepository(provider)

	record := &EmbeddingRecord{
		ID:             "123",
		OrganizationID: "org-1",
		MemoryType:     "TEST",
		Content:        "test content",
		Embedding:      []float32{0.1, 0.2},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-1",
	}

	err := repo.Upsert(context.Background(), record)
	if err != nil {
		t.Errorf("expected no error on Upsert SQLite, got %v", err)
	}
}

func TestVectorRepository_Upsert_Postgres(t *testing.T) {
	mockDB := &mockPostgresDB{}
	repo := NewVectorRepository(mockDB)

	record := &EmbeddingRecord{
		ID:             "123",
		OrganizationID: "org-1",
		MemoryType:     "TEST",
		Content:        "test content",
		Embedding:      []float32{0.1, 0.2},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-1",
	}

	err := repo.Upsert(context.Background(), record)
	if err != nil {
		t.Errorf("expected no error on Upsert Postgres, got %v", err)
	}
	if mockDB.execCount != 1 {
		t.Errorf("expected Exec to be called once")
	}
}

type mockRow struct {
	count int
	failErr bool
}

func (r *mockRow) Next() bool {
	if r.count == 0 {
		r.count++
		return true
	}
	return false
}

func (r *mockRow) Scan(dest ...any) error {
	if len(dest) == 5 {
		if id, ok := dest[0].(*string); ok { *id = "123" }
		if org, ok := dest[1].(*string); ok { *org = "org-1" }
		if src, ok := dest[2].(*string); ok { *src = "TEST" }
		if content, ok := dest[3].(*string); ok { *content = "content" }
		if ts, ok := dest[4].(*time.Time); ok { *ts = time.Now() }
	}
	return nil
}

func (r *mockRow) Close() {}
func (r *mockRow) Columns() ([]string, error) { return nil, nil }
func (r *mockRow) Err() error {
	if r.failErr {
		return errors.New("rows error")
	}
	return nil
}

type mockPostgresDBQuery struct {
	mockPostgresDB
	failQuery bool
	failScan bool
	failRowsErr bool
}

func (m *mockPostgresDBQuery) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.failQuery {
		return nil, errors.New("query error")
	}
	if m.failScan {
		return &mockRowScanFail{}, nil
	}
	if m.failRowsErr {
		return &mockRow{failErr: true, count: 1}, nil // count: 1 skips the Next loop
	}
	return &mockRow{}, nil
}

type mockRowScanFail struct {
	count int
}
func (r *mockRowScanFail) Next() bool {
	if r.count == 0 {
		r.count++
		return true
	}
	return false
}
func (r *mockRowScanFail) Scan(dest ...any) error { return errors.New("scan error") }
func (r *mockRowScanFail) Close() {}
func (r *mockRowScanFail) Columns() ([]string, error) { return nil, nil }
func (r *mockRowScanFail) Err() error { return nil }

func TestVectorRepository_SemanticSearch_SQLite(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewVectorRepository(provider)
	ctx := context.Background()

	// Insert record first
	repo.Upsert(ctx, &EmbeddingRecord{
		ID: "search1", OrganizationID: "org-1", Content: "test", Embedding: []float32{0.1},
	})

	records, err := repo.SemanticSearch(ctx, "org-1", []float32{0.1}, 10)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
}

func TestVectorRepository_SemanticSearch_Postgres(t *testing.T) {
	mockDB := &mockPostgresDBQuery{}
	repo := NewVectorRepository(mockDB)

	records, err := repo.SemanticSearch(context.Background(), "org-1", []float32{0.1}, 10)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record from mock row, got %d", len(records))
	}

	mockDBFail := &mockPostgresDBQuery{failQuery: true}
	repoFail := NewVectorRepository(mockDBFail)
	_, err = repoFail.SemanticSearch(context.Background(), "org-1", []float32{0.1}, 10)
	if err == nil {
		t.Errorf("expected error from failing query")
	}

	mockDBScanFail := &mockPostgresDBQuery{failScan: true}
	repoScanFail := NewVectorRepository(mockDBScanFail)
	_, err = repoScanFail.SemanticSearch(context.Background(), "org-1", []float32{0.1}, 10)
	if err == nil {
		t.Errorf("expected error from failing scan")
	}

	mockDBRowsErr := &mockPostgresDBQuery{failRowsErr: true}
	repoRowsErr := NewVectorRepository(mockDBRowsErr)
	_, err = repoRowsErr.SemanticSearch(context.Background(), "org-1", []float32{0.1}, 10)
	if err == nil {
		t.Errorf("expected error from failing rows.Err()")
	}
}

type mockConflictRow struct {
	count int
}
func (r *mockConflictRow) Next() bool {
	if r.count == 0 {
		r.count++
		return true
	}
	return false
}
func (r *mockConflictRow) Scan(dest ...any) error {
	if len(dest) == 4 {
		if id1, ok := dest[0].(*string); ok { *id1 = "1" }
		if id2, ok := dest[1].(*string); ok { *id2 = "2" }
		if c1, ok := dest[2].(*string); ok { *c1 = "content1" }
		if c2, ok := dest[3].(*string); ok { *c2 = "content2" }
	}
	return nil
}
func (r *mockConflictRow) Close() {}
func (r *mockConflictRow) Columns() ([]string, error) { return nil, nil }
func (r *mockConflictRow) Err() error { return nil }

type mockPostgresDBConflict struct {
	mockPostgresDB
	failQuery bool
	failScan bool
}
func (m *mockPostgresDBConflict) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.failQuery {
		return nil, errors.New("query error")
	}
	if m.failScan {
		return &mockRowScanFail{}, nil
	}
	return &mockConflictRow{}, nil
}

func TestVectorRepository_FindConflicts_SQLite(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewVectorRepository(provider)
	ctx := context.Background()

	// Insert record first
	repo.Upsert(ctx, &EmbeddingRecord{ID: "1", OrganizationID: "org-1", Content: "A", Embedding: []float32{0.1}})
	repo.Upsert(ctx, &EmbeddingRecord{ID: "2", OrganizationID: "org-1", Content: "B", Embedding: []float32{0.1}})

	conflicts, err := repo.FindConflicts(ctx, "org-1", 0.5)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(conflicts) != 1 {
		t.Errorf("expected 1 conflict, got %d", len(conflicts))
	}
}

func TestVectorRepository_FindConflicts_Postgres(t *testing.T) {
	mockDB := &mockPostgresDBConflict{}
	repo := NewVectorRepository(mockDB)

	conflicts, err := repo.FindConflicts(context.Background(), "org-1", 0.5)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(conflicts) != 1 {
		t.Errorf("expected 1 conflict from mock row, got %d", len(conflicts))
	}

	mockDBFail := &mockPostgresDBConflict{failQuery: true}
	repoFail := NewVectorRepository(mockDBFail)
	_, err = repoFail.FindConflicts(context.Background(), "org-1", 0.5)
	if err == nil {
		t.Errorf("expected error from failing query")
	}

	mockDBScanFail := &mockPostgresDBConflict{failScan: true}
	repoScanFail := NewVectorRepository(mockDBScanFail)
	_, err = repoScanFail.FindConflicts(context.Background(), "org-1", 0.5)
	if err == nil {
		t.Errorf("expected error from failing scan")
	}
}

type failDBProviderExec struct {
	db.Provider
}

func (f *failDBProviderExec) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, errors.New("forced exec error")
}

func TestVectorRepository_DeleteMemories(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewVectorRepository(provider)
	ctx := context.Background()

	err := repo.DeleteMemories(ctx, []string{})
	if err != nil {
		t.Errorf("expected no error for empty slice, got %v", err)
	}

	repo.Upsert(ctx, &EmbeddingRecord{ID: "del1", OrganizationID: "org-1", Content: "A", Embedding: []float32{0.1}})

	err = repo.DeleteMemories(ctx, []string{"del1"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	failProv := &failDBProviderExec{Provider: provider}
	failRepo := NewVectorRepository(failProv)
	err = failRepo.DeleteMemories(ctx, []string{"del1"})
	if err == nil {
		t.Errorf("expected error from fail exec")
	}
}

func TestVectorRepository_PruneOlderThan(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewVectorRepository(provider)
	ctx := context.Background()

	err := repo.PruneOlderThan(ctx, "org-1", time.Now())
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}
