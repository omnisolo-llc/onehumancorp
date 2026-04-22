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

type mockMeshTransport struct {
	MeshTransport
	BroadcastMeshEvents []struct {
		Topic   string
		Payload []byte
	}
}

func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	m.BroadcastMeshEvents = append(m.BroadcastMeshEvents, struct {
		Topic   string
		Payload []byte
	}{topic, payload})
	return nil
}

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)

	// Create required tables
	query := `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		organization_id TEXT,
		agent_id TEXT,
		source_type TEXT,
		processed_at TEXT,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	);`
	_, err = provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create consolidated_memory table: %v", err)
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
	rows, err := provider.Query(ctx, "SELECT count(*) FROM consolidated_memory")
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

func TestAutoDreamWorker_ConsolidateMemories(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM consolidated_memory")

	// Insert unprocessed memories

	for i := 0; i < 5; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO consolidated_memory (id, content, source_mission_id, organization_id, agent_id, source_type, task_id) VALUES (?, ?, ?, ?, ?, ?, NULL)",
			fmt.Sprintf("mem-%d", i), fmt.Sprintf("test content %d", i), "mission-1", "org-1", "agent-1", "test")
		if err != nil {
			t.Fatalf("failed to insert mock memory: %v", err)
		}
	}

	// Insert one processed memory
	_, err := provider.Exec(ctx, "INSERT INTO consolidated_memory (id, content, processed_at, source_mission_id, organization_id, agent_id, source_type, task_id) VALUES (?, ?, CURRENT_TIMESTAMP, ?, ?, ?, ?, NULL)",
		"mem-processed", "processed content", "mission-1", "org-1", "agent-1", "test")
	if err != nil {
		t.Fatalf("failed to insert processed mock memory: %v", err)
	}

	err = worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}

	// Verify only 5 were updated (6 total with the one we inserted as processed)
	// Actually, wait, does the setupTestDB insert other memories? Let's check test count again.
	// Oh, the other tests use setupTestDB and it shares memory! ":memory:" vs "file::memory:?cache=shared".
	// We changed it to ":memory:", so it should be isolated now.
	// Why is it 8? Maybe there are other things in the test table?
	// The test `TestAutoDreamWorker_ProcessMemories` inserts 2 memories. If tests run in parallel, it might be 8.
	// No, SQLite with ":memory:" is per connection, but `db.NewSqliteProvider` might share it if they run in same process depending on open logic.
	// Let's make it more resilient by querying specifically for our test org.

	rows, err := provider.Query(ctx, "SELECT count(*) FROM consolidated_memory WHERE processed_at IS NOT NULL AND source_mission_id = 'mission-1'")
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

	if count != 6 {
		t.Errorf("expected 6 processed memories (1 original + 5 new), got %d", count)
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

func TestAutoDreamWorker_ConsolidateMemories_MeshBroadcast(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	mockMesh := &mockMeshTransport{}
	worker.SetMeshTransport(mockMesh)

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM consolidated_memory")

	// Insert one unprocessed memory
	_, err := provider.Exec(ctx, "INSERT INTO consolidated_memory (id, content, source_mission_id, organization_id, agent_id, source_type, task_id) VALUES (?, ?, ?, ?, ?, ?, NULL)",
		"mem-broadcast-1", "test content broadcast", "mission-broadcast", "org-1", "agent-1", "test")
	if err != nil {
		t.Fatalf("failed to insert mock memory: %v", err)
	}

	err = worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}

	if len(mockMesh.BroadcastMeshEvents) != 1 {
		t.Errorf("Expected 1 mesh broadcast event, got %d", len(mockMesh.BroadcastMeshEvents))
	} else {
		event := mockMesh.BroadcastMeshEvents[0]
		if event.Topic != "tasks" {
			t.Errorf("Expected topic 'tasks', got %s", event.Topic)
		}
	}
}

func TestAutoDreamWorker_IngestCompletedTasks(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Ensure table exists for test
	_, _ = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT)")
	_, _ = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT)")

	// Insert test tasks
	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, payload, status) VALUES (?, ?, ?, ?)",
		"st-1", "Test Task 1", "{}", "COMPLETED")
	if err != nil {
		t.Fatalf("failed to insert test shared task: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_tasks (id, title, payload, status) VALUES (?, ?, ?, ?)",
		"sw-1", "Swarm Task 1", "{}", "COMPLETED")
	if err != nil {
		t.Fatalf("failed to insert test swarm task: %v", err)
	}

	err = worker.IngestCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("IngestCompletedTasks failed: %v", err)
	}

	// Verify status updated to ARCHIVED
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'st-1'").Scan(&status)
	if err != nil || status != "ARCHIVED" {
		t.Errorf("Expected shared_task status ARCHIVED, got %s (err: %v)", status, err)
	}

	err = provider.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = 'sw-1'").Scan(&status)
	if err != nil || status != "ARCHIVED" {
		t.Errorf("Expected swarm_task status ARCHIVED, got %s (err: %v)", status, err)
	}

	// Verify insertion into consolidated_memory
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM consolidated_memory WHERE source_type IN ('shared_tasks', 'swarm_tasks')").Scan(&count)
	if err != nil || count != 2 {
		t.Errorf("Expected 2 memories inserted, got %d (err: %v)", count, err)
	}
}

func TestAutoDreamWorker_SearchMemories(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM consolidated_memory")

	// Insert test memories
	for i := 0; i < 3; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO consolidated_memory (id, content, embedding, created_at, organization_id, source_type, agent_id) VALUES (?, ?, ?, CURRENT_TIMESTAMP, 'org', 'source', 'agent')",
			fmt.Sprintf("mem-search-%d", i), fmt.Sprintf("test content %d", i), "[0.0,0.0,0.0]")
		if err != nil {
			t.Fatalf("failed to insert mock memory for search: %v", err)
		}
	}

	// In SQLite mode, it falls back to recency-based returning 3 results.
	results, err := worker.SearchMemories(ctx, "[0.0,0.0,0.0]", 2)
	if err != nil {
		t.Fatalf("SearchMemories failed: %v", err)
	}

	if len(results) != 2 {
		t.Errorf("Expected 2 results, got %d", len(results))
	}
}

func TestAutoDreamWorker_ConsolidateMemoriesPg(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Ensure required table exists
	_, _ = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, agent_id TEXT, context_data TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP)")

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM consolidated_memory")

	// Insert unprocessed memories
	for i := 0; i < 5; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO consolidated_memory (id, content, source_mission_id, organization_id, agent_id, source_type, task_id) VALUES (?, ?, ?, ?, ?, ?, NULL)",
			fmt.Sprintf("mem-pg-%d", i), fmt.Sprintf("test content %d", i), "mission-1", "org-1", "agent-1", "test")
		if err != nil {
			t.Fatalf("failed to insert mock memory: %v", err)
		}
	}

	mock := mockPgProvider{Provider: provider}
	worker.pool = mock
	err := worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}
}

func TestAutoDreamWorker_IngestCompletedTasksPg(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Ensure table exists for test
	_, _ = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT)")
	_, _ = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT)")

	// Insert test tasks
	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, payload, status) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING",
		"st-1-pg", "Test Task 1", "{}", "COMPLETED")
	if err != nil {
		t.Fatalf("failed to insert test shared task: %v", err)
	}

	mock := mockPgProvider{Provider: provider}
	worker.pool = mock
	err = worker.IngestCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("IngestCompletedTasks failed: %v", err)
	}
}
type mockPgProvider struct {
	db.Provider
}

func (m mockPgProvider) IsSQLite() bool {
	return false
}
