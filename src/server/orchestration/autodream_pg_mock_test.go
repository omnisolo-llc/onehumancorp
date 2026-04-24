package orchestration

import (
	"context"
	"strings"
	"fmt"
	"path/filepath"
	"os"
	"testing"
	"github.com/onehumancorp/mono/src/server/db"
)

type mockPgProvider struct {
	db.Provider
	execCount int
	queries   []string
}

func (m *mockPgProvider) IsSQLite() bool {
	return false
}

func (m *mockPgProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &mockPgTx{Tx: tx, provider: m}, nil
}

func (m *mockPgProvider) Exec(ctx context.Context, sql string, arguments ...interface{}) (db.Result, error) {
	m.queries = append(m.queries, sql)
	m.execCount++
	return m.Provider.Exec(ctx, sql, arguments...)
}

func (m *mockPgProvider) Query(ctx context.Context, sql string, args ...interface{}) (db.Rows, error) {
	m.queries = append(m.queries, sql)

	// Intercept vector operator queries and replace them with standard SQLite syntax
	if strings.Contains(sql, "ORDER BY distance ASC") {
		// Mock query for testing pgvector SearchMemories path
		sql = "SELECT id, content, 0 as distance FROM autodream_memories LIMIT $2"
		return m.Provider.Query(ctx, sql, args[1])
	}

	return m.Provider.Query(ctx, sql, args...)
}

type mockPgTx struct {
	db.Tx
	provider *mockPgProvider
}

func (t *mockPgTx) Exec(ctx context.Context, sql string, arguments ...interface{}) (db.Result, error) {
	t.provider.queries = append(t.provider.queries, sql)

	if sql == "SELECT 1 FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED" {
		return t.Tx.Exec(ctx, "SELECT 1 FROM agent_session_data WHERE session_id = ?", arguments...)
	}

	// Intercept vector insert query
	if strings.Contains(sql, "$3::vector") {
		sql = strings.Replace(sql, "$3::vector", "$3", 1)
		// Convert postgres $N args to sqlite ? args
		for i := 1; i <= 7; i++ {
			sql = strings.Replace(sql, "$"+string(rune('0'+i)), "?", 1)
		}
		return t.Tx.Exec(ctx, sql, arguments...)
	}

	return t.Tx.Exec(ctx, sql, arguments...)
}

func (t *mockPgTx) Query(ctx context.Context, sql string, args ...interface{}) (db.Rows, error) {
	t.provider.queries = append(t.provider.queries, sql)
	if sql == "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500 FOR UPDATE SKIP LOCKED" {
		return t.Tx.Query(ctx, "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500", args...)
	}
	if sql == "SELECT id, title, COALESCE(payload, '{}') FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 500 FOR UPDATE SKIP LOCKED" {
		return t.Tx.Query(ctx, "SELECT id, title, COALESCE(payload, '{}') FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 500", args...)
	}
	if sql == "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 500 FOR UPDATE SKIP LOCKED" {
		return t.Tx.Query(ctx, "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 500", args...)
	}
	return t.Tx.Query(ctx, sql, args...)
}

func (t *mockPgTx) QueryRow(ctx context.Context, sql string, args ...interface{}) db.Row {
	t.provider.queries = append(t.provider.queries, sql)
	if strings.Contains(sql, "$1::vector") {
		sql = strings.Replace(sql, "$1::vector", "$1", 1)
		sql = strings.Replace(sql, "$2", "?", 1)
		sql = strings.Replace(sql, "$1", "?", 1)
		return t.Tx.QueryRow(ctx, sql, args...)
	}
	if strings.Contains(sql, "UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = $1 RETURNING id") {
		sql = "UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = ? RETURNING id"
		return t.Tx.QueryRow(ctx, sql, args...)
	}
	if strings.Contains(sql, "UPDATE swarm_tasks SET status = 'ARCHIVED' WHERE id = $1 RETURNING id") {
		sql = "UPDATE swarm_tasks SET status = 'ARCHIVED' WHERE id = ? RETURNING id"
		return t.Tx.QueryRow(ctx, sql, args...)
	}

	return t.Tx.QueryRow(ctx, sql, args...)
}

func TestAutoDreamWorker_ProcessMemories_Pg(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 2)

	// Pre-insert session data so lock query succeeds
	_, err := provider.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, content TEXT)")
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}
	_, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, content) VALUES ('mission-1', 'test content')")
	if err != nil {
		t.Fatalf("failed to insert agent_session_data: %v", err)
	}

	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())

	err = worker.ProcessMemories(context.Background())
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	err = provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}
	if count != 2 {
		t.Errorf("expected 2 memories inserted, got %d", count)
	}
}

func TestAutoDreamWorker_ProcessMemories_Pg_LockFailed(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 2)

	// Create agent_session_data but do not insert anything
	_, err := provider.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, content TEXT)")
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}

	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())

	err = worker.ProcessMemories(context.Background())
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	err = provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}
	// Because agent_session_data has no row, it should skip insertion
	if count != 0 {
		t.Errorf("expected 0 memories inserted due to lock failure, got %d", count)
	}
}

func TestAutoDreamWorker_IngestCompletedTasks_Pg(t *testing.T) {
	provider := setupTestDB(t)
	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)
	ctx := context.Background()

	// Ensure table exists for test
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")

	// Insert completed tasks
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, status) VALUES ('task-1', 'title-1', 'COMPLETED')")
	provider.Exec(ctx, "INSERT INTO swarm_tasks (id, title, status) VALUES ('task-2', 'title-2', 'COMPLETED')")

	err := worker.IngestCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("IngestCompletedTasks failed: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_type IN ('shared_tasks', 'swarm_tasks')").Scan(&count)
	if err != nil || count != 2 {
		t.Errorf("Expected 2 memories inserted, got %d (err: %v)", count, err)
	}
}

func TestAutoDreamWorker_SearchMemories_Pg(t *testing.T) {
	provider := setupTestDB(t)
	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)
	ctx := context.Background()

	provider.Exec(ctx, "DELETE FROM autodream_memories")

	for i := 0; i < 3; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
			fmt.Sprintf("mem-%d", i), fmt.Sprintf("content %d", i), "[]")
		if err != nil {
			t.Fatalf("failed to insert memory: %v", err)
		}
	}

	results, err := worker.SearchMemories(ctx, "[]", 2)
	if err != nil {
		t.Fatalf("SearchMemories failed: %v", err)
	}

	if len(results) != 2 {
		t.Errorf("expected 2 results, got %d", len(results))
	}
}

func TestAutoDreamWorker_ConsolidateMemories_Pg(t *testing.T) {
	provider := setupTestDB(t)
	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)
	ctx := context.Background()

	provider.Exec(ctx, "DELETE FROM autodream_memories")

	for i := 0; i < 5; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id, organization_id, agent_id, source_type) VALUES (?, ?, ?, ?, ?, ?)",
			fmt.Sprintf("mem-pg-%d", i), fmt.Sprintf("test content %d", i), "mission-1", "org-1", "agent-1", "test")
		if err != nil {
			t.Fatalf("failed to insert mock memory: %v", err)
		}
	}

	err := worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}

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

	if count != 5 {
		t.Errorf("expected 5 processed memories, got %d", count)
	}
}

// A simple mock for minimax client to trigger error/nil paths
type failingMinimaxClient struct{}

func (c *failingMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return nil, fmt.Errorf("minimax error")
}
func (c *failingMinimaxClient) GenerateCompletion(ctx context.Context, text string) (string, error) {
	return "", nil
}
func (c *failingMinimaxClient) ExtractActionableItems(ctx context.Context, text string) ([]string, error) {
	return nil, nil
}
func (c *failingMinimaxClient) PlanTask(ctx context.Context, title, content string) ([]SubTaskPlan, error) {
	return nil, nil
}

func TestAutoDreamWorkerDaemon_Coverage(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	provider := setupTestDB(t)

	// Create tables without data
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, content TEXT)")

	worker := NewAutoDreamWorker(provider)
	daemon := NewAutoDreamWorkerDaemon(worker)

	go daemon.Start(ctx)

	cancel()
	daemon.Stop()
}

// A provider that fails Begin
type failingTxProvider struct {
	db.Provider
}
func (p *failingTxProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, fmt.Errorf("begin failed")
}

func TestAutoDreamWorker_Failures(t *testing.T) {
	provider := setupTestDB(t)
	failProvider := &failingTxProvider{Provider: provider}
	worker := NewAutoDreamWorker(failProvider)
	ctx := context.Background()

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())
	setupMockMemories(t, 2)

	err := worker.ProcessMemories(ctx)
	if err == nil {
		t.Errorf("expected ProcessMemories to fail on Begin")
	}

	err = worker.IngestCompletedTasks(ctx)
	if err == nil {
		t.Errorf("expected IngestCompletedTasks to fail on Begin")
	}

	err = worker.ConsolidateMemories(ctx)
	if err == nil {
		t.Errorf("expected ConsolidateMemories to fail on Begin")
	}
}

// A provider that fails Commit
type failingCommitTx struct {
	db.Tx
}
func (tx *failingCommitTx) Commit(ctx context.Context) error {
	return fmt.Errorf("commit failed")
}
type failingCommitProvider struct {
	db.Provider
}
func (p *failingCommitProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := p.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &failingCommitTx{Tx: tx}, nil
}

func TestAutoDreamWorker_CommitFailures(t *testing.T) {
	provider := setupTestDB(t)
	failProvider := &failingCommitProvider{Provider: provider}
	worker := NewAutoDreamWorker(failProvider)
	ctx := context.Background()

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())
	setupMockMemories(t, 2)

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Errorf("expected ProcessMemories to not fail on Commit but log it, got %v", err)
	}

	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, status) VALUES ('task-1', 'title-1', 'COMPLETED')")

	err = worker.IngestCompletedTasks(ctx)
	if err == nil {
		t.Errorf("expected IngestCompletedTasks to fail on Commit")
	}

	provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, source_mission_id, organization_id, agent_id, source_type) VALUES ('mem-1', 'test content', 'mission-1', 'org-1', 'agent-1', 'test')")
	err = worker.ConsolidateMemories(ctx)
	if err == nil {
		t.Errorf("expected ConsolidateMemories to fail on Commit")
	}
}

// A provider that simulates lock errors in ProcessMemories
type lockErrorPgTx struct {
	db.Tx
	provider *mockPgProvider
}
func (t *lockErrorPgTx) Exec(ctx context.Context, sql string, arguments ...interface{}) (db.Result, error) {
	if sql == "SELECT 1 FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED" {
		return nil, fmt.Errorf("lock error")
	}
	return t.Tx.Exec(ctx, sql, arguments...)
}
func (t *lockErrorPgTx) Query(ctx context.Context, sql string, args ...interface{}) (db.Rows, error) {
	return t.Tx.Query(ctx, sql, args...)
}
func (t *lockErrorPgTx) QueryRow(ctx context.Context, sql string, args ...interface{}) db.Row {
	return t.Tx.QueryRow(ctx, sql, args...)
}

type lockErrorPgProvider struct {
	mockPgProvider
}
func (p *lockErrorPgProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := p.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &lockErrorPgTx{Tx: tx, provider: &p.mockPgProvider}, nil
}

func TestAutoDreamWorker_ProcessMemories_LockError(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 2)

	mockProvider := &lockErrorPgProvider{mockPgProvider{Provider: provider}}
	worker := NewAutoDreamWorker(mockProvider)

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())

	err := worker.ProcessMemories(context.Background())
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	err = provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&count)
	if count != 0 {
		t.Errorf("expected 0 memories inserted due to lock error, got %d", count)
	}
}

// A provider that simulates insertion error in ProcessMemories
type insertErrorPgTx struct {
	*mockPgTx
}
func (t *insertErrorPgTx) Exec(ctx context.Context, sql string, arguments ...interface{}) (db.Result, error) {
	if strings.Contains(sql, "INSERT INTO autodream_memories") {
		return nil, fmt.Errorf("insert error")
	}
	return t.mockPgTx.Exec(ctx, sql, arguments...)
}
type insertErrorPgProvider struct {
	mockPgProvider
}
func (p *insertErrorPgProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := p.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	mockTx := &mockPgTx{Tx: tx, provider: &p.mockPgProvider}
	return &insertErrorPgTx{mockPgTx: mockTx}, nil
}

func TestAutoDreamWorker_ProcessMemories_InsertError(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 2)

	// Pre-insert session data so lock query succeeds
	_, err := provider.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, content TEXT)")
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}
	_, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, content) VALUES ('mission-1', 'test content')")

	mockProvider := &insertErrorPgProvider{mockPgProvider{Provider: provider}}
	worker := NewAutoDreamWorker(mockProvider)

	t.Setenv("OHC_MEMORY_DIR", t.TempDir())

	err = worker.ProcessMemories(context.Background())
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	err = provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&count)
	if count != 0 {
		t.Errorf("expected 0 memories inserted due to insert error, got %d", count)
	}
}

func TestAutoDreamWorker_IngestCompletedTasks_Minimax(t *testing.T) {
	t.Setenv("MINIMAX_API_KEY", "dummy_key")
	provider := setupTestDB(t)
	mockProvider := &mockPgProvider{Provider: provider}
	worker := NewAutoDreamWorker(mockProvider)
	worker.SetLLMClient(&failingMinimaxClient{})
	ctx := context.Background()

	// Ensure table exists for test
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")

	// Insert completed tasks
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, title, status) VALUES ('task-1', 'title-1', 'COMPLETED')")

	err := worker.IngestCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("IngestCompletedTasks failed: %v", err)
	}
}

// A provider that simulates fetch error in ingestTasksFromTable
type fetchErrorPgTx struct {
	*mockPgTx
}
func (t *fetchErrorPgTx) Query(ctx context.Context, sql string, args ...interface{}) (db.Rows, error) {
	if strings.Contains(sql, "FROM shared_tasks") {
		return nil, fmt.Errorf("query error")
	}
	return t.mockPgTx.Query(ctx, sql, args...)
}
type fetchErrorPgProvider struct {
	mockPgProvider
}
func (p *fetchErrorPgProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := p.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	mockTx := &mockPgTx{Tx: tx, provider: &p.mockPgProvider}
	return &fetchErrorPgTx{mockPgTx: mockTx}, nil
}

func TestAutoDreamWorker_IngestCompletedTasks_FetchError(t *testing.T) {
	provider := setupTestDB(t)
	mockProvider := &fetchErrorPgProvider{mockPgProvider{Provider: provider}}
	worker := NewAutoDreamWorker(mockProvider)

	err := worker.IngestCompletedTasks(context.Background())
	if err == nil {
		t.Fatalf("expected error from IngestCompletedTasks")
	}
}

func TestAutoDreamWorker_IngestCompletedTasks_Empty(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT, title TEXT, payload TEXT, status TEXT)")
	// No tasks inserted

	err := worker.IngestCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("IngestCompletedTasks failed: %v", err)
	}
}

func TestAutoDreamWorker_ConsolidateMemories_Empty(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	err := worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}
}

func TestFormatFloat32SliceForVector(t *testing.T) {
	if formatFloat32SliceForVector(nil) != "[]" {
		t.Errorf("Expected []")
	}
	res := formatFloat32SliceForVector([]float32{1.5, -2.0, 0.0})
	if !strings.HasPrefix(res, "[") || !strings.HasSuffix(res, "]") {
		t.Errorf("Unexpected format: %s", res)
	}
}

func TestAutoDreamWorker_ProcessMemories_EdgeCases(t *testing.T) {
	provider := setupTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// glob error edge case is hard to trigger without custom FS, but we can hit empty match
	dir := t.TempDir()

	t.Setenv("OHC_MEMORY_DIR", dir)

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	// Add more than 500 files
	for i := 0; i < 505; i++ {
		content := fmt.Sprintf(`{"agent_session_data": "data %d"}`, i)
		os.WriteFile(filepath.Join(dir, fmt.Sprintf("test_%d.yml", i)), []byte(content), 0o644)
	}

	err = worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	// Create a file that can't be read (chmod 000)
	os.WriteFile(filepath.Join(dir, "unread.yml"), []byte(`{"agent_session_data": "unread"}`), 0o000)

	// Create an empty memory file
	os.WriteFile(filepath.Join(dir, "empty.yml"), []byte(`{"agent_session_data": "", "content": ""}`), 0o644)

	// Create a fallback content file
	os.WriteFile(filepath.Join(dir, "fallback.yml"), []byte(`{"content": "fallback content"}`), 0o644)

	err = worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}
}
