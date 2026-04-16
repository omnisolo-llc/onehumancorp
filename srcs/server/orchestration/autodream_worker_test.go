package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"gopkg.in/yaml.v3"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)

	// Create required tables
	query := `CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		organization_id TEXT,
		agent_id TEXT,
		source_type TEXT,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	);`
	_, err = provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	return provider
}

// setupMockMemories creates YAML memory files in a temporary directory and
// configures OHC_MEMORY_DIR to point at it.  Returns the temp dir path.
func setupMockMemories(t *testing.T, count int) string {
	t.Helper()
	dir := t.TempDir()
	t.Setenv("OHC_MEMORY_DIR", dir)

	for i := 0; i < count; i++ {
		memFile := MemoryFile{
			AgentSessionData: "mock session data " + fmt.Sprint(i),
			Content:          "mock content " + fmt.Sprint(i),
		}
		data, _ := yaml.Marshal(&memFile)
		filePath := filepath.Join(dir, fmt.Sprintf("test_memory_%d.yml", i))
		os.WriteFile(filePath, data, 0o644)
	}

	// Add an empty one to test edge cases
	os.WriteFile(filepath.Join(dir, "empty.yml"), []byte(""), 0o644)

	// Add a non-yaml one to test error cases
	os.WriteFile(filepath.Join(dir, "invalid.yml"), []byte("invalid yaml content: : :"), 0o644)

	return dir
}

func TestAutoDreamWorker_ProcessMemories(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 2)

	worker := NewAutoDreamWorker(provider)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	// Verify insertion
	rows, err := provider.Query(ctx, "SELECT count(*) FROM autodream_memories")
	if err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}
	defer rows.Close()

	var count int
	if rows.Next() {
		if err := rows.Scan(&count); err != nil {
			t.Fatalf("failed to scan count: %v", err)
		}
	}

	if count != 2 {
		t.Errorf("expected 2 memories inserted, got %d", count)
	}
}

func TestAutoDreamWorker_ProcessMemories_EmptyDir(t *testing.T) {
	provider := setupTestDB(t)

	// Point at an empty temp directory.
	t.Setenv("OHC_MEMORY_DIR", t.TempDir())

	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed on empty dir: %v", err)
	}
}

func TestAutoDreamWorkerDaemon_StartStop(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	provider := setupTestDB(t)
	worker := &AutoDreamWorker{pool: provider}
	daemon := NewAutoDreamWorkerDaemon(worker)

	go daemon.Start(ctx)

	// ensure it doesn't panic on multiple stops
	daemon.Stop()
	daemon.Stop()
	cancel() // to clean up contexts and verify ctx.Done doesn't panic
}
