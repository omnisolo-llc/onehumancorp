package kairos

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestAutoDreamWorker(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()
	pool := db.NewSqliteProvider(sqlDB)
	defer pool.Close()

	ctx := context.Background()
	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	tmpDir := t.TempDir()

	// Create mock .agent-task/memory
	memDir := filepath.Join(tmpDir, ".agent-task/memory")
	err = os.MkdirAll(memDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create mock memory directory: %v", err)
	}

	// Change working directory to tmpDir so the fallback glob finds it
	origDir, _ := os.Getwd()
	os.Chdir(tmpDir)
	defer os.Chdir(origDir)

	testFile := filepath.Join(memDir, "test_memory.yml")
	err = os.WriteFile(testFile, []byte(`
agent_session_data: "this is test content"
status: "COMPLETED"
`), 0644)
	if err != nil {
		t.Fatalf("Failed to write mock memory file: %v", err)
	}

	// Write another file that is not completed
	testFile2 := filepath.Join(memDir, "test_memory_pending.yml")
	err = os.WriteFile(testFile2, []byte(`
agent_session_data: "should not process"
status: "PENDING"
`), 0644)
	if err != nil {
		t.Fatalf("Failed to write mock memory file: %v", err)
	}

    // Write a malformed yaml file
	testFile3 := filepath.Join(memDir, "test_memory_bad.yml")
	err = os.WriteFile(testFile3, []byte(`
agent_session_data: "should not process
status: "PENDING"
`), 0644)
	if err != nil {
		t.Fatalf("Failed to write mock memory file: %v", err)
	}

	worker := NewAutoDreamWorker(pool)

	// Run full pipeline
	err = worker.Run(ctx)
	if err != nil {
		t.Fatalf("Failed to run worker: %v", err)
	}

	var count int
	row := pool.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories")
	err = row.Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query DB: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 row, got %d", count)
	}

	// Test processMemory explicitly to increase coverage
	err = worker.processMemory(ctx, "content", "source")
	if err != nil {
		t.Fatalf("processMemory failed: %v", err)
	}

	// Also test Postgres path for processMemory if we can inject a mock pool, or just rely on another test
    // Actually, we can use db.NewPostgresProvider with a mock, or we can just mock db.Provider
}

type mockPool struct {
    db.Provider
    isSqlite bool
    execErr error
}

func (m *mockPool) IsSQLite() bool {
    return m.isSqlite
}

func (m *mockPool) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTx{execErr: m.execErr}, nil
}

type mockTx struct {
    db.Tx
    execErr error
}

func (m *mockTx) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
    if m.execErr != nil {
        return 0, m.execErr
    }
    return 1, nil
}

func (m *mockTx) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) db.Row {
    return &mockRow{}
}

func (m *mockTx) Commit(ctx context.Context) error {
    return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
    return nil
}

type mockRow struct {
    db.Row
}
func (m *mockRow) Scan(dest ...any) error {
    return nil
}

func TestAutoDreamWorker_PostgresPath(t *testing.T) {
    mp := &mockPool{isSqlite: false}
    worker := NewAutoDreamWorker(mp)
    err := worker.processMemory(context.Background(), "test", "test")
    if err != nil {
        t.Fatalf("Expected no err, got %v", err)
    }

    mp.execErr = sql.ErrNoRows // force error
    err = worker.processMemory(context.Background(), "test", "test")
    if err == nil {
        t.Fatalf("Expected error, got nil")
    }
}
