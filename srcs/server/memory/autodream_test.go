package memory

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
)

type MockLLMClient struct {
	called bool
}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.called = true
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS memory_embeddings (
			id TEXT PRIMARY KEY,
			content TEXT,
			vector_embedding BLOB
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestAutoDreamDaemon(t *testing.T) {
	// Setup db
	db := setupTestDB(t)
	defer db.Close()

	// Setup directories
	memDir := t.TempDir()
	missDir := t.TempDir()

	// Create test files
	doneFile := filepath.Join(memDir, "done.md")
	err := os.WriteFile(doneFile, []byte("memory execution\nstatus: DONE\nresults..."), 0644)
	assert.NoError(t, err)

	notDoneFile := filepath.Join(memDir, "not_done.md")
	err = os.WriteFile(notDoneFile, []byte("memory execution\nstatus: PENDING\nresults..."), 0644)
	assert.NoError(t, err)

	// Init daemon
	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Run process directories directly instead of starting the ticker to avoid races in tests
	daemon.processDirectories(ctx)

	// Check if mock LLM was called
	assert.True(t, mockLLM.called, "expected mock LLM to be called")

	// Check if DB has the embedding
	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM memory_embeddings").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count, "expected 1 embedding in database")

	// Check if file was renamed
	_, err = os.Stat(doneFile + ".processed")
	assert.NoError(t, err, "expected done.md to be renamed to done.md.processed")

	_, err = os.Stat(notDoneFile)
	assert.NoError(t, err, "expected not_done.md to remain untouched")
}
