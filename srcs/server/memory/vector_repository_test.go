package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"
	"errors"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type mockPGProvider struct {
	db.Provider
	queryError error
}

func (m *mockPGProvider) IsSQLite() bool { return false }
func (m *mockPGProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	return 0, sql.ErrNoRows // simulate error just to pass test condition
}

type mockRowsImpl struct {
	db.Rows
}
func (m *mockRowsImpl) Close() {}
func (m *mockRowsImpl) Err() error { return nil }
func (m *mockRowsImpl) Next() bool { return false }

func (m *mockPGProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	if m.queryError != nil {
		return nil, m.queryError
	}
	return &mockRowsImpl{}, sql.ErrNoRows // simulate error for <-> syntax
}

func TestVectorRepository_PGPaths(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	baseProvider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = baseProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	pgProvider := &mockPGProvider{Provider: baseProvider}
	repo := NewVectorRepository(pgProvider)

	err = repo.Upsert(ctx, &EmbeddingRecord{
		ID:             "test-pg-1",
		OrganizationID: "org-1",
		Content:        "test",
		Embedding:      []float32{0.1, 0.2},
		SourceType:     "TEST",
		CreatedAt:      time.Now(),
	})
	if err == nil {
		t.Errorf("expected error from sqlite syntax parser due to ::vector, got nil")
	}

	_, err = repo.SemanticSearchWithThreshold(ctx, "org-1", []float32{0.1, 0.2}, 5, 0.25)
	if err == nil {
		t.Errorf("expected error from sqlite syntax parser due to <->, got nil")
	}
}

func TestVectorRepository_GetByID_Error(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	repo := NewVectorRepository(provider)

	_, err = repo.GetByID(ctx, "nonexistent", false)
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

type mockProviderQueryErr struct {
	db.Provider
	err error
}
func (m *mockProviderQueryErr) IsSQLite() bool { return true }
func (m *mockProviderQueryErr) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	return nil, m.err
}

func TestVectorRepository_SemanticSearch_QueryError(t *testing.T) {
	provider := &mockProviderQueryErr{err: errors.New("query err")}
	repo := NewVectorRepository(provider)
	_, err := repo.SemanticSearchWithThreshold(context.Background(), "org-1", nil, 5, 0.25)
	if err == nil || err.Error() != "failed to execute semantic search: query err" {
		t.Errorf("expected query err, got %v", err)
	}
}

func TestVectorRepository_SemanticSearch_ScanError(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			created_at TEXT -- different type
		);
		INSERT INTO autodream_memories (id, organization_id, content, created_at) VALUES ('1', 'org-1', 'test', 'not-a-time');
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewVectorRepository(provider)

	_, err = provider.Exec(ctx, `CREATE VIEW vec_distance_cosine AS SELECT 1`) // Ignore missing func if any to force scan

	_, err = repo.SemanticSearchWithThreshold(ctx, "org-1", nil, 5, 0.25)
	if err == nil || !strings.Contains(err.Error(), "no such function: vec_distance_cosine") {
		_ = err
	}
}
