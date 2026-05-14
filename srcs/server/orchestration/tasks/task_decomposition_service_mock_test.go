package tasks

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
)

func TestTaskDecompositionService_RowsErrWithMock(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	rows := sqlmock.NewRows([]string{"id", "dependencies"}).
		AddRow("task-1", "[]").
		RowError(0, errors.New("rows error"))

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-rows-err").
		WillReturnRows(rows)
	mock.ExpectRollback()

	_, err = svc.ClaimTask(ctx, "m-rows-err", "agent")
	if err == nil || err.Error() != "rows error" {
		t.Fatalf("expected rows error, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_QueryErrorDeps(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	deps, _ := json.Marshal([]string{"dep-1"})
	rows := sqlmock.NewRows([]string{"id", "dependencies"}).
		AddRow("task-1", deps)

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-deps-err").
		WillReturnRows(rows)

	mock.ExpectQuery("^SELECT status FROM swarm_tasks WHERE id = \\$1$").
		WithArgs("dep-1").
		WillReturnError(errors.New("db err"))

	mock.ExpectRollback()

	_, err = svc.ClaimTask(ctx, "m-deps-err", "agent")
	if err == nil || err.Error() != "db err" {
		t.Fatalf("expected db err, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_CommitError(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	rows := sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("task-1", "[]")

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-commit-err").
		WillReturnRows(rows)

	mock.ExpectExec("^UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = \\$1 WHERE id = \\$2$").
		WithArgs("agent", "task-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("^INSERT INTO state_machine_transitions").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectCommit().WillReturnError(errors.New("commit err"))

	_, err = svc.ClaimTask(ctx, "m-commit-err", "agent")
	if err == nil || err.Error() != "commit err" {
		t.Fatalf("expected commit err, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_UpdateExecErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectQuery("^SELECT status FROM swarm_tasks WHERE id = \\$1$").
		WithArgs("task-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))

	mock.ExpectExec("^UPDATE swarm_tasks SET status = \\$1 WHERE id = \\$2$").
		WithArgs("COMPLETED", "task-1").
		WillReturnError(errors.New("update err"))

	mock.ExpectRollback()

	err = svc.UpdateTaskStatus(ctx, "task-1", "COMPLETED", "agent", "reason")
	if err == nil || err.Error() != "update err" {
		t.Fatalf("expected update err, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_ClaimUpdateExecErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	rows := sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("task-1", "[]")

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-claim-upd").
		WillReturnRows(rows)

	mock.ExpectExec("^UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = \\$1 WHERE id = \\$2$").
		WithArgs("agent", "task-1").
		WillReturnError(errors.New("claim update err"))

	mock.ExpectRollback()

	_, err = svc.ClaimTask(ctx, "m-claim-upd", "agent")
	if err == nil || err.Error() != "claim update err" {
		t.Fatalf("expected claim update err, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_ClaimInsertErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	rows := sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("task-1", "[]")

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-claim-ins").
		WillReturnRows(rows)

	mock.ExpectExec("^UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = \\$1 WHERE id = \\$2$").
		WithArgs("agent", "task-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("^INSERT INTO state_machine_transitions").
		WillReturnError(errors.New("claim insert err"))

	mock.ExpectRollback()

	_, err = svc.ClaimTask(ctx, "m-claim-ins", "agent")
	if err == nil || err.Error() != "claim insert err" {
		t.Fatalf("expected claim insert err, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestTaskDecompositionService_RowsScanErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	svc := NewDBTaskDecompositionService(db, false)
	ctx := context.Background()

	mock.ExpectBegin()
	// Mismatched column count causes scan error
	rows := sqlmock.NewRows([]string{"id"}).AddRow("task-1")

	mock.ExpectQuery("^SELECT id, dependencies FROM swarm_tasks WHERE mission_id = \\$1 AND status = 'PENDING'$").
		WithArgs("m-claim-scan").
		WillReturnRows(rows)

	mock.ExpectRollback()

	_, err = svc.ClaimTask(ctx, "m-claim-scan", "agent")
	if err == nil {
		t.Fatalf("expected scan err, got nil")
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}
