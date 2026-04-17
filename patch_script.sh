#!/bin/bash

# Restore the correct version
cat << 'MODELS' >> srcs/server/orchestration/models.go

type OrchestrationTask struct {
	ID        string     \`json:"id"\`
	EpicID    *string    \`json:"epic_id,omitempty"\`
	Title     string     \`json:"title"\`
	Status    string     \`json:"status"\`
	Payload   *string    \`json:"payload,omitempty"\`
	LockedBy  *string    \`json:"locked_by,omitempty"\`
	LockedAt  *time.Time \`json:"locked_at,omitempty"\`
	CreatedAt time.Time  \`json:"created_at"\`
	UpdatedAt time.Time  \`json:"updated_at"\`
}

type TaskDependency struct {
	TaskID          string \`json:"task_id"\`
	DependsOnTaskID string \`json:"depends_on_task_id"\`
}
MODELS

cat << 'MIG' > srcs/server/db/migrations/20260416060000_create_tasks_tables.sql
-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY,
    epic_id UUID,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT,
    locked_by UUID,
    locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id) ON DELETE CASCADE
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS tasks;
-- +goose StatementEnd
MIG

cat << 'REPO' > srcs/server/orchestration/tasks_repo.go
package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type TasksRepository struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewTasksRepository(dbProvider db.Provider) *TasksRepository {
	return &TasksRepository{
		dbProvider: dbProvider,
	}
}

func (r *TasksRepository) CreateTask(ctx context.Context, task *OrchestrationTask) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, \`
		INSERT INTO tasks (id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at)
		VALUES (\$1, \$2, \$3, \$4, \$5, \$6, \$7, \$8, \$9)
	\`, task.ID, task.EpicID, task.Title, task.Status, task.Payload, task.LockedBy, task.LockedAt, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *TasksRepository) UpdateTaskStatus(ctx context.Context, id, status string) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "UPDATE tasks SET status = \$1, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", status, id)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *TasksRepository) GetNextAvailableTask(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	if r.dbProvider.IsSQLite() {
		return r.getNextAvailableTaskSQLite(ctx, agentID)
	}
	return r.getNextAvailableTaskPostgres(ctx, agentID)
}

func (r *TasksRepository) getNextAvailableTaskSQLite(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task OrchestrationTask
	row := tx.QueryRow(ctx, "SELECT id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at FROM tasks WHERE status = 'PENDING' AND locked_by IS NULL LIMIT 1")
	if err := row.Scan(&task.ID, &task.EpicID, &task.Title, &task.Status, &task.Payload, &task.LockedBy, &task.LockedAt, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	now := time.Now()
	_, err = tx.Exec(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', locked_by = \$1, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.LockedBy = &agentID
	task.LockedAt = &now
	return &task, nil
}

func (r *TasksRepository) getNextAvailableTaskPostgres(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task OrchestrationTask
	row := tx.QueryRow(ctx, "SELECT id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at FROM tasks WHERE status = 'PENDING' AND locked_by IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED")
	if err := row.Scan(&task.ID, &task.EpicID, &task.Title, &task.Status, &task.Payload, &task.LockedBy, &task.LockedAt, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	now := time.Now()
	_, err = tx.Exec(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', locked_by = \$1, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.LockedBy = &agentID
	task.LockedAt = &now
	return &task, nil
}
REPO

cat << 'TESTS' > srcs/server/orchestration/tasks_repo_test.go
package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type mockProvider struct {
	isSQLite bool
	beginErr error
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.beginErr != nil {
		return nil, m.beginErr
	}
	return &mockTx{}, nil
}
func (m *mockProvider) Close() {}
func (m *mockProvider) Ping(ctx context.Context) error { return nil }
func (m *mockProvider) IsSQLite() bool { return m.isSQLite }
func (m *mockProvider) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) { return nil, nil }

type mockTx struct {
	execErr error
	commitErr error
	queryRow db.Row
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, m.execErr }
func (m *mockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *mockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return m.queryRow }
func (m *mockTx) Commit(ctx context.Context) error { return m.commitErr }
func (m *mockTx) Rollback(ctx context.Context) error { return nil }

type mockRow struct {
	scanErr error
}

func (m *mockRow) Scan(dest ...any) error { return m.scanErr }

func setupTasksTestDB(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}
	t.Cleanup(func() { sqlDB.Close() })

	_, err = sqlDB.Exec(\`
		CREATE TABLE tasks (
			id TEXT PRIMARY KEY,
			epic_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL,
			payload TEXT,
			locked_by TEXT,
			locked_at TIMESTAMP,
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
	\`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqlDB)
}

func TestTasksRepository_CreateTask(t *testing.T) {
	provider := setupTasksTestDB(t)
	repo := NewTasksRepository(provider)
	ctx := context.Background()

	task := &OrchestrationTask{
		ID:        "task-1",
		Title:     "Test Task",
		Status:    "PENDING",
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}

	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}
}

func TestTasksRepository_UpdateTaskStatus(t *testing.T) {
	provider := setupTasksTestDB(t)
	repo := NewTasksRepository(provider)
	ctx := context.Background()

	task := &OrchestrationTask{
		ID:        "task-2",
		Title:     "Test Task 2",
		Status:    "PENDING",
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}
	_ = repo.CreateTask(ctx, task)

	err := repo.UpdateTaskStatus(ctx, "task-2", "COMPLETED")
	if err != nil {
		t.Fatalf("UpdateTaskStatus failed: %v", err)
	}
}

func TestTasksRepository_GetNextAvailableTaskSQLite(t *testing.T) {
	provider := setupTasksTestDB(t)
	repo := NewTasksRepository(provider)
	ctx := context.Background()

	task := &OrchestrationTask{
		ID:        "task-3",
		Title:     "Test Task 3",
		Status:    "PENDING",
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}
	_ = repo.CreateTask(ctx, task)

	agentID := "agent-1"
	nextTask, err := repo.GetNextAvailableTask(ctx, agentID)
	if err != nil {
		t.Fatalf("GetNextAvailableTask failed: %v", err)
	}
	if nextTask == nil {
		t.Fatalf("expected a task, got nil")
	}
	if nextTask.ID != "task-3" {
		t.Errorf("expected task-3, got %s", nextTask.ID)
	}
	if nextTask.Status != "IN_PROGRESS" {
		t.Errorf("expected IN_PROGRESS, got %s", nextTask.Status)
	}
	if *nextTask.LockedBy != agentID {
		t.Errorf("expected %s, got %s", agentID, *nextTask.LockedBy)
	}

	nextTask2, err := repo.GetNextAvailableTask(ctx, agentID)
	if err != nil {
		t.Fatalf("GetNextAvailableTask failed: %v", err)
	}
	if nextTask2 != nil {
		t.Fatalf("expected no task, got %v", nextTask2.ID)
	}
}

func TestTasksRepository_MockErrors(t *testing.T) {
	ctx := context.Background()
	task := &OrchestrationTask{ID: "t1", Title: "Title", Status: "PENDING"}

	t.Run("CreateTask Begin Error", func(t *testing.T) {
		repo := NewTasksRepository(&mockProvider{beginErr: errors.New("begin error")})
		if err := repo.CreateTask(ctx, task); err == nil {
			t.Fatal("expected error")
		}
	})

	t.Run("UpdateTaskStatus Begin Error", func(t *testing.T) {
		repo := NewTasksRepository(&mockProvider{beginErr: errors.New("begin error")})
		if err := repo.UpdateTaskStatus(ctx, "id", "st"); err == nil {
			t.Fatal("expected error")
		}
	})

	t.Run("GetNextAvailableTask SQLite Begin Error", func(t *testing.T) {
		repo := NewTasksRepository(&mockProvider{isSQLite: true, beginErr: errors.New("begin error")})
		if _, err := repo.GetNextAvailableTask(ctx, "agent"); err == nil {
			t.Fatal("expected error")
		}
	})

	t.Run("GetNextAvailableTask Postgres Begin Error", func(t *testing.T) {
		repo := NewTasksRepository(&mockProvider{isSQLite: false, beginErr: errors.New("begin error")})
		if _, err := repo.GetNextAvailableTask(ctx, "agent"); err == nil {
			t.Fatal("expected error")
		}
	})
}
TESTS
