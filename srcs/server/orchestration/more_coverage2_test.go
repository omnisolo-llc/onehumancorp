package orchestration

import (
	"context"
	"testing"
    "database/sql"

	"github.com/stretchr/testify/assert"
    "github.com/DATA-DOG/go-sqlmock"
)

func TestSubAgentWorker_Poll_QueryError(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)
	defer db.Close()

    // Mock Postgres (select sqlite_version fails)
    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}
	worker := NewSubAgentWorker(db, sm, spawner)

    mock.ExpectBegin()
    mock.ExpectQuery("SELECT id, parent_task_id FROM sub_agent_queue").WillReturnError(assert.AnError)
    mock.ExpectRollback()

    worker.Poll(context.Background())
}

func TestSubAgentWorker_Poll_UpdateError(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)
	defer db.Close()

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}
	worker := NewSubAgentWorker(db, sm, spawner)

    mock.ExpectBegin()
    rows := sqlmock.NewRows([]string{"id", "parent_task_id"}).AddRow("job-1", "task-1")
    mock.ExpectQuery("SELECT id, parent_task_id FROM sub_agent_queue").WillReturnRows(rows)
    mock.ExpectExec("UPDATE sub_agent_queue SET status = 'RUNNING'").WillReturnError(assert.AnError)
    mock.ExpectRollback()

    worker.Poll(context.Background())
}

func TestSubAgentWorker_Poll_CommitError(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)
	defer db.Close()

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}
	worker := NewSubAgentWorker(db, sm, spawner)

    mock.ExpectBegin()
    rows := sqlmock.NewRows([]string{"id", "parent_task_id"}).AddRow("job-1", "task-1")
    mock.ExpectQuery("SELECT id, parent_task_id FROM sub_agent_queue").WillReturnRows(rows)
    mock.ExpectExec("UPDATE sub_agent_queue SET status = 'RUNNING'").WillReturnResult(sqlmock.NewResult(1, 1))
    mock.ExpectCommit().WillReturnError(assert.AnError)

    worker.Poll(context.Background())
}

func TestSubAgentWorker_ProcessJob_UpdateError(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)
	defer db.Close()

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}
	worker := NewSubAgentWorker(db, sm, spawner)

    // Process event mock
    mock.ExpectBegin()
    mock.ExpectQuery("SELECT status, workflow_state FROM ohc_tasks").WillReturnError(sql.ErrNoRows) // Just let it fail and ignore
    mock.ExpectRollback()

    mock.ExpectBegin()
    mock.ExpectExec("UPDATE sub_agent_queue SET status = (.+), completed_at = CURRENT_TIMESTAMP").WillReturnError(assert.AnError)
    mock.ExpectRollback()

    worker.processJob(context.Background(), "job-1", "task-1")
}

func TestTaskStateMachine_ProcessEvent_CountError(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)

	sm := NewTaskStateMachine(db)

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

    mock.ExpectBegin()
    rows1 := sqlmock.NewRows([]string{"status", "workflow_state"}).AddRow("EXECUTING", nil)
    mock.ExpectQuery("SELECT status, workflow_state FROM ohc_tasks").WillReturnRows(rows1)

    mock.ExpectQuery("SELECT COUNT(.+) FROM ohc_tasks").WillReturnError(assert.AnError)

    mock.ExpectRollback()

	ctx := context.Background()
	err = sm.ProcessEvent(ctx, "task-1", EventSubTaskCompleted)
	assert.Error(t, err)
}

func TestTaskStateMachine_ProcessEvent_SubTaskCompleted_UpdateErr(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)

	sm := NewTaskStateMachine(db)

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

    mock.ExpectBegin()
    rows1 := sqlmock.NewRows([]string{"status", "workflow_state"}).AddRow("EXECUTING", nil)
    mock.ExpectQuery("SELECT status, workflow_state FROM ohc_tasks").WillReturnRows(rows1)

    rows2 := sqlmock.NewRows([]string{"COUNT(*)"}).AddRow(0)
    mock.ExpectQuery("SELECT COUNT(.+) FROM ohc_tasks").WillReturnRows(rows2)

    mock.ExpectExec("UPDATE ohc_tasks SET status = 'VERIFYING'").WillReturnError(assert.AnError)

    mock.ExpectRollback()

	ctx := context.Background()
	err = sm.ProcessEvent(ctx, "task-1", EventSubTaskCompleted)
	assert.Error(t, err)
}

func TestTaskStateMachine_ProcessEvent_Decomp_UpdateErr(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)

	sm := NewTaskStateMachine(db)

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

    mock.ExpectBegin()
    rows1 := sqlmock.NewRows([]string{"status", "workflow_state"}).AddRow("DECOMPOSING", nil)
    mock.ExpectQuery("SELECT status, workflow_state FROM ohc_tasks").WillReturnRows(rows1)

    mock.ExpectExec("UPDATE ohc_tasks SET status = 'EXECUTING'").WillReturnError(assert.AnError)

    mock.ExpectRollback()

	ctx := context.Background()
	err = sm.ProcessEvent(ctx, "task-1", EventDecompositionComplete)
	assert.Error(t, err)
}
