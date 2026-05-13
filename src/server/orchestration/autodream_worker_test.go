package orchestration

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDBForMemory(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	createTableQuery := `
	CREATE TABLE agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestAutoDreamWorker_ProcessMemories(t *testing.T) {
	db := setupTestDBForMemory(t)
	defer db.Close()

	tmpDir := t.TempDir()
	worker := NewAutoDreamWorker(db, tmpDir)

	// Create test memory file
	fileData := []byte("organization_id: test-org\nmemory_content: Test finding from agent")
	if err := os.WriteFile(filepath.Join(tmpDir, "test.yml"), fileData, 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	if err := worker.ProcessMemories(context.Background()); err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	// Verify insertion
	var count int
	err := db.QueryRow("SELECT count(*) FROM agent_memories WHERE organization_id = 'test-org'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 memory, got %d", count)
	}

	// Verify file deletion
	files, _ := filepath.Glob(filepath.Join(tmpDir, "*.yml"))
	if len(files) != 0 {
		t.Errorf("Expected file to be deleted, but %d remain", len(files))
	}
}
