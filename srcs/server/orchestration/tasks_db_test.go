package orchestration

import (
	"context"
	"database/sql"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
)

func TestTaskDB_ClaimTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	taskDB := NewTaskDB(db, true) // PostgreSQL mode

	// Mock BeginTx
	mock.ExpectBegin()

	// Mock the SELECT FOR UPDATE SKIP LOCKED
	rows := sqlmock.NewRows([]string{"id", "organization_id", "title", "status", "dependencies"}).
		AddRow("task-123", "org-456", "Test Task", "PENDING", []byte(`[]`))

	mock.ExpectQuery(`SELECT id, organization_id, title, status, dependencies FROM shared_tasks WHERE status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED`).
		WillReturnRows(rows)

	// Mock the UPDATE
	mock.ExpectExec(`UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = \$1`).
		WithArgs("task-123").
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Mock Commit
	mock.ExpectCommit()

	ctx := context.Background()
	task, err := taskDB.ClaimTask(ctx, "agent-789")
	if err != nil {
		t.Errorf("error was not expected while claiming task: %s", err)
	}
	if task == nil {
		t.Fatal("expected task, got nil")
	}
	if task.ID != "task-123" {
		t.Errorf("expected task ID task-123, got %s", task.ID)
	}

	// we make sure that all expectations were met
	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDB_ClaimTask_Sqlite(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	taskDB := NewTaskDB(db, false) // SQLite mode

	// Mock BeginTx
	mock.ExpectBegin()

	// Mock the standard SELECT
	rows := sqlmock.NewRows([]string{"id", "organization_id", "title", "status", "dependencies"}).
		AddRow("task-123", "org-456", "Test Task", "PENDING", []byte(`[]`))

	mock.ExpectQuery(`SELECT id, organization_id, title, status, dependencies FROM shared_tasks WHERE status = 'PENDING' LIMIT 1`).
		WillReturnRows(rows)

	// Mock the standard UPDATE (without $1)
	// sqlmock expects exact ? strings to be matched using regexp, but we just pass exact regex.
	mock.ExpectExec(`UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = \?`).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Mock Commit
	mock.ExpectCommit()

	ctx := context.Background()
	task, err := taskDB.ClaimTask(ctx, "agent-789")
	if err != nil {
		t.Errorf("error was not expected while claiming task: %s", err)
	}
	if task == nil {
		t.Fatal("expected task, got nil")
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDB_ClaimTask_NoRows(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	taskDB := NewTaskDB(db, true)

	// Mock BeginTx
	mock.ExpectBegin()

	// Mock SELECT returning no rows
	mock.ExpectQuery(`SELECT id, organization_id, title, status, dependencies FROM shared_tasks WHERE status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED`).
		WillReturnError(sql.ErrNoRows)

	// Rollback is deferred
	mock.ExpectRollback()

	ctx := context.Background()
	task, err := taskDB.ClaimTask(ctx, "agent-789")

	if err != nil {
		t.Errorf("expected nil error on ErrNoRows, got %s", err)
	}
	if task != nil {
		t.Errorf("expected nil task, got %v", task)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}
