package autodream

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	if len(prompt) > 20 && prompt[:20] == "You are an AI Memory" {
		if prompt[:24] == "You are an AI Memory Pru" {
			return "YES", nil
		}
		return `{"superseded_ids":["mem-1"], "summary":"Mock summary resolved"}`, nil
	}
	return "Mock summary resolved", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamConsolidationWithSQLite(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
			source_task_id VARCHAR
		);
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err = service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	err = service.PruneStaleMemories(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	ctxCancel, cancel := context.WithCancel(ctx)
	service.StartGlobalPruningWorker(ctxCancel, 10*time.Millisecond, 24*time.Hour, provider)
	time.Sleep(20 * time.Millisecond)
	cancel()
	time.Sleep(20 * time.Millisecond)
}
