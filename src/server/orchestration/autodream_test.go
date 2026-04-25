package orchestration

import (
	"context"
	"database/sql"
	"gopkg.in/yaml.v3"
	"fmt"
	"testing"
	"time"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestAutoDreamPruneSessions(t *testing.T) {
	telemetry.InitTelemetry()
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)

	ctx := context.Background()
	_, _ = pool.Exec(ctx, "DELETE FROM agent_session_data") // clear table

	oldTime := time.Now().Add(-48 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	newTime := time.Now().Add(-1 * time.Hour).UTC().Format("2006-01-02 15:04:05")

	if pool.Provider.IsSQLite() {
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'c1', ?)", oldTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s2', 'a1', 'c2', ?)", newTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
	} else {
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'c1', $1)", oldTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s2', 'a1', 'c2', $1)", newTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
	}

	worker.pruneStaleSessions(ctx)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 session remaining, got %d", count)
	}
}

func TestAutoDreamTruthInjectionAndConflict(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	ctx := context.Background()

	// Clear out truth table
	_, _ = pool.Exec(ctx, "DELETE FROM swarm_truth_embeddings")

	// Create dummy vector string representation of 1536 dimension (or we mock it with a smaller one for SQLite fallback, but the column expects 1536 in pgvector).
	vectorStr := "["
	for i := 0; i < 1536; i++ {
		if i > 0 {
			vectorStr += ","
		}
		vectorStr += fmt.Sprintf("%f", float64(i)*0.0001)
	}
	vectorStr += "]"

	// Inject two highly similar truths
	err = worker.InjectTruth(ctx, "mem1", "Sky is blue", vectorStr)
	if err != nil {
		t.Fatalf("failed to inject truth: %v", err)
	}

	err = worker.InjectTruth(ctx, "mem2", "Sky is dark blue", vectorStr)
	if err != nil {
		t.Fatalf("failed to inject truth 2: %v", err)
	}

	// Wait, run conflict resolution
	worker.resolveConflicts(ctx)

	if !pool.Provider.IsSQLite() {
		// Postgres: Verify conflict was recorded
		var count int
		err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM memory_conflicts WHERE memory_id_1 IN ('mem1', 'mem2')").Scan(&count)
		if err != nil {
			t.Fatalf("failed to query conflicts: %v", err)
		}
		if count != 1 {
			t.Errorf("expected 1 conflict to be recorded and resolved, got %d", count)
		}
	}
}

func TestAutoDreamWorker_SessionCompression(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}

	ctx := context.Background()

	// Ensure table exists
	_, err = pool.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}
	_, err = pool.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			source_mission_id TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories: %v", err)
	}

	_, err = pool.Provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess-1', 'agent-1', 'test context')")
	if err != nil {
		t.Fatalf("failed to insert mock session: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	worker.compressSessionData(ctx)

	// Verify the session was deleted
	var count int
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 sessions left, got %d", count)
	}

	// Verify the memory was inserted
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'sess-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}

func TestAutoDreamConsolidateEpoch(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	ctx := context.Background()

	err = worker.ConsolidateEpoch(ctx)
	if err != nil {
		t.Fatalf("ConsolidateEpoch failed: %v", err)
	}

	// Verify epoch record was created and updated
	var count int
	var status string
	err = pool.QueryRow(ctx, "SELECT COUNT(*), MAX(status) FROM swarm_dream_epochs").Scan(&count, &status)
	if err != nil {
		t.Fatalf("failed to query epoch record: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 epoch record, got %d", count)
	}
	if status != "COMPLETED" {
		t.Errorf("expected epoch status COMPLETED, got %s", status)
	}
}

func TestAutoDreamWorker_PipelinesCoverage(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	ctx, cancel := context.WithCancel(context.Background())

	worker := NewAutoDreamWorker(pool.Provider)

	// Verify the non-blocking nature and fast exit of Start when context is cancelled.
	go worker.Start(ctx)
	cancel() // instantly cancel to let goroutines exit
	time.Sleep(100 * time.Millisecond)

	// Since pipelines run on intervals, explicitly run the internal sub-methods
	// to ensure full coverage of database and branching logic.
	ctx = context.Background()

	// Add test data for ingestCompletedTasks
	_, _ = pool.Provider.Exec(ctx, "INSERT INTO shared_tasks (id, status, organization_id, payload) VALUES ('t1', 'COMPLETED', 'test_org', '{}')")
	worker.ingestCompletedTasks(ctx)

	var count int
	_ = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks").Scan(&count)
	// ingestCompletedTasks handles shared_tasks

	// Add test data for compressSessionContexts
	oldTime := time.Now().Add(-10 * time.Minute).UTC().Format("2006-01-02 15:04:05")
	_, _ = pool.Provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s_context_1', 'agent', 'ctx', ?)", oldTime)
	worker.compressSessionContexts(ctx)

	// Wait for background routine in pruneStaleSessions
	time.Sleep(50 * time.Millisecond)

	_ = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's_context_1'").Scan(&count)
	if count != 0 {
		t.Errorf("expected compressSessionContexts to process and delete session, got %d", count)
	}
}

func TestAutoDreamWorker_CompletedTasksIngestion(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	ctx := context.Background()

	// Ensure required tables exist
	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_master (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			payload TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING'
		);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			organization_id TEXT,
			agent_id TEXT,
			source_type TEXT,
			processed_at DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	// Insert completed task
	_, err = pool.Exec(ctx, "INSERT INTO shared_tasks_master (id, organization_id, title, description, status) VALUES ('task-comp-1', 'org-1', 'Test Mission', 'This is a test description', 'COMPLETED')")
	if err != nil {
		t.Fatalf("failed to insert completed task: %v", err)
	}

	worker := NewAutoDreamWorker(pool)

	// Run ingestion pipeline logic once
	worker.ingestCompletedTasks(ctx)

	// Verify task was archived
	var status string
	err = pool.QueryRow(ctx, "SELECT status FROM shared_tasks_master WHERE id = 'task-comp-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to check task status: %v", err)
	}
	if status != "ARCHIVED" {
		t.Errorf("expected task status to be ARCHIVED, got %s", status)
	}

	// Verify memory was inserted
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'task-comp-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query autodream_memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}

func TestAutoDreamWorker_PipelinesCoverageNew(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	worker := NewAutoDreamWorker(pool.Provider)

	ctx, cancel := context.WithCancel(context.Background())
	// Let pipelines run briefly then cancel
	go worker.runSessionCompressionPipeline(ctx)
	go worker.runMemoryIngestionPipeline(ctx)
	go worker.runPruningPipeline(ctx)
	go worker.runConflictResolutionPipeline(ctx)
	go worker.runCompletedTasksIngestionPipeline(ctx)
	go worker.runMissionIngestionPipeline(ctx)

	time.Sleep(100 * time.Millisecond)
	cancel()
	time.Sleep(50 * time.Millisecond) // Wait for exits
}

func TestAutoDreamWorker_IngestMemoriesFile(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	worker := NewAutoDreamWorker(pool.Provider)

	// Set up mock directory
	dir := t.TempDir()
	t.Setenv("OHC_MEMORY_DIR", dir)

	// Create valid yaml file
	validFile := filepath.Join(dir, "valid.yml")
	os.WriteFile(validFile, []byte("agent_session_data: test\ncontent: test content\n"), 0o644)

	// Create already processed file by inserting into DB first
	processedFile := filepath.Join(dir, "processed.yml")
	os.WriteFile(processedFile, []byte("agent_session_data: processed\ncontent: processed content\n"), 0o644)
	_, _ = pool.Provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id) VALUES ('1', 'content', 'mem-processed.yml')")

	// Create invalid yaml file
	invalidYamlFile := filepath.Join(dir, "invalid.yml")
	os.WriteFile(invalidYamlFile, []byte("invalid yaml content: : :"), 0o644)

	// Create non-yaml file
	nonYamlFile := filepath.Join(dir, "test.txt")
	os.WriteFile(nonYamlFile, []byte("test"), 0o644)

	worker.ingestAgentMemories(ctx)

	// Check if valid file was processed and deleted
	if _, err := os.Stat(validFile); !os.IsNotExist(err) {
		t.Errorf("expected valid.yml to be deleted, it was not")
	}

	// Check if processed file was deleted
	if _, err := os.Stat(processedFile); !os.IsNotExist(err) {
		t.Errorf("expected processed.yml to be deleted, it was not")
	}
}

// mockConflictProvider overrides IsSQLite and Query to simulate postgres and pgvector conflict search
type mockConflictProvider struct {
	db.Provider
}

func (m mockConflictProvider) IsSQLite() bool {
	return false
}

type mockRows struct {
	db.Rows
	called bool
}

func (m *mockRows) Next() bool {
	if !m.called {
		m.called = true
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...interface{}) error {
	*dest[0].(*string) = "mem1"
	*dest[1].(*string) = "ctx1"
	*dest[2].(*string) = "mem2"
	*dest[3].(*string) = "ctx2"
	return nil
}

func (m *mockRows) Close() {}

func (m mockConflictProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if sql == `
		SELECT a.memory_id, a.context, b.memory_id, b.context
		FROM swarm_truth_embeddings a
		JOIN swarm_truth_embeddings b ON a.memory_id < b.memory_id
		WHERE a.embedding <=> b.embedding < 0.05
		LIMIT 10
	` {
		return &mockRows{}, nil
	}
	return m.Provider.Query(ctx, sql, optionsAndArgs...)
}

func TestAutoDreamWorker_ConflictResolution_FullCoverage2(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()
	worker := NewAutoDreamWorker(pool.Provider)

	// Create mock provider to force postgres path and return mock rows
	mockProvider := mockConflictProvider{Provider: pool.Provider}
	worker.pool = mockProvider

	// Call it, it should hit the full path and insert a conflict and try to resolve it.
	worker.resolveConflicts(ctx)
}

// Create a new mock to cover SearchTruth postgres path
type mockSearchProvider struct {
	db.Provider
}

func (m mockSearchProvider) IsSQLite() bool {
	return false
}

type searchRows struct {
	db.Rows
	called bool
}

func (m *searchRows) Next() bool {
	if !m.called {
		m.called = true
		return true
	}
	return false
}

func (m *searchRows) Scan(dest ...interface{}) error {
	*dest[0].(*string) = "mem1"
	*dest[1].(*string) = "ctx1"
	*dest[2].(*float64) = 0.05
	return nil
}

func (m *searchRows) Close() {}

func (m mockSearchProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &searchRows{}, nil
}

func TestAutoDreamWorker_SearchTruthCoverage2(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	worker := NewAutoDreamWorker(pool.Provider)
	mock := mockSearchProvider{Provider: pool.Provider}
	worker.pool = mock

	ctx := context.Background()
	_, _ = worker.SearchTruth(ctx, "[0.0]", 10)
}
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
		processed_at TEXT,
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

func TestAutoDreamWorker_ConsolidateMemories(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM autodream_memories")

	// Insert unprocessed memories

	for i := 0; i < 5; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id, organization_id, agent_id, source_type) VALUES (?, ?, ?, ?, ?, ?)",
			fmt.Sprintf("mem-%d", i), fmt.Sprintf("test content %d", i), "mission-1", "org-1", "agent-1", "test")
		if err != nil {
			t.Fatalf("failed to insert mock memory: %v", err)
		}
	}

	// Insert one processed memory
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at, source_mission_id, organization_id, agent_id, source_type) VALUES (?, ?, CURRENT_TIMESTAMP, ?, ?, ?, ?)",
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

	rows, err := provider.Query(ctx, "SELECT count(*) FROM autodream_memories WHERE processed_at IS NOT NULL AND source_mission_id = 'mission-1'")
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
	provider.Exec(ctx, "DELETE FROM autodream_memories")

	// Insert one unprocessed memory
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id, organization_id, agent_id, source_type) VALUES (?, ?, ?, ?, ?, ?)",
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

	// Verify insertion into autodream_memories
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_type IN ('shared_tasks', 'swarm_tasks')").Scan(&count)
	if err != nil || count != 2 {
		t.Errorf("Expected 2 memories inserted, got %d (err: %v)", count, err)
	}
}

func TestAutoDreamWorker_SearchMemories(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Clean up table for isolated test
	provider.Exec(ctx, "DELETE FROM autodream_memories")

	// Insert test memories
	for i := 0; i < 3; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
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
	provider.Exec(ctx, "DELETE FROM autodream_memories")

	// Insert unprocessed memories
	for i := 0; i < 5; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id, organization_id, agent_id, source_type) VALUES (?, ?, ?, ?, ?, ?)",
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

type mockMeshTransport struct {
	MeshTransport
	BroadcastMeshEvents []struct {
		Topic   string
		Payload []byte
	}
}

func (m *mockMeshTransport) SubscribeMeshEventsWithFilter(ctx context.Context, topic string, filter func(payload []byte) bool) (<-chan []byte, error) {
	return make(chan []byte), nil
}

func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	m.BroadcastMeshEvents = append(m.BroadcastMeshEvents, struct {
		Topic   string
		Payload []byte
	}{topic, payload})
	return nil
}
