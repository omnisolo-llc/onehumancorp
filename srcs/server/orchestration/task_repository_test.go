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

func setupTaskTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS tasks (
			id TEXT PRIMARY KEY,
			epic_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			payload TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			locked_by TEXT,
			locked_at DATETIME
		);
	`)
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id),
			FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
			FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id) ON DELETE CASCADE
		);
	`)
	require.NoError(t, err)

	return db
}

func TestTaskRepository_Create(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()

	repo := NewTaskRepository(db, false)
	ctx := context.Background()

	payload := json.RawMessage(`{"key": "value"}`)
	task := &Task{
		ID:      "task-1",
		Title:   "Test Task",
		Payload: &payload,
	}

	err := repo.Create(ctx, task)
	assert.NoError(t, err)

	var title, status string
	var pl []byte
	err = db.QueryRow("SELECT title, status, payload FROM tasks WHERE id = 'task-1'").Scan(&title, &status, &pl)
	assert.NoError(t, err)
	assert.Equal(t, "Test Task", title)
	assert.Equal(t, "PENDING", status)
	assert.Equal(t, `{"key": "value"}`, string(pl))
}

func TestTaskRepository_UpdateStatus(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()

	repo := NewTaskRepository(db, false)
	ctx := context.Background()

	task := &Task{
		ID:    "task-update",
		Title: "Update Task",
	}
	err := repo.Create(ctx, task)
	assert.NoError(t, err)

	err = repo.UpdateStatus(ctx, "task-update", "COMPLETED")
	assert.NoError(t, err)

	var status string
	err = db.QueryRow("SELECT status FROM tasks WHERE id = 'task-update'").Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

func TestTaskRepository_GetNextAvailableTask(t *testing.T) {
	db := setupTaskTestDB(t)
	defer db.Close()

	repo := NewTaskRepository(db, false)
	ctx := context.Background()

	// Task 1: pending, no dependencies
	task1 := &Task{ID: "task-1", Title: "Task 1"}
	err := repo.Create(ctx, task1)
	assert.NoError(t, err)

	// Task 2: pending, depends on Task 1
	task2 := &Task{ID: "task-2", Title: "Task 2"}
	err = repo.Create(ctx, task2)
	assert.NoError(t, err)
	_, err = db.Exec("INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ('task-2', 'task-1')")
	assert.NoError(t, err)

	// Fetch next task - should be task-1
	fetchedTask, err := repo.GetNextAvailableTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.NotNil(t, fetchedTask)
	assert.Equal(t, "task-1", fetchedTask.ID)
	assert.Equal(t, "IN_PROGRESS", fetchedTask.Status)
	assert.Equal(t, "worker-1", *fetchedTask.LockedBy)
	assert.NotNil(t, fetchedTask.LockedAt)

	// Fetch next task - none available because task-2 depends on task-1 which is IN_PROGRESS
	fetchedTask2, err := repo.GetNextAvailableTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.Nil(t, fetchedTask2)

	// Complete task-1
	err = repo.UpdateStatus(ctx, "task-1", "COMPLETED")
	assert.NoError(t, err)

	// Fetch next task - should be task-2 now
	fetchedTask3, err := repo.GetNextAvailableTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.NotNil(t, fetchedTask3)
	assert.Equal(t, "task-2", fetchedTask3.ID)
	assert.Equal(t, "IN_PROGRESS", fetchedTask3.Status)
}

// PostgreSQL Tests using sqlmock
func TestTaskRepository_Postgres_Create(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	repo := NewTaskRepository(db, true)
	ctx := context.Background()

	payload := json.RawMessage(`{"key": "value"}`)
	task := &Task{
		ID:      "task-1",
		Title:   "Test Task",
		Payload: &payload,
	}

	mock.ExpectExec("INSERT INTO tasks").
		WithArgs("task-1", nil, "Test Task", "PENDING", []byte(`{"key": "value"}`)).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.Create(ctx, task)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestTaskRepository_Postgres_UpdateStatus(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	repo := NewTaskRepository(db, true)
	ctx := context.Background()

	mock.ExpectExec("UPDATE tasks SET status = \\$1").
		WithArgs("COMPLETED", "task-update").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.UpdateStatus(ctx, "task-update", "COMPLETED")
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestTaskRepository_Postgres_GetNextAvailableTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	repo := NewTaskRepository(db, true)
	ctx := context.Background()

	mock.ExpectBegin()

	tNow := time.Now()
	rows := sqlmock.NewRows([]string{"id", "epic_id", "title", "status", "payload", "created_at", "updated_at", "locked_by", "locked_at"}).
		AddRow("task-1", nil, "Task 1", "PENDING", nil, tNow, tNow, nil, nil)

	mock.ExpectQuery("SELECT id, epic_id, title, status, payload, created_at, updated_at, locked_by, locked_at FROM tasks").
		WillReturnRows(rows)

	mock.ExpectExec("UPDATE tasks SET status = \\$1").
		WithArgs("IN_PROGRESS", "worker-1", "task-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectCommit()

	fetchedTask, err := repo.GetNextAvailableTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.NotNil(t, fetchedTask)
	assert.Equal(t, "task-1", fetchedTask.ID)
	assert.Equal(t, "IN_PROGRESS", fetchedTask.Status)
	assert.Equal(t, "worker-1", *fetchedTask.LockedBy)
	assert.NotNil(t, fetchedTask.LockedAt)

	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestTaskRepository_Postgres_GetNextAvailableTask_NoTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	repo := NewTaskRepository(db, true)
	ctx := context.Background()

	mock.ExpectBegin()

	// Simulate no rows returned
	mock.ExpectQuery("SELECT id, epic_id, title, status, payload, created_at, updated_at, locked_by, locked_at FROM tasks").
		WillReturnError(sql.ErrNoRows)

	mock.ExpectRollback()

	fetchedTask, err := repo.GetNextAvailableTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.Nil(t, fetchedTask)

	assert.NoError(t, mock.ExpectationsWereMet())
}
