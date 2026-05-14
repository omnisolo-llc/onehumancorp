package autodream_pipeline

import (
	"database/sql"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/lib/pricing"
)

type MockEmbeddingApi struct {
	generateEmbeddingFunc func(text string) (string, error)
	calls                 int
}

func (m *MockEmbeddingApi) GenerateEmbedding(text string) (string, error) {
	m.calls++
	if m.generateEmbeddingFunc != nil {
		return m.generateEmbeddingFunc(text)
	}
	return "[0.1, 0.2, 0.3]", nil
}

type MockDB struct {
	isSQLite bool
	execFunc func(query string, args ...any) (sql.Result, error)
}

func (m *MockDB) IsSQLite() bool {
	return m.isSQLite
}

func (m *MockDB) Exec(query string, args ...any) (sql.Result, error) {
	if m.execFunc != nil {
		return m.execFunc(query, args...)
	}
	return nil, nil
}

func TestSweepAndConsolidate_SQLite(t *testing.T) {
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "test1.yml")
	err := os.WriteFile(testFile, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	execCalled := false
	db := &MockDB{
		isSQLite: true,
		execFunc: func(query string, args ...any) (sql.Result, error) {
			execCalled = true
			if !strings.Contains(query, "?") {
				t.Errorf("expected SQLite query (using ? placeholders), got %s", query)
			}
			return nil, nil
		},
	}

	api := &MockEmbeddingApi{}

	worker := NewAutoDreamWorker(db, api, memDir, nil)
	err = worker.SweepAndConsolidate()

	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if !execCalled {
		t.Errorf("expected db.Exec to be called")
	}

	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Errorf("expected file to be deleted")
	}
}

func TestSweepAndConsolidate_Postgres(t *testing.T) {
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "test2.yml")
	err := os.WriteFile(testFile, []byte("test content 2"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	execCalled := false
	db := &MockDB{
		isSQLite: false,
		execFunc: func(query string, args ...any) (sql.Result, error) {
			execCalled = true
			if !strings.Contains(query, "$") {
				t.Errorf("expected Postgres query (using $ placeholders), got %s", query)
			}
			return nil, nil
		},
	}

	api := &MockEmbeddingApi{}

	worker := NewAutoDreamWorker(db, api, memDir, nil)
	err = worker.SweepAndConsolidate()

	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if !execCalled {
		t.Errorf("expected db.Exec to be called")
	}

	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Errorf("expected file to be deleted")
	}
}

func TestSweepAndConsolidate_DirNotExist(t *testing.T) {
	db := &MockDB{}
	api := &MockEmbeddingApi{}

	worker := NewAutoDreamWorker(db, api, "/path/that/does/not/exist", nil)
	err := worker.SweepAndConsolidate()

	if err != nil {
		t.Errorf("expected no error for non-existent dir, got %v", err)
	}
}

func TestSweepAndConsolidate_ErrorHandling(t *testing.T) {
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "test_err.yml")
	err := os.WriteFile(testFile, []byte("test error content"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &MockDB{
		isSQLite: true,
		execFunc: func(query string, args ...any) (sql.Result, error) {
			return nil, sql.ErrConnDone
		},
	}

	api := &MockEmbeddingApi{}

	worker := NewAutoDreamWorker(db, api, memDir, nil)
	err = worker.SweepAndConsolidate()

	if err == nil {
		t.Errorf("expected error from SweepAndConsolidate when db Exec fails")
	}

	// Test GenerateEmbedding error
	db = &MockDB{isSQLite: true}
	apiErr := &MockEmbeddingApi{
		generateEmbeddingFunc: func(text string) (string, error) {
			return "", sql.ErrNoRows
		},
	}

	worker = NewAutoDreamWorker(db, apiErr, memDir, nil)
	err = worker.SweepAndConsolidate()

	if err == nil {
		t.Errorf("expected error from SweepAndConsolidate when embedding fails")
	}
}

func TestStartDaemon(t *testing.T) {
	db := &MockDB{}
	api := &MockEmbeddingApi{}
	worker := NewAutoDreamWorker(db, api, "/path/that/does/not/exist", nil)
	// Test creating worker with empty string
	worker2 := NewAutoDreamWorker(db, api, "", nil)
	if worker2.memDir != ".agent-task/memory" {
		t.Errorf("expected memDir to fallback to .agent-task/memory")
	}

	worker.StartDaemon(0)
}

func TestSweepAndConsolidate_WithCache(t *testing.T) {
	memDir := t.TempDir()

	testFile1 := filepath.Join(memDir, "test1.yml")
	err := os.WriteFile(testFile1, []byte("test content for cache"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	testFile2 := filepath.Join(memDir, "test2.yml")
	err = os.WriteFile(testFile2, []byte("test content for cache"), 0644) // Same content
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	db := &MockDB{
		isSQLite: true,
		execFunc: func(query string, args ...any) (sql.Result, error) {
			return nil, nil
		},
	}

	api := &MockEmbeddingApi{}
	cache := pricing.NewLocalEmbeddingCache(10 * time.Minute)

	worker := NewAutoDreamWorker(db, api, memDir, cache)

	// First sweep (cache miss)
	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Wait a little to ensure file deleted and write new file with same content
	time.Sleep(10 * time.Millisecond)
	err = os.WriteFile(testFile2, []byte("test content for cache"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	// Second sweep (cache hit)
	err = worker.SweepAndConsolidate()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if api.calls != 1 {
		t.Errorf("expected API to be called exactly once due to cache, got %d calls", api.calls)
	}
}
