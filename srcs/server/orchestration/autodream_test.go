package orchestration

import (
	"database/sql"
	"errors"

	"os"
	"path/filepath"
	"testing"
	"time"
)

type mockDB struct {
	isSQLite bool
	execErr  error
	execArgs []any
}

func (m *mockDB) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockDB) Exec(query string, args ...any) (sql.Result, error) {
	m.execArgs = args
	return nil, m.execErr
}

type mockAPI struct {
	generateErr       error
	generateEmbedding string
}

func (m *mockAPI) GenerateEmbedding(text string) (string, error) {
	return m.generateEmbedding, m.generateErr
}

func TestSweepAndConsolidate(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("test memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1, 0.2, 0.3]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Fatalf("expected file to be deleted, but it still exists")
	}

	if len(db.execArgs) == 0 {
		t.Fatalf("expected database execution, but got none")
	}

	if db.execArgs[1] != "test memory content" {
		t.Errorf("expected content 'test memory content', got %v", db.execArgs[1])
	}
}

func TestSweepAndConsolidatePostgres(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory2.yml")
	err := os.WriteFile(testFile, []byte("postgres memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: false}
	api := &mockAPI{generateEmbedding: "[0.1, 0.2, 0.3]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Fatalf("expected file to be deleted, but it still exists")
	}

	if len(db.execArgs) == 0 {
		t.Fatalf("expected database execution, but got none")
	}

	if db.execArgs[1] != "postgres memory content" {
		t.Errorf("expected content 'postgres memory content', got %v", db.execArgs[1])
	}
}

func TestSweepAndConsolidateDirNotExist(t *testing.T) {
	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1]"}
	worker := NewAutoDreamWorker(db, api, "/does/not/exist/ever")
	err := worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
}

func TestSweepAndConsolidateIgnoreNonYml(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory1.txt")
	err := os.WriteFile(testFile, []byte("test memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if len(db.execArgs) > 0 {
		t.Fatalf("expected no db execution")
	}
}

func TestSweepAndConsolidateAPIError(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("test memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateErr: errors.New("api error")}
	worker := NewAutoDreamWorker(db, api, tempDir)

	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if len(db.execArgs) > 0 {
		t.Fatalf("expected no db execution")
	}
}

func TestSweepAndConsolidateDBError(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("test memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: true, execErr: errors.New("db error")}
	api := &mockAPI{generateEmbedding: "[0.1]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
}

func TestSweepAndConsolidateUnreadableFile(t *testing.T) {
	tempDir := t.TempDir()
	testFile := filepath.Join(tempDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("test memory content"), 0000)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	// In test environment, root might still read it. But at least we trigger WalkDir.
	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	os.Chmod(testFile, 0644) // cleanup
}

func TestStartDaemon(t *testing.T) {
	tempDir := t.TempDir()
	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1, 0.2, 0.3]"}
	worker := NewAutoDreamWorker(db, api, tempDir)

	worker.StartDaemon(1 * time.Millisecond)
	time.Sleep(10 * time.Millisecond) // Let it run a bit
}

func TestNewAutoDreamWorkerDefaultDir(t *testing.T) {
	db := &mockDB{isSQLite: true}
	api := &mockAPI{generateEmbedding: "[0.1]"}
	worker := NewAutoDreamWorker(db, api, "")
	if worker.memDir != ".agent-task/memory" {
		t.Fatalf("expected default dir .agent-task/memory, got %v", worker.memDir)
	}
}
