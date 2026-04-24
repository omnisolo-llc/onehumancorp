package repository

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"errors"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}

	schema := `
	CREATE TABLE tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_role TEXT,
		created_at DATETIME,
		updated_at DATETIME
	);
	CREATE TABLE task_dependencies (
		task_id TEXT NOT NULL,
		depends_on_task_id TEXT NOT NULL,
		PRIMARY KEY (task_id, depends_on_task_id),
		FOREIGN KEY (task_id) REFERENCES tasks(id),
		FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id)
	);
	`
	_, err = db.Exec(schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return db
}

func TestTaskRepository_CreateTask(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctx := context.WithValue(context.Background(), orgIDKey, "org-123")

	task := &Task{
		Title:             "Test Task",
		Description:       "Test Description",
		Status:            "PENDING",
		AssignedAgentRole: "tester",
	}

	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	if task.ID == "" {
		t.Errorf("expected task ID to be set")
	}
	if task.OrganizationID != "org-123" {
		t.Errorf("expected organization_id to be org-123, got %s", task.OrganizationID)
	}
	if task.CreatedAt.IsZero() {
		t.Errorf("expected created_at to be set")
	}
	if task.UpdatedAt.IsZero() {
		t.Errorf("expected updated_at to be set")
	}

	// Test missing organization ID in context
	ctxNoOrg := context.Background()
	err = repo.CreateTask(ctxNoOrg, &Task{Title: "Should Fail"})
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}
}

func TestTaskRepository_GetTasksByOrg(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctxOrg1 := context.WithValue(context.Background(), orgIDKey, "org-1")
	ctxOrg2 := context.WithValue(context.Background(), orgIDKey, "org-2")

	// Create tasks for org-1
	_ = repo.CreateTask(ctxOrg1, &Task{Title: "Task 1"})
	_ = repo.CreateTask(ctxOrg1, &Task{Title: "Task 2"})

	// Create tasks for org-2
	_ = repo.CreateTask(ctxOrg2, &Task{Title: "Task 3"})

	// Test getting tasks for org-1
	tasksOrg1, err := repo.GetTasksByOrg(ctxOrg1)
	if err != nil {
		t.Fatalf("failed to get tasks: %v", err)
	}
	if len(tasksOrg1) != 2 {
		t.Errorf("expected 2 tasks for org-1, got %d", len(tasksOrg1))
	}

	// Test getting tasks for org-2
	tasksOrg2, err := repo.GetTasksByOrg(ctxOrg2)
	if err != nil {
		t.Fatalf("failed to get tasks: %v", err)
	}
	if len(tasksOrg2) != 1 {
		t.Errorf("expected 1 task for org-2, got %d", len(tasksOrg2))
	}
	if tasksOrg2[0].Title != "Task 3" {
		t.Errorf("expected task title 'Task 3', got %s", tasksOrg2[0].Title)
	}

	// Test missing organization ID
	_, err = repo.GetTasksByOrg(context.Background())
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}

    // Drop table to test error handling
    db.Exec("DROP TABLE tasks;")
    _, err = repo.GetTasksByOrg(ctxOrg1)
    if err == nil {
        t.Errorf("expected error after dropping table")
    }
}

func TestTaskRepository_UpdateTaskStatus(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctx := context.WithValue(context.Background(), orgIDKey, "org-123")

	task := &Task{
		Title: "Test Task",
	}
	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	time.Sleep(10 * time.Millisecond) // Ensure updated_at changes

	err = repo.UpdateTaskStatus(ctx, task.ID, "IN_PROGRESS")
	if err != nil {
		t.Fatalf("failed to update task status: %v", err)
	}

	// Verify the update
	tasks, _ := repo.GetTasksByOrg(ctx)
	if tasks[0].Status != "IN_PROGRESS" {
		t.Errorf("expected status 'IN_PROGRESS', got %s", tasks[0].Status)
	}
	if !tasks[0].UpdatedAt.After(tasks[0].CreatedAt) {
		t.Errorf("expected updated_at (%v) to be after created_at (%v)", tasks[0].UpdatedAt, tasks[0].CreatedAt)
	}

	// Test updating task for different org (should fail)
	ctxOtherOrg := context.WithValue(context.Background(), orgIDKey, "org-other")
	err = repo.UpdateTaskStatus(ctxOtherOrg, task.ID, "DONE")
	if err == nil {
		t.Errorf("expected error when updating task owned by different organization")
	}

	// Test missing organization ID
	err = repo.UpdateTaskStatus(context.Background(), task.ID, "DONE")
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}

    // Test update with nonexistent task (RowsAffected == 0)
    err = repo.UpdateTaskStatus(ctx, "nonexistent-id", "DONE")
    if err == nil {
        t.Errorf("expected error when updating nonexistent task")
    }
    if err.Error() != "task not found or not owned by organization" {
        t.Errorf("expected specific error, got %v", err)
    }

    // Drop table to test error handling
    db.Exec("DROP TABLE tasks;")
    err = repo.UpdateTaskStatus(ctx, task.ID, "DONE")
    if err == nil {
        t.Errorf("expected error after dropping table")
    }
}

func TestTaskRepository_GetTasksByOrg_RowsError(t *testing.T) {
    db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctxOrg1 := context.WithValue(context.Background(), orgIDKey, "org-1")

    // Test with closed DB
    db.Close()

    err := repo.UpdateTaskStatus(ctxOrg1, "id", "status")
    if err == nil {
        t.Errorf("expected error when updating with closed db")
    }

    task := &Task{
		Title:             "Test Task",
		Description:       "Test Description",
		Status:            "PENDING",
		AssignedAgentRole: "tester",
	}
    err = repo.CreateTask(ctxOrg1, task)
    if err == nil {
        t.Errorf("expected error when creating with closed db")
    }
}

// A custom driver to test rows.Err(), res.RowsAffected() and Scan() errors

type mockDriver struct{}

func (d *mockDriver) Open(name string) (driver.Conn, error) {
	return &mockConn{}, nil
}

type mockConn struct{}
func (c *mockConn) Prepare(query string) (driver.Stmt, error) { return &mockStmt{}, nil }
func (c *mockConn) Close() error                              { return nil }
func (c *mockConn) Begin() (driver.Tx, error)                 { return nil, nil }

type mockStmt struct{}
func (s *mockStmt) Close() error                                    { return nil }
func (s *mockStmt) NumInput() int                                   { return -1 }
func (s *mockStmt) Exec(args []driver.Value) (driver.Result, error) { return &mockResult{}, nil }
func (s *mockStmt) Query(args []driver.Value) (driver.Rows, error)  {
    if len(args) > 0 && args[0] == "scan_error_org" {
        return &mockRows{failScan: true}, nil
    }
    if len(args) > 0 && args[0] == "rows_error_org" {
        return &mockRows{failNext: true}, nil
    }
    return &mockRows{}, nil
}

type mockResult struct{}
func (r *mockResult) LastInsertId() (int64, error) { return 0, nil }
func (r *mockResult) RowsAffected() (int64, error) { return 0, errors.New("mock rows affected error") }

type mockRows struct{
    failScan bool
    failNext bool
    count int
}
func (r *mockRows) Columns() []string              {
    return []string{"id", "organization_id", "parent_task_id", "title", "description", "status", "assigned_agent_role", "created_at", "updated_at"}
}
func (r *mockRows) Close() error                   { return nil }
func (r *mockRows) Next(dest []driver.Value) error {
    if r.count > 0 {
        if r.failNext {
            return errors.New("mock rows error")
        }
        return driver.ErrBadConn // End of rows or error
    }
    r.count++

    if r.failScan {
        // Return incompatible type to fail scan
        dest[0] = nil
    } else {
        dest[0] = "id"
    }
    for i := 1; i < 7; i++ {
        dest[i] = "dummy"
    }

    // valid dates to avoid Scan error
    now := time.Now()
    dest[7] = now
    dest[8] = now
    return nil
}

func init() {
	sql.Register("mockDriver", &mockDriver{})
}

func TestTaskRepository_MockErrors(t *testing.T) {
	db, _ := sql.Open("mockDriver", "mock")
	repo := NewTaskRepository(db)

    // Test rows.Err()
    ctx := context.WithValue(context.Background(), orgIDKey, "rows_error_org")
	_, err := repo.GetTasksByOrg(ctx)
	if err == nil || err.Error() != "mock rows error" {
		t.Errorf("expected mock rows error, got %v", err)
	}

    // Test scan error
    ctxScanError := context.WithValue(context.Background(), orgIDKey, "scan_error_org")
    _, err = repo.GetTasksByOrg(ctxScanError)
    if err == nil {
		t.Errorf("expected scan error, got nil")
	}

    // Test update task status error
	err = repo.UpdateTaskStatus(ctx, "id", "status")
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}
