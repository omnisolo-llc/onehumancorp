package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupQueueTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return db
}

func TestSqliteTaskQueue_EnqueueDequeueAcknowledge(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewSqliteTaskQueue(db)
	ctx := context.Background()

	task := &SubAgentTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		ParentTaskID:   "parent-1",
		Payload:        json.RawMessage(`{"agent_id": "agent-1", "command": "echo 'hello'"}`),
		Status:         "PENDING",
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

	err := queue.Enqueue(ctx, task)
	assert.NoError(t, err)

	dequeuedTask, err := queue.Dequeue(ctx, "worker-1")
	assert.NoError(t, err)
	require.NotNil(t, dequeuedTask)
	assert.Equal(t, "task-1", dequeuedTask.ID)
	assert.Equal(t, "IN_PROGRESS", dequeuedTask.Status)
	assert.Equal(t, "worker-1", *dequeuedTask.WorkerID)

	err = queue.Acknowledge(ctx, "task-1", "COMPLETED")
	assert.NoError(t, err)

	// Verify status
	var status string
	err = db.QueryRow("SELECT status FROM sub_agent_queue WHERE id = 'task-1'").Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

func TestSqliteTaskQueue_DequeueEmpty(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	queue := NewSqliteTaskQueue(db)
	ctx := context.Background()

	task, err := queue.Dequeue(ctx, "worker-1")
	assert.NoError(t, err)
	assert.Nil(t, task)
}

func TestPostgresTaskQueue_Enqueue(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	require.NoError(t, err)
	defer db.Close()

	queue := NewPostgresTaskQueue(db)
	ctx := context.Background()

	task := &SubAgentTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		ParentTaskID:   "parent-1",
		Payload:        json.RawMessage(`{"agent_id": "agent-1", "command": "echo 'hello'"}`),
		Status:         "PENDING",
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	mock.ExpectExec(query).WithArgs(task.ID, task.OrganizationID, task.ParentTaskID, task.Payload, task.Status, task.WorkerID, task.CreatedAt, task.UpdatedAt).WillReturnResult(sqlmock.NewResult(1, 1))

	err = queue.Enqueue(ctx, task)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPostgresTaskQueue_Dequeue(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	require.NoError(t, err)
	defer db.Close()

	queue := NewPostgresTaskQueue(db)
	ctx := context.Background()

	mock.ExpectBegin()
	query := `
		SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		FROM sub_agent_queue
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	createdAt := time.Now()
	updatedAt := time.Now()
	rows := sqlmock.NewRows([]string{"id", "organization_id", "parent_task_id", "payload", "status", "worker_id", "created_at", "updated_at"}).
		AddRow("task-1", "org-1", "parent-1", []byte(`{"agent_id": "agent-1"}`), "PENDING", nil, createdAt, updatedAt)
	mock.ExpectQuery(query).WillReturnRows(rows)

	updateQuery := `
		UPDATE sub_agent_queue
		SET status = 'IN_PROGRESS', worker_id = $1, updated_at = NOW()
		WHERE id = $2
	`
	mock.ExpectExec(updateQuery).WithArgs("worker-1", "task-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	task, err := queue.Dequeue(ctx, "worker-1")
	assert.NoError(t, err)
	require.NotNil(t, task)
	assert.Equal(t, "task-1", task.ID)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPostgresTaskQueue_Acknowledge(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	require.NoError(t, err)
	defer db.Close()

	queue := NewPostgresTaskQueue(db)
	ctx := context.Background()

	query := `
		UPDATE sub_agent_queue
		SET status = $1, updated_at = NOW()
		WHERE id = $2
	`
	mock.ExpectExec(query).WithArgs("COMPLETED", "task-1").WillReturnResult(sqlmock.NewResult(1, 1))

	err = queue.Acknowledge(ctx, "task-1", "COMPLETED")
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPostgresTaskQueue_DequeueEmpty(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	require.NoError(t, err)
	defer db.Close()

	queue := NewPostgresTaskQueue(db)
	ctx := context.Background()

	mock.ExpectBegin()
	query := `
		SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		FROM sub_agent_queue
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	mock.ExpectQuery(query).WillReturnError(sql.ErrNoRows)
	mock.ExpectRollback()

	task, err := queue.Dequeue(ctx, "worker-1")
	assert.NoError(t, err)
	assert.Nil(t, task)
	assert.NoError(t, mock.ExpectationsWereMet())
}
