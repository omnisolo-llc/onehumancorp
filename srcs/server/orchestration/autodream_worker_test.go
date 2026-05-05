package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

type MockDB struct{
    called bool
}

func (m *MockDB) ExecContext(ctx context.Context, query string, args ...any) error {
	m.called = true
    return nil
}

func (m *MockDB) IsSQLite() bool {
	return true
}

func TestAutoDreamWorker(t *testing.T) {
	tempDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tempDir, "test.yml"), []byte("dummy"), 0644)
	if err != nil {
		t.Fatal(err)
	}

    db := &MockDB{}
	worker := NewAutoDreamWorker(db, tempDir, func(s string) (string, error) {
		return "[0.1]", nil
	})

	err = worker.RunSweep(context.Background())
	if err != nil {
		t.Fatal(err)
	}

    if !db.called {
        t.Fatal("DB was not called")
    }
}
