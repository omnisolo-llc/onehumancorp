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

type MockLLMClient struct{}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

type FailingMockLLMClient struct{}

func (m *FailingMockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return nil, assert.AnError
}

func TestAutoDreamDaemon_ProcessDirectories(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()

	doneFile := filepath.Join(memDir, "done_task.md")
	doneContent := "memory execution\nstatus: DONE\ntenant_id: org-123\nagent_id: agent-456\ntask_id: task-789\nresults..."
	err = os.WriteFile(doneFile, []byte(doneContent), 0644)
	assert.NoError(t, err)

	notDoneFile := filepath.Join(missDir, "pending_mission.json")
	err = os.WriteFile(notDoneFile, []byte("memory execution\nstatus: PENDING\nresults..."), 0644)
	assert.NoError(t, err)

    // And a directory
    err = os.Mkdir(filepath.Join(memDir, "somedir"), 0755)
	assert.NoError(t, err)

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WithArgs("org-123").
		WillReturnResult(sqlmock.NewResult(0, 0))

	mock.ExpectExec("INSERT INTO consolidated_memory").
		WithArgs("done_task.md", "org-123", "agent-456", "task-789", doneContent, "[0.1,0.2,0.3]", "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	daemon.processDirectories(context.Background())

	assert.NoError(t, mock.ExpectationsWereMet())

	_, err = os.Stat(doneFile)
	assert.True(t, os.IsNotExist(err))

	_, err = os.Stat(doneFile + ".processed")
	assert.NoError(t, err)

	_, err = os.Stat(notDoneFile)
	assert.NoError(t, err)
}

func TestAutoDreamDaemon_UpsertFailure(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()

	doneFile := filepath.Join(memDir, "done_task.md")
	doneContent := "memory execution\nstatus: DONE\n"
	err = os.WriteFile(doneFile, []byte(doneContent), 0644)
	assert.NoError(t, err)

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WithArgs("system").
		WillReturnResult(sqlmock.NewResult(0, 0))

	mock.ExpectExec("INSERT INTO consolidated_memory").WillReturnError(assert.AnError)
	mock.ExpectRollback()

	daemon.processDirectories(context.Background())

	assert.NoError(t, mock.ExpectationsWereMet())

	// File should not be renamed if upsert fails
	_, err = os.Stat(doneFile)
	assert.NoError(t, err)
}

func TestAutoDreamDaemon_TenantFallback(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()

	doneFile := filepath.Join(memDir, "agent-123_task-456_tenant-fallback123.md")
	doneContent := "memory execution\nstatus: DONE\n"
	err = os.WriteFile(doneFile, []byte(doneContent), 0644)
	assert.NoError(t, err)

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WithArgs("fallback123").
		WillReturnResult(sqlmock.NewResult(0, 0))

	mock.ExpectExec("INSERT INTO consolidated_memory").
		WithArgs("agent-123_task-456_tenant-fallback123.md", "fallback123", "system", "system", doneContent, "[0.1,0.2,0.3]", "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	daemon.processDirectories(context.Background())
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestAutoDreamDaemon_Run(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

    mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}))

    mock.ExpectExec("DELETE FROM consolidated_memory").WillReturnResult(sqlmock.NewResult(0, 0))
    mock.ExpectExec("DELETE FROM consolidated_memory").WillReturnResult(sqlmock.NewResult(0, 0))

	ctx, cancel := context.WithCancel(context.Background())

	// Run it in background and cancel shortly after
	go daemon.Run(ctx)
	time.Sleep(20 * time.Millisecond)
	cancel()
}

func TestAutoDreamDaemon_ProcessDirectories_FailingLLM(t *testing.T) {
	db, _, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()

	doneFile := filepath.Join(memDir, "done_task.md")
	doneContent := "memory execution\nstatus: DONE\n"
	err = os.WriteFile(doneFile, []byte(doneContent), 0644)
	assert.NoError(t, err)

	mockLLM := &FailingMockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	daemon.processDirectories(context.Background())

	// File should not be renamed if LLM fails
	_, err = os.Stat(doneFile)
	assert.NoError(t, err)
}

func TestAutoDreamDaemon_ProcessFile_InvalidPath(t *testing.T) {
    db, _, err := sqlmock.New()
    assert.NoError(t, err)
    defer db.Close()

    mockLLM := &MockLLMClient{}
    daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
    assert.NoError(t, err)

    daemon.processFile(context.Background(), "invalid/path/that/does/not/exist.md")
}

func TestAutoDreamDaemon_ProcessDirectories_WalkError(t *testing.T) {
    db, _, err := sqlmock.New()
    assert.NoError(t, err)
    defer db.Close()

    mockLLM := &MockLLMClient{}
    // Pass a path that doesn't exist
    daemon, err := NewAutoDreamDaemon(db, mockLLM, "/invalid/path/1", "/invalid/path/2", 10*time.Millisecond)
    assert.NoError(t, err)

    daemon.processDirectories(context.Background())
}

func TestAutoDreamDaemon_UpsertMemory_BeginError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

    mock.ExpectBegin().WillReturnError(assert.AnError)
    err = daemon.upsertMemory(context.Background(), "id", "org", "agent", "task", "content", []byte{})
    assert.Error(t, err)
}

func TestAutoDreamDaemon_UpsertMemory_SetConfigError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

    mock.ExpectBegin()
    mock.ExpectExec("SELECT set_config").WillReturnError(assert.AnError)
    // mock.ExpectRollback()

    err = daemon.upsertMemory(context.Background(), "id", "org", "agent", "task", "content", []byte{})
    assert.Error(t, err)
}
