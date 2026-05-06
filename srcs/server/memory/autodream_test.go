package memory

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

type MockLLMClient struct {
	called bool
}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.called = true
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamDaemon(t *testing.T) {
	// Setup sqlmock
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	// Setup directories
	memDir := t.TempDir()
	missDir := t.TempDir()

	// Create test files
	doneFile := filepath.Join(memDir, "done.md")
	doneContent := "memory execution\nstatus: DONE\ntenant_id: org-123\nagent_id: agent-456\ntask_id: task-789\nresults..."
	err = os.WriteFile(doneFile, []byte(doneContent), 0644)
	assert.NoError(t, err)

	notDoneFile := filepath.Join(memDir, "not_done.md")
	err = os.WriteFile(notDoneFile, []byte("memory execution\nstatus: PENDING\nresults..."), 0644)
	assert.NoError(t, err)

	// Init daemon
	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Setup mock expectations
	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WithArgs("org-123").WillReturnResult(sqlmock.NewResult(0, 0))
	// Vector format comes from json marshal
	expectedEmbedding := "[0.1,0.2,0.3]"
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("done.md", "org-123", "agent-456", "task-789", doneContent, expectedEmbedding, "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	// Run process directories directly instead of starting the ticker to avoid races in tests
	daemon.processDirectories(ctx)

	// Check if mock LLM was called
	assert.True(t, mockLLM.called, "expected mock LLM to be called")

	// Ensure all db expectations were met
	assert.NoError(t, mock.ExpectationsWereMet())

	// Check if file was renamed
	_, err = os.Stat(doneFile + ".processed")
	assert.NoError(t, err, "expected done.md to be renamed to done.md.processed")

	_, err = os.Stat(notDoneFile)
	assert.NoError(t, err, "expected not_done.md to remain untouched")
}
