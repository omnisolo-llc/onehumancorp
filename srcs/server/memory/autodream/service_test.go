package autodream

import (
	"context"
	"database/sql"
	"testing"
	"time"
	"strings"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/memory"
	_ "modernc.org/sqlite"
)

type mockLLM struct{
	failReason bool
	failEmbed bool
}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	if m.failReason {
		return "", errors.New("mock reason error")
	}
	if strings.Contains(prompt, "Analyze these potentially conflicting memories") {
		return "Resolved Mock Fact", nil
	}
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.failEmbed {
		return nil, errors.New("mock embed error")
	}
	return []float32{0.1, 0.2, 0.3}, nil
}

// Since sqlite-vec isn't loaded in test, we need a mock provider that intercepts SemanticSearch.
type mockVectorDB struct {
	db.Provider
}

type mockVectorRows struct {
	db.Rows
	records []*memory.EmbeddingRecord
	idx     int
}
func (m *mockVectorRows) Close() {}
func (m *mockVectorRows) Err() error { return nil }
func (m *mockVectorRows) Next() bool {
	if m.idx < len(m.records) {
		return true
	}
	return false
}
func (m *mockVectorRows) Scan(dest ...interface{}) error {
	rec := m.records[m.idx]
	m.idx++
	// map fields
	*dest[0].(*string) = rec.ID
	*dest[1].(*string) = rec.OrganizationID
	taskNull := dest[2].(*sql.NullString)
	if rec.TaskID != "" {
		taskNull.String = rec.TaskID
		taskNull.Valid = true
	} else {
		taskNull.Valid = false
	}
	*dest[3].(*string) = rec.Content
	*dest[4].(*string) = rec.SourceType
	*dest[5].(*time.Time) = rec.CreatedAt
	return nil
}

func (m *mockVectorDB) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	if strings.Contains(query, "vec_distance_cosine") {
		// Mock semantic search response
		orgID := args[0].(string)
		if orgID == "org-1" {
			records := []*memory.EmbeddingRecord{
				{ID: "mem-1", OrganizationID: "org-1", TaskID: "task-1", Content: "fact A is true", SourceType: "TEST", CreatedAt: time.Now()},
				{ID: "mem-2", OrganizationID: "org-1", TaskID: "task-2", Content: "fact A is false", SourceType: "TEST", CreatedAt: time.Now()},
			}
			return &mockVectorRows{records: records, idx: 0}, nil
		}
		return &mockVectorRows{records: nil}, nil
	}
	return m.Provider.Query(ctx, query, args...)
}

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
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

	return &mockVectorDB{Provider: provider}
}

func TestAutoDreamConsolidation(t *testing.T) {
	provider := setupTestDB(t)

	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err := service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Verify it was inserted
	rec, err := repo.GetByID(ctx, "task-123-summary", false)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if rec.Content != "Mock summary" {
		t.Errorf("expected Mock summary, got %v", rec.Content)
	}

	// Test no logs
	err = service.Consolidate(ctx, "task-123", []string{})
	if err != nil {
		t.Errorf("expected no error for empty logs, got %v", err)
	}

	// Test reason error
	service.llm = &mockLLM{failReason: true}
	err = service.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil || !strings.Contains(err.Error(), "mock reason error") {
		t.Errorf("expected mock reason error, got %v", err)
	}

	// Test embed error
	service.llm = &mockLLM{failEmbed: true}
	err = service.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil || !strings.Contains(err.Error(), "mock embed error") {
		t.Errorf("expected mock embed error, got %v", err)
	}

	// Test unauthorized
	service.llm = &mockLLM{}
	ctxNoClaims := context.Background()
	err = service.Consolidate(ctxNoClaims, "task-123", []string{"log 1"})
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	// Test upsert error (simulate by dropping table)
	provider.Exec(ctx, "DROP TABLE autodream_memories")
	err = service.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil || !strings.Contains(err.Error(), "failed to upsert") {
		t.Errorf("expected upsert error, got %v", err)
	}
}

func TestAutoDreamPruneStaleMemories(t *testing.T) {
	provider := setupTestDB(t)
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)
	ctx := context.Background()

	// Insert an old memory
	oldTime := time.Now().Add(-48 * time.Hour)
	err := repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "old-memory",
		OrganizationID: "org-1",
		TaskID:         "task-1",
		Content:        "old content",
		Embedding:      []float32{0.1},
		SourceType:     "TEST",
		CreatedAt:      oldTime,
	})
	if err != nil {
		t.Fatalf("failed to insert old memory: %v", err)
	}

	// Insert a new memory
	newTime := time.Now()
	err = repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "new-memory",
		OrganizationID: "org-1",
		TaskID:         "task-2",
		Content:        "new content",
		Embedding:      []float32{0.1},
		SourceType:     "TEST",
		CreatedAt:      newTime,
	})
	if err != nil {
		t.Fatalf("failed to insert new memory: %v", err)
	}

	err = service.PruneStaleMemories(ctx, 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	_, err = repo.GetByID(ctx, "old-memory", false)
	if err == nil {
		t.Errorf("expected old memory to be deleted")
	}

	_, err = repo.GetByID(ctx, "new-memory", false)
	if err != nil {
		t.Errorf("expected new memory to still exist")
	}
}

func TestAutoDreamResolveConflicts(t *testing.T) {
	provider := setupTestDB(t)
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)
	ctx := context.Background()

	// Insert a memory that will be the target
	err := repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "mem-1",
		OrganizationID: "org-1",
		TaskID:         "task-1",
		Content:        "fact A is true",
		Embedding:      []float32{0.1, 0.2, 0.3},
		SourceType:     "TEST",
		CreatedAt:      time.Now(),
	})
	if err != nil {
		t.Fatalf("failed to insert memory: %v", err)
	}

	// Insert a conflicting memory
	err = repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "mem-2",
		OrganizationID: "org-1",
		TaskID:         "task-2",
		Content:        "fact A is false",
		Embedding:      []float32{0.1, 0.2, 0.3}, // Same embedding for test
		SourceType:     "TEST",
		CreatedAt:      time.Now(),
	})
	if err != nil {
		t.Fatalf("failed to insert memory: %v", err)
	}

	err = service.ResolveConflicts(ctx, "mem-1", "org-1")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// mem-1 and mem-2 should be deleted
	_, err = repo.GetByID(ctx, "mem-1", false)
	if err == nil {
		t.Errorf("expected mem-1 to be deleted")
	}
	_, err = repo.GetByID(ctx, "mem-2", false)
	if err == nil {
		t.Errorf("expected mem-2 to be deleted")
	}
}

func TestAutoDreamResolveConflictsErrors(t *testing.T) {
	provider := setupTestDB(t)
	repo := memory.NewVectorRepository(provider)
	ctx := context.Background()
	service := NewService(repo, &mockLLM{})

	// Test target memory missing
	err := service.ResolveConflicts(ctx, "nonexistent", "org-1")
	if err == nil {
		t.Errorf("expected error for missing target")
	}
}

type customLLMWithFail struct{}
func (m *customLLMWithFail) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock", nil
}
func (m *customLLMWithFail) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if text == "Mock" {
		return nil, errors.New("fail on resolved")
	}
	return []float32{0.1, 0.2, 0.3}, nil
}
