package orchestration

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"strings"
    "path/filepath"

	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
)

func TestAutoDreamWorker(t *testing.T) {
    dbURL := os.Getenv("DATABASE_URL")
    if dbURL == "" {
        dbURL = "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable"
    }

	db, err := sql.Open("postgres", dbURL)
    if err != nil {
        t.Skip("Skipping test because postgres is not available")
    }
	defer db.Close()

    err = db.Ping()
    if err != nil {
        t.Skip("Skipping test because postgres is not reachable")
    }

    _, err = db.Exec("CREATE EXTENSION IF NOT EXISTS vector;")
    assert.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id SERIAL PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			content TEXT,
			embedding VECTOR(1536)
		);
	`)
	assert.NoError(t, err)

	_, err = db.Exec("TRUNCATE TABLE consolidated_memory;")
	assert.NoError(t, err)

    tmpDir := t.TempDir()
    err = os.WriteFile(filepath.Join(tmpDir, "test.txt"), []byte("test memory"), 0644)
	assert.NoError(t, err)

	worker := &AutoDreamWorker{DB: db, MemoryDir: tmpDir}

    ctx := context.WithValue(context.Background(), "tenant_id", "test_tenant")

	err = worker.Run(ctx)
	assert.NoError(t, err)

    var mockEmbedding []string
    for i := 0; i < 1536; i++ {
        mockEmbedding = append(mockEmbedding, "0.0")
    }
    embeddingStr := "[" + strings.Join(mockEmbedding, ",") + "]"

	results, err := worker.Search(ctx, embeddingStr)
	assert.NoError(t, err)
	assert.Len(t, results, 1)
	assert.Equal(t, "test memory", results[0])
}
