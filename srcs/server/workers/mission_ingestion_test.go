package workers

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)

	// Create required tables
	query := `CREATE TABLE IF NOT EXISTS autodream_memories_master (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_task_id TEXT,
		tenant_id TEXT,

		memory_type TEXT,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS agent_session_data (
		session_id TEXT PRIMARY KEY
	);
	`
	_, err = provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	return provider
}

func TestMissionIngestionWorker_StripHTML(t *testing.T) {
	input := `<div markdown="1" style="backdrop-filter: blur(20px);">Hello</div>`
	output := stripHTML(input)
	assert.Equal(t, "Hello", output)
}

func TestMissionIngestionWorker_IngestMissions(t *testing.T) {
	provider := setupTestDB(t)

	tmpDir, _ := os.MkdirTemp("", "mission-test")
	defer os.RemoveAll(tmpDir)
	missionsDir := filepath.Join(tmpDir, "missions")
	t.Setenv("OHC_MISSIONS_DIR", missionsDir)
	os.MkdirAll(missionsDir, 0755)

	content := `---
title: Test
---
<div class="glassmorphism">Test Content</div>`

	err := os.WriteFile(filepath.Join(missionsDir, "test_mission.md"), []byte(content), 0644)
	assert.NoError(t, err)

	worker := NewMissionIngestionWorker(provider)
	ctx := context.Background()

	// Ingest once
	worker.IngestMissions(ctx)

	// Verify insertion
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories_master WHERE source_task_id = 'test_mission'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	var savedContent string
	err = provider.QueryRow(ctx, "SELECT content FROM autodream_memories_master WHERE source_task_id = 'test_mission'").Scan(&savedContent)
	assert.NoError(t, err)

	// Should have stripped HTML
	assert.Contains(t, savedContent, "Test Content")
	assert.NotContains(t, savedContent, "<div")

	// Ingest again should not duplicate
	worker.IngestMissions(ctx)

	var countAfter int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories_master WHERE source_task_id = 'test_mission'").Scan(&countAfter)
	assert.NoError(t, err)
	assert.Equal(t, 1, countAfter)
}

func TestMissionIngestionWorker_IngestMissionsYML(t *testing.T) {
	provider := setupTestDB(t)

	tmpDir, _ := os.MkdirTemp("", "mission-test-yml")
	defer os.RemoveAll(tmpDir)
	missionsDir := filepath.Join(tmpDir, "missions")
	t.Setenv("OHC_MISSIONS_DIR", missionsDir)
	os.MkdirAll(missionsDir, 0755)

	content := `---
title: Test YAML
---
<div class="glassmorphism">Test Content YML</div>`

	err := os.WriteFile(filepath.Join(missionsDir, "test_mission.yml"), []byte(content), 0644)
	assert.NoError(t, err)

	worker := NewMissionIngestionWorker(provider)
	ctx := context.Background()

	// Ingest once
	worker.IngestMissions(ctx)

	// Verify insertion
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories_master WHERE source_task_id = 'test_mission'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}
