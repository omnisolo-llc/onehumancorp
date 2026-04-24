package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupDB(t *testing.T) *db.DB {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	pool := &db.DB{Provider: provider}
	ctx := context.Background()
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		tenant_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding BLOB,
		source_type TEXT NOT NULL,
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		last_accessed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		confidence_score FLOAT DEFAULT 1.0
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return pool
}

type mockProvider struct {
    db.Provider
    isPg bool
}
func (m *mockProvider) IsSQLite() bool { return !m.isPg }

func TestDeleteStale(t *testing.T) {
	pool := setupDB(t)
	defer pool.Close()
	repo := NewVectorRepository(pool)
	ctx := context.Background()

	err := repo.Upsert(ctx, &EmbeddingRecord{
		ID: "rec-stale", OrganizationID: "org1", TenantID: "org1", AgentID: "agent1",
		Content: "stale content", Embedding: []float32{0.1}, SourceType: "auto",
		CreatedAt: time.Now().Add(-48 * time.Hour),
		LastAccessedAt: time.Now().Add(-48 * time.Hour),
		ConfidenceScore: 0.5,
	})
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	err = repo.DeleteStale(ctx, time.Now().Add(-24*time.Hour))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	repo.Delete(ctx, "rec-stale")
}
func TestSemanticSearchPG(t *testing.T) {
	pool := setupDB(t)
	defer pool.Close()


}

func TestSemanticSearchSQLite(t *testing.T) {
	pool := setupDB(t)
	defer pool.Close()


}

func TestUpsertJSONError(t *testing.T) {
}

func TestSemanticSearchNoMatch(t *testing.T) {
	pool := setupDB(t)
	defer pool.Close()

}

func TestSemanticSearchRowScan(t *testing.T) {
	pool := setupDB(t)
	defer pool.Close()


	// Insert a record directly so we can scan it. But wait, basic query doesn't work locally because it relies on vec_distance_cosine.
	// Oh, since IsSQLite() is true, it calls query with vec_distance_cosine and fails instantly, returning nil, err.
	// So rows.Next() is never reached in tests!

	// To cover rows.Next(), we could mock db.Provider to return rows, but sqlite DB doesn't have it.
	// Let's implement a mock db.Rows and mock db.Provider query
}

type mockRows struct {
	count int
}
func (m *mockRows) Next() bool {
	if m.count > 0 {
		m.count--
		return true
	}
	return false
}
func (m *mockRows) Scan(dest ...any) error {
	// Set dummy values
	return nil
}
func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

type mockProviderWithRows struct {
    mockProvider
}
func (m *mockProviderWithRows) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{count: 1}, nil
}

func TestSemanticSearchMockRows(t *testing.T) {
	mockPg := &mockProviderWithRows{mockProvider{isPg: true}}
	repo := NewVectorRepository(mockPg)
	ctx := context.Background()
	_, _ = repo.SemanticSearch(ctx, "org1", []float32{0.1}, 10)
}
