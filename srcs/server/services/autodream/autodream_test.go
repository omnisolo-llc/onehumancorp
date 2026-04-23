package autodream

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockLLMClient struct{}

func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	emb := make([]float32, 1536)
	emb[0] = 0.5
	return emb, nil
}

func TestKnowledgeWorker_ExtractFinalizedTasks(t *testing.T) {
	pool := db.NewTestProvider(t)
	defer pool.Close()

	client := &mockLLMClient{}
	worker := NewKnowledgeWorker(pool, client)

	_, err := pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			payload TEXT,
			status TEXT
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create shared_tasks_decomposition: %v", err)
	}

	_, err = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS knowledge_embeddings (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create knowledge_embeddings: %v", err)
	}

	_, err = pool.Exec(context.Background(), `
		INSERT INTO shared_tasks_decomposition (id, organization_id, payload, status)
		VALUES ('task1', 'org1', '{"test":"payload"}', 'COMPLETED')
	`)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	worker.ExtractFinalizedTasks(context.Background())

	var count int
	err = pool.QueryRow(context.Background(), "SELECT COUNT(*) FROM knowledge_embeddings WHERE task_id = 'task1'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query knowledge_embeddings: %v", err)
	}
	if count != 1 {
		t.Errorf("Expected 1 knowledge embedding, got %d", count)
	}
}

func TestSearchKnowledge(t *testing.T) {
	pool := db.NewTestProvider(t)
	defer pool.Close()

	client := &mockLLMClient{}

	_, err := pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS knowledge_embeddings (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create knowledge_embeddings: %v", err)
	}

	_, err = pool.Exec(context.Background(), `
		INSERT INTO knowledge_embeddings (id, tenant_id, task_id, content, embedding)
		VALUES ('mem1', 'org1', 'task1', 'content1', '[]')
	`)
	if err != nil {
		t.Fatalf("Failed to insert knowledge_embeddings: %v", err)
	}

	results, err := SearchKnowledge(context.Background(), pool, client, "org1", "query", 5)
	if err != nil {
		t.Fatalf("SearchKnowledge failed: %v", err)
	}

	if len(results) != 1 {
		t.Errorf("Expected 1 result, got %d", len(results))
	}
	if results[0].MemoryID != "mem1" {
		t.Errorf("Expected MemoryID mem1, got %s", results[0].MemoryID)
	}
}
