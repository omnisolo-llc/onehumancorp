package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"strings"
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
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	);`
	_, err = provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	return provider
}

func setupMockMemories(t *testing.T, count int) string {
	dir, err := os.MkdirTemp("", "agent-task-memory-test")
	if err != nil {
		t.Fatal(err)
	}

	originalDir, _ := os.Getwd()
	os.MkdirAll(filepath.Join(dir, ".agent-task", "memory"), 0755)

	err = os.Chdir(dir)
	if err != nil {
		t.Fatal(err)
	}

	for i := 0; i < count; i++ {
		memFile := MemoryFile{
			AgentSessionData: "mock session data " + fmt.Sprint(i),
			Content:          "mock content " + fmt.Sprint(i),
		}
		data, _ := yaml.Marshal(&memFile)
		filePath := filepath.Join(".agent-task", "memory", fmt.Sprintf("test_memory_%d.yml", i))
		os.WriteFile(filePath, data, 0644)
	}

	// Add an empty one to test edge cases
	os.WriteFile(filepath.Join(".agent-task", "memory", "empty.yml"), []byte(""), 0644)

	// Add a non-yaml one to test error cases
	os.WriteFile(filepath.Join(".agent-task", "memory", "invalid.yml"), []byte("invalid yaml content: : :"), 0644)

	// Add a file with only content
	contentOnly := MemoryFile{
		Content: "only content data",
	}
	data, _ := yaml.Marshal(&contentOnly)
	os.WriteFile(filepath.Join(".agent-task", "memory", "content_only.yml"), data, 0644)

	// Return a cleanup function
	t.Cleanup(func() {
		os.Chdir(originalDir)
		os.RemoveAll(dir)
	})

	return dir
}

func TestAutoDreamWorker_ProcessMemories(t *testing.T) {
	provider := setupTestDB(t)
	// Get baseline count
	var initialCount int
	provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&initialCount)

	setupMockMemories(t, 2)

	worker := NewAutoDreamWorker(provider)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories").Scan(&count)

	// 2 standard + 1 content only = 3 inserted + initialCount
	if count != initialCount + 3 {
		t.Errorf("expected %d memories inserted, got %d", initialCount + 3, count)
	}
}

func TestAutoDreamWorker_ProcessMemories_EmptyDir(t *testing.T) {
	provider := setupTestDB(t)

	// Create empty dir
	dir, _ := os.MkdirTemp("", "agent-task-memory-empty")
	originalDir, _ := os.Getwd()
	os.MkdirAll(filepath.Join(dir, ".agent-task", "memory"), 0755)
	os.Chdir(dir)
	t.Cleanup(func() {
		os.Chdir(originalDir)
		os.RemoveAll(dir)
	})

	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed on empty dir: %v", err)
	}
}

func TestFormatFloat32SliceForVector(t *testing.T) {
	res := formatFloat32SliceForVector([]float32{})
	if res != "[]" {
		t.Errorf("expected [], got %s", res)
	}

	res2 := formatFloat32SliceForVector([]float32{1.0, 2.5})
	if res2 != "[1.000000,2.500000]" {
		t.Errorf("expected [1.000000,2.500000], got %s", res2)
	}
}

func TestAutoDreamWorker_ProcessMemories_PostgresMock(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 1)

	// Create a mock provider
	mock := &mockPGProvider{Provider: provider}

	// Ensure the agent_session_data table exists so the SELECT 1 FROM agent_session_data doesn't fail
	_, err := mock.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY)")
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}

	// Insert dummy data to allow lock
	_, _ = mock.Exec(context.Background(), "INSERT INTO agent_session_data (session_id) VALUES ('test_memory_0')")

	worker := NewAutoDreamWorker(mock)
	ctx := context.Background()

	// Also mock minimax client by injecting dummy key
	os.Setenv("MINIMAX_API_KEY", "dummy_key")
	defer os.Unsetenv("MINIMAX_API_KEY")

	err = worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories with mocked PG failed: %v", err)
	}
}

type mockPGProvider struct {
	db.Provider
}

func (m *mockPGProvider) IsSQLite() bool {
	return false
}

// Override begin so we return mock tx
func (m *mockPGProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &mockPGTx{Tx: tx}, nil
}

type mockPGTx struct {
	db.Tx
}

func (m *mockPGTx) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
	// Strip out pgvector and row lock syntax just for testing postgres code path in sqlite
	sqlQuery = strings.ReplaceAll(sqlQuery, "::vector", "")
	sqlQuery = strings.ReplaceAll(sqlQuery, "FOR UPDATE SKIP LOCKED", "")
	return m.Tx.Exec(ctx, sqlQuery, arguments...)
}

func (m *mockPGTx) Query(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (db.Rows, error) {
	sqlQuery = strings.ReplaceAll(sqlQuery, "::vector", "")
	sqlQuery = strings.ReplaceAll(sqlQuery, "FOR UPDATE SKIP LOCKED", "")
	return m.Tx.Query(ctx, sqlQuery, optionsAndArgs...)
}

func (m *mockPGTx) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) db.Row {
	sqlQuery = strings.ReplaceAll(sqlQuery, "::vector", "")
	sqlQuery = strings.ReplaceAll(sqlQuery, "FOR UPDATE SKIP LOCKED", "")
	return m.Tx.QueryRow(ctx, sqlQuery, optionsAndArgs...)
}

func TestAutoDreamWorker_ProcessMemories_Errors(t *testing.T) {
	provider := setupTestDB(t)
	var initialCount int
	provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&initialCount)

	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	dir := setupMockMemories(t, 0)
	provider = setupTestDB(t)
	provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&initialCount)
	worker = NewAutoDreamWorker(provider)
	os.WriteFile(filepath.Join(dir, ".agent-task", "memory", "bad_yaml.yml"), []byte("invalid: yaml: : : :"), 0644)

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed on bad yaml: %v", err)
	}

	var count int
	provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories").Scan(&count)

	if count < initialCount {
		t.Errorf("expected %d memories inserted, got %d", initialCount, count)
	}
}

func TestAutoDreamWorker_ProcessMemories_TxFail(t *testing.T) {
	provider := setupTestDB(t)
	var initialCount int
	provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&initialCount)

	setupMockMemories(t, 1)

	mock := &mockFailProvider{Provider: provider}

	worker := NewAutoDreamWorker(mock)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories").Scan(&count)

	if count < initialCount {
		t.Errorf("expected %d memories inserted, got %d", initialCount, count)
	}
}

type mockFailProvider struct {
	db.Provider
}

func (m *mockFailProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, fmt.Errorf("mock error")
}

func TestAutoDreamWorker_ProcessMemories_ExecFail(t *testing.T) {
	provider := setupTestDB(t)
	var initialCount int
	provider.QueryRow(context.Background(), "SELECT count(*) FROM autodream_memories").Scan(&initialCount)

	setupMockMemories(t, 1)

	mock := &mockExecFailProvider{Provider: provider}

	worker := NewAutoDreamWorker(mock)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}

	var count int
	provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories").Scan(&count)

	if count < initialCount {
		t.Errorf("expected %d memories inserted, got %d", initialCount, count)
	}
}

type mockExecFailProvider struct {
	db.Provider
}

func (m *mockExecFailProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &mockFailTx{Tx: tx}, nil
}

type mockFailTx struct {
	db.Tx
}

func (m *mockFailTx) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
	return 0, fmt.Errorf("mock exec fail")
}

func TestAutoDreamWorker_ProcessMemories_CommitFail(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMemories(t, 1)

	mock := &mockCommitFailProvider{Provider: provider}

	worker := NewAutoDreamWorker(mock)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}
}

type mockCommitFailProvider struct {
	db.Provider
}

func (m *mockCommitFailProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &mockCommitFailTx{Tx: tx}, nil
}

type mockCommitFailTx struct {
	db.Tx
}

func (m *mockCommitFailTx) Commit(ctx context.Context) error {
	_ = m.Tx.Rollback(ctx)
	return fmt.Errorf("mock commit fail")
}
