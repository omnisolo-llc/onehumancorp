package memory

import (
	"context"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestSweepCompletedTasks(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	// Mock sweep query
	mock.ExpectQuery("SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE'").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
			AddRow("task-1", "org-1", "agent-1", []byte("payload 1")))

	// Mock upsert memory
	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	expectedEmbedding := "[0.1,0.2,0.3]"
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org-1", "agent-1", "task-1", "payload 1", expectedEmbedding, "autodream").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	// Mock update task status
	mock.ExpectExec("UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = \\$1").
		WithArgs("task-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = daemon.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)

	assert.NoError(t, mock.ExpectationsWereMet())
}
