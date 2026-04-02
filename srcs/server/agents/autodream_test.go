package agents

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func setupDreamDB(t *testing.T) db.Provider {
	provider, err := db.NewSqliteProviderMemory()
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT,
			description TEXT,
			status TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			source_mission_id TEXT UNIQUE,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	assert.NoError(t, err)
	return provider
}

func TestAutoDreamManager(t *testing.T) {
	provider := setupDreamDB(t)
	defer provider.Close()

	ctx := context.Background()

	// Seed completed tasks
	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, description, status) VALUES ('t1', 'Fix DB', 'Use UUIDs', 'COMPLETED')")
	assert.NoError(t, err)

	os.Setenv("OHC_TEST_MODE", "true")
	defer os.Unsetenv("OHC_TEST_MODE")

	adm := NewAutoDreamManager(provider, nil)
	adm.Start()

	// Wait a moment for sweep
	time.Sleep(1 * time.Second)
	adm.Stop()

	// Check if memory was inserted
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 't1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	var content string
	err = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE source_mission_id = 't1'").Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Fix DB: Use UUIDs", content)
}
