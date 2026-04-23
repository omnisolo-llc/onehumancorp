package autodream

import (
	"context"
	"testing"
	"strings"
	"time"
	"database/sql"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/memory"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	if strings.Contains(prompt, "identify if there are any genuine conflicts") {
		return `{"resolved_fact": "Maya's cake price is $55", "conflicting_ids": ["fact-1", "fact-2"]}`, nil
	}
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupTestDB(t *testing.T) (*db.SqliteProvider, *memory.VectorRepository, *Service, context.Context) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

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

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	return provider, repo, service, ctx
}

func TestAutoDreamConsolidation(t *testing.T) {
	provider, _, service, ctx := setupTestDB(t)
	defer provider.Close()

	err := service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestResolveConflicts(t *testing.T) {
	provider, repo, service, ctx := setupTestDB(t)
	defer provider.Close()

	// Insert some conflicting facts
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "fact-1",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Maya's cake price is $50",
		Embedding:      []float32{0.1, 0.2, 0.3},
		CreatedAt:      time.Now().Add(-2 * time.Hour),
		SourceTaskID:   "task-1",
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "fact-2",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Maya's cake price is $55",
		Embedding:      []float32{0.1, 0.2, 0.3},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-2",
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "unrelated-fact",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Maya sells cookies",
		Embedding:      []float32{0.1, 0.2, 0.3},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-3",
	})

	err := service.ResolveConflicts(ctx, "test-tenant-123", "cake price")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	memories, _ := repo.SemanticSearch(ctx, "test-tenant-123", []float32{0.1, 0.2, 0.3}, 10)

	// Should have deleted fact-1 and fact-2, kept unrelated-fact, and created 1 RESOLVED_FACT
	if len(memories) != 2 {
		t.Errorf("expected 2 memories (1 unrelated, 1 resolved), got %d", len(memories))
	}

	resolvedCount := 0
	unrelatedCount := 0
	for _, m := range memories {
		if m.MemoryType == "RESOLVED_FACT" {
			resolvedCount++
			if m.Content != "Maya's cake price is $55" {
				t.Errorf("expected resolved content to be 'Maya's cake price is $55', got %s", m.Content)
			}
		} else if m.ID == "unrelated-fact" {
			unrelatedCount++
		} else {
			t.Errorf("unexpected memory ID found: %s", m.ID)
		}
	}

	if resolvedCount != 1 {
		t.Errorf("expected 1 RESOLVED_FACT, got %d", resolvedCount)
	}
	if unrelatedCount != 1 {
		t.Errorf("expected 1 unrelated-fact, got %d", unrelatedCount)
	}
}

func TestPruneStaleContext(t *testing.T) {
	provider, repo, service, ctx := setupTestDB(t)
	defer provider.Close()

	// Insert stale and fresh facts
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "stale-fact",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Stale content",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "fresh-fact",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Fresh content",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now(),
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "permanent-fact",
		OrganizationID: "test-tenant-123",
		MemoryType:     "PERMANENT_FACT",
		Content:        "Permanent content",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
	})

	deleted, err := service.PruneStaleContext(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if deleted != 1 {
		t.Errorf("expected 1 record to be pruned, got %d", deleted)
	}

	memories, _ := repo.SemanticSearch(ctx, "test-tenant-123", []float32{0.1}, 10)
	if len(memories) != 2 {
		t.Errorf("expected 2 remaining memories, got %d", len(memories))
	}
}

func TestGetSharedContext(t *testing.T) {
	provider, repo, service, ctx := setupTestDB(t)
	defer provider.Close()

	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "context-1",
		OrganizationID: "test-tenant-123",
		MemoryType:     "TASK_SUMMARY",
		Content:        "Context part 1",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now(),
	})

	shared, err := service.GetSharedContext(ctx, "query")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if shared == "" {
		t.Errorf("expected some shared context, got empty string")
	}
}
