package autodream

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"

	import_sqlite "modernc.org/sqlite"
)

type mockLLM struct{
	failReason bool
	failEmbed bool
}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	if m.failReason {
		return "", errors.New("mock reason error")
	}
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.failEmbed {
		return nil, errors.New("mock embed error")
	}
	return []float32{0.1, 0.2, 0.3}, nil
}

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

	// Empty logs
	err = service.Consolidate(ctx, "task-123", []string{})
	if err != nil {
		t.Errorf("expected no error for empty logs, got %v", err)
	}

	// Reason failure
	llmFailReason := &mockLLM{failReason: true}
	serviceReasonFail := NewService(repo, llmFailReason)
	err = serviceReasonFail.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil {
		t.Errorf("expected error for reason failure, got nil")
	}

	// Embed failure
	llmFailEmbed := &mockLLM{failEmbed: true}
	serviceEmbedFail := NewService(repo, llmFailEmbed)
	err = serviceEmbedFail.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil {
		t.Errorf("expected error for embed failure, got nil")
	}

	// Missing claims
	err = service.Consolidate(context.Background(), "task-123", []string{"log 1"})
	if err == nil {
		t.Errorf("expected error for missing claims, got nil")
	}

	// Missing org ID
	ctxNoOrg := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})
	err = service.Consolidate(ctxNoOrg, "task-123", []string{"log 1"})
	if err == nil {
		t.Errorf("expected error for missing org ID, got nil")
	}
}

type failDBProvider struct {
	db.Provider
	failExecDelete bool
	failQuery bool
}

func (f *failDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if f.failExecDelete && strings.Contains(strings.ToUpper(sql), "DELETE") {
		return 0, errors.New("forced delete error")
	}
	if !f.failExecDelete && !f.failQuery {
		return 0, errors.New("forced exec error") // Default failure if neither specific is set
	}
	if !f.failExecDelete {
		return 0, errors.New("forced exec error")
	}
	return f.Provider.Exec(ctx, sql, arguments...)
}

func (f *failDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if f.failQuery {
		return nil, errors.New("forced query error")
	}
	return f.Provider.Query(ctx, sql, optionsAndArgs...)
}


func TestAutoDreamConsolidation_DBFail(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Make sure failProvider fails Exec generally
	failProvider := &failDBProvider{Provider: provider}
	repo := memory.NewVectorRepository(failProvider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err := service.Consolidate(ctx, "task-123", []string{"log 1"})
	if err == nil {
		t.Errorf("expected error for upsert failure, got nil")
	}
}

func TestResolveConflicts(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert mock data
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m2", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})

	err := service.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// DB Failure finding conflicts
	failProvider := &failDBProvider{Provider: provider, failQuery: true}
	failRepo := memory.NewVectorRepository(failProvider)
	serviceFailRepo := NewService(failRepo, llm)
	err = serviceFailRepo.ResolveConflicts(ctx, "test-tenant-123")
	if err == nil {
		t.Errorf("expected error when FindConflicts fails, got nil")
	}
}

func TestResolveConflicts_LLMAndDBFailures(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)

	// Reason failure
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m2", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})
	llmReasonFail := &mockLLM{failReason: true}
	serviceReasonFail := NewService(repo, llmReasonFail)
	err := serviceReasonFail.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error (swallowed), got %v", err)
	}

	// Embedding failure
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m3", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m4", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})
	llmEmbedFail := &mockLLM{failEmbed: true}
	serviceEmbedFail := NewService(repo, llmEmbedFail)
	err = serviceEmbedFail.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error (swallowed), got %v", err)
	}

	// DeleteMemories failure
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m5", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m6", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})
	failDeleteProvider := &failDBProvider{Provider: provider, failExecDelete: true}
	failDeleteRepo := memory.NewVectorRepository(failDeleteProvider)
	serviceFailDelete := NewService(failDeleteRepo, &mockLLM{})
	err = serviceFailDelete.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error (swallowed), got %v", err)
	}
}

func TestPruneStaleContext(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert mock data with old date
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1}, CreatedAt: time.Now().Add(-48 * time.Hour),
	})

	err := service.PruneStaleContext(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Verify it was deleted
	records, _ := repo.SemanticSearch(ctx, "test-tenant-123", []float32{0.1}, 10)
	if len(records) != 0 {
		t.Errorf("expected 0 records, got %d", len(records))
	}
}
