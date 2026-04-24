package autodream

import (
	"context"
	"testing"
	"database/sql"
	"time"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/memory"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

type mockProvider struct {
	db.Provider
	queryFunc func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
	execFunc func(ctx context.Context, sql string, arguments ...any) (int64, error)
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryFunc != nil {
		return m.queryFunc(ctx, sql, optionsAndArgs...)
	}
	return m.Provider.Query(ctx, sql, optionsAndArgs...)
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return m.Provider.Exec(ctx, sql, arguments...)
}

func setupTestDB(t *testing.T) (*db.SqliteProvider, context.Context) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		dbConn.Close()
	})

	provider := db.NewSqliteProvider(dbConn)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// In test, creating table
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories_master (
		id VARCHAR PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		memory_type TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding BLOB,
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		source_task_id VARCHAR
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider, ctx
}

// Mock rows
type mockRows struct {
	records []*memory.EmbeddingRecord
	index   int
}

func (m *mockRows) Next() bool {
	if m.index < len(m.records) {
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	rec := m.records[m.index]

	idPtr := dest[0].(*string)
	orgPtr := dest[1].(*string)
	typePtr := dest[2].(*string)
	contentPtr := dest[3].(*string)
	timePtr := dest[4].(*time.Time)
	taskPtr := dest[5].(*string)

	*idPtr = rec.ID
	*orgPtr = rec.OrganizationID
	*typePtr = rec.MemoryType
	*contentPtr = rec.Content
	*timePtr = rec.CreatedAt
	*taskPtr = rec.SourceTaskID

	m.index++
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }


func TestAutoDreamConsolidation(t *testing.T) {
	provider, ctx := setupTestDB(t)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err := service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestAutoDreamExtractAndStoreFacts(t *testing.T) {
	provider, ctx := setupTestDB(t)

	mockProv := &mockProvider{Provider: provider}

	// Mock the semantic search query to return a fake conflicting fact
	mockProv.queryFunc = func(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (db.Rows, error) {
		recs := []*memory.EmbeddingRecord{
			{
				ID: "conflict-fact-1",
				OrganizationID: "test-tenant-123",
				MemoryType: "PERMANENT_FACT",
				Content: "old fact",
				CreatedAt: time.Now(),
				SourceTaskID: "task-0",
			},
		}
		return &mockRows{records: recs}, nil
	}

	repo := memory.NewVectorRepository(mockProv)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err := service.ExtractAndStoreFacts(ctx, "task-124", "new fact")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestAutoDreamPruning(t *testing.T) {
	provider, ctx := setupTestDB(t)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert old memory
	oldRecord := &memory.EmbeddingRecord{
		ID:             "old-task",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Old content",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
		SourceTaskID:   "old-1",
	}
	if err := repo.Upsert(ctx, oldRecord); err != nil {
		t.Fatalf("failed to upsert old record: %v", err)
	}

	// Prune
	deleted, err := service.PruneStaleMemories(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error pruning, got %v", err)
	}
	if deleted != 1 {
		t.Errorf("expected 1 deleted record, got %d", deleted)
	}
}

func TestAutoDreamPruningConservative(t *testing.T) {
	provider, ctx := setupTestDB(t)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert old memory of different type (should NOT be pruned by PruneStaleMemories)
	oldRecord := &memory.EmbeddingRecord{
		ID:             "old-task-2",
		OrganizationID: "test-tenant-123",
		MemoryType:     "PERMANENT_FACT",
		Content:        "Important old fact",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
		SourceTaskID:   "old-2",
	}
	if err := repo.Upsert(ctx, oldRecord); err != nil {
		t.Fatalf("failed to upsert old record: %v", err)
	}

	// Prune
	deleted, err := service.PruneStaleMemories(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error pruning, got %v", err)
	}
	if deleted != 0 {
		t.Errorf("expected 0 deleted records (conservative pruning), got %d", deleted)
	}
}
