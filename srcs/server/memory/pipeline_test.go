package memory

import (
	"context"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestSweepCompletedTasks_Success(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	rows := sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
		AddRow("task1", "org1", "agent1", []byte("some valid content"))

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(rows)

	// Mock upsert memory
	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("INSERT INTO consolidated_memory").
		WithArgs(sqlmock.AnyArg(), "org1", "agent1", "task1", "some valid content", "[0.1,0.2,0.3]", "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	mock.ExpectExec("UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = \\$1").
		WithArgs("task1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSweepCompletedTasks_FailingLLM(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &FailingMockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	rows := sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
		AddRow("task1", "org1", "agent1", []byte("some valid content"))

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(rows)

	// Should not try to upsert since embedding fails

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSweepCompletedTasks_QueryError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnError(assert.AnError)

	err = daemon.SweepCompletedTasks(context.Background())
	assert.Error(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSweepCompletedTasks_ScanError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	// Wrong number of columns
	rows := sqlmock.NewRows([]string{"id", "organization_id"}).
		AddRow("task1", "org1")

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(rows)

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSweepCompletedTasks_UpsertError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	rows := sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
		AddRow("task1", "org1", "agent1", []byte("some valid content"))

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(rows)

	// Mock upsert memory failure
	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("INSERT INTO consolidated_memory").
		WithArgs(sqlmock.AnyArg(), "org1", "agent1", "task1", "some valid content", "[0.1,0.2,0.3]", "autodream").
		WillReturnError(assert.AnError)
	mock.ExpectRollback()

	// Update should not be called

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSweepCompletedTasks_UpdateError(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	rows := sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
		AddRow("task1", "org1", "agent1", []byte("some valid content"))

	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(rows)

	// Mock upsert memory
	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("INSERT INTO consolidated_memory").
		WithArgs(sqlmock.AnyArg(), "org1", "agent1", "task1", "some valid content", "[0.1,0.2,0.3]", "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	mock.ExpectExec("UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = \\$1").
		WithArgs("task1").
		WillReturnError(assert.AnError)

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}
