package orchestration

import (
	"context"
	"testing"
    "time"

	"github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
    "github.com/DATA-DOG/go-sqlmock"
)

func TestTaskStateMachine_SetRedisClient(t *testing.T) {
    sm := NewTaskStateMachine(nil)
    sm.SetRedisClient(nil)
}

func TestSubAgentWorker_Start(t *testing.T) {
    db := setupTestDBForSubAgentWorker(t)
	defer db.Close()

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}

	worker := NewSubAgentWorker(db, sm, spawner)

    ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)
    time.Sleep(10 * time.Millisecond)
    cancel()
}

func TestPostgresTaskStore_EnqueueSubAgentTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)

	mock.ExpectExec("INSERT INTO sub_agent_queue").WithArgs(sqlmock.AnyArg(), "task-1", "payload1").WillReturnResult(sqlmock.NewResult(1, 1))

	err = store.EnqueueSubAgentTask(context.Background(), "task-1", "payload1")
	assert.NoError(t, err)
}

func TestSqliteTaskStore_ReportMissionHandover(t *testing.T) {
    db := setupTestDB(t) // Assuming this gives an sqlite db
	defer db.Close()

    _, err := db.Exec("CREATE TABLE IF NOT EXISTS agent_missions (id TEXT, status TEXT, mission_log TEXT)")
    require.NoError(t, err)

	store := NewSqliteTaskStore(db)
	err = store.ReportMissionHandover(context.Background(), "m1", "blocked")
	assert.NoError(t, err)
}

func TestPostgresTaskStore_ReportMissionHandover(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)

    mock.ExpectExec("UPDATE agent_missions").WithArgs("blocked", "m1").WillReturnResult(sqlmock.NewResult(1, 1))

	err = store.ReportMissionHandover(context.Background(), "m1", "blocked")
	assert.NoError(t, err)
}

func TestTaskStateMachine_ProcessEvent_SubTaskCompleted_ZeroIncomplete(t *testing.T) {
	db, mock, err := sqlmock.New()
    assert.NoError(t, err)

	sm := NewTaskStateMachine(db)

    mock.ExpectQuery("SELECT sqlite_version()").WillReturnError(assert.AnError)

    mock.ExpectBegin()
    rows1 := sqlmock.NewRows([]string{"status", "workflow_state"}).AddRow("EXECUTING", nil)
    mock.ExpectQuery("SELECT status, workflow_state FROM ohc_tasks").WillReturnRows(rows1)

    rows2 := sqlmock.NewRows([]string{"COUNT(*)"}).AddRow(0)
    mock.ExpectQuery("SELECT COUNT(.+) FROM ohc_tasks").WillReturnRows(rows2)

    mock.ExpectExec("UPDATE ohc_tasks SET status = 'VERIFYING'").WillReturnResult(sqlmock.NewResult(1, 1))

    mock.ExpectCommit()

	ctx := context.Background()
	err = sm.ProcessEvent(ctx, "task-1", EventSubTaskCompleted)
	assert.NoError(t, err)
}
