package orchestration

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

type mockLLMClient struct{}

func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamWorker(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_referenced_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			reference_count INTEGER DEFAULT 0,
			reliability_score INTEGER DEFAULT 50,
			owner_override BOOLEAN DEFAULT FALSE,
			metadata TEXT,
			task_id TEXT
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	tempDir := t.TempDir()
	yamlContent := `
task_id: task-123
tenant_id: tenant-abc
agent_id: agent-xyz
payload: some important task payload
deliberation_log: some log data
`
	err = os.WriteFile(filepath.Join(tempDir, "memory1.yaml"), []byte(yamlContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write yaml: %v", err)
	}

	worker := NewAutoDreamWorker(db, &mockLLMClient{}, tempDir)
	err = worker.ProcessMemoryFiles(context.Background())
	if err != nil {
		t.Fatalf("ProcessMemoryFiles failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT count(*) FROM consolidated_memory").Scan(&count)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 row inserted, got %d", count)
	}

	// Verify file was deleted
	files, _ := os.ReadDir(tempDir)
	if len(files) != 0 {
		t.Errorf("Expected memory file to be deleted, but %d files remain", len(files))
	}
}
