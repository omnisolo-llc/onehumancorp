package orchestration

import (
	"context"
	"testing"
    "database/sql"
    "github.com/DATA-DOG/go-sqlmock"
)

func TestClaimTaskFallback(t *testing.T) {
	orchestrator := NewSharedTaskOrchestrator(nil)

	err := orchestrator.ClaimTask(context.Background(), "org1", "agent1")
	if err != nil {
		t.Fatalf("Expected no error for standalone fallback, got %v", err)
	}
}

func TestVerifyTeammateMesh(t *testing.T) {
	res := VerifyTeammateMesh(nil)
	if res != false {
		t.Fatalf("Expected false when redisClient is nil")
	}

	res2 := VerifyTeammateMesh(&struct{}{})
	if res2 != true {
		t.Fatalf("Expected true when redisClient is not nil")
	}
}

func TestVerifyAutoDream(t *testing.T) {
	res := VerifyAutoDream(nil)
	if res != false {
		t.Fatalf("Expected false when db is nil")
	}

    db, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer db.Close()

    mock.ExpectQuery("SELECT count(.*) FROM autodream_memories").WillReturnRows(sqlmock.NewRows([]string{"count"}).AddRow(1))

    res2 := VerifyAutoDream(db)
    if res2 != true {
        t.Fatalf("Expected true when db query succeeds")
    }

    db2, mock2, _ := sqlmock.New()
    mock2.ExpectQuery("SELECT count(.*) FROM autodream_memories").WillReturnError(sql.ErrNoRows)
    res3 := VerifyAutoDream(db2)
    if res3 != false {
        t.Fatalf("Expected false when db query fails")
    }
}

func TestClaimTaskWithDb(t *testing.T) {
    db, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer db.Close()

    orchestrator := NewSharedTaskOrchestrator(db)

    mock.ExpectBegin()
    mock.ExpectQuery("SELECT id FROM shared_tasks_v4").WillReturnRows(sqlmock.NewRows([]string{"id"}).AddRow("task1"))
    mock.ExpectExec("UPDATE shared_tasks_v4").WithArgs("agent1", sqlmock.AnyArg(), "task1").WillReturnResult(sqlmock.NewResult(1, 1))
    mock.ExpectCommit()

    err = orchestrator.ClaimTask(context.Background(), "org1", "agent1")
    if err != nil {
        t.Fatalf("Expected no error, got %v", err)
    }

    // No rows
    mock.ExpectBegin()
    mock.ExpectQuery("SELECT id FROM shared_tasks_v4").WillReturnError(sql.ErrNoRows)
    mock.ExpectRollback()

    err = orchestrator.ClaimTask(context.Background(), "org1", "agent1")
    if err != nil {
        t.Fatalf("Expected no error when no rows, got %v", err)
    }

    // Begin error
    db2, mock2, _ := sqlmock.New()
    orchestrator2 := NewSharedTaskOrchestrator(db2)
    mock2.ExpectBegin().WillReturnError(sql.ErrConnDone)
    err = orchestrator2.ClaimTask(context.Background(), "org1", "agent1")
    if err == nil {
        t.Fatalf("Expected error when begin fails")
    }

    // Exec error
    mock.ExpectBegin()
    mock.ExpectQuery("SELECT id FROM shared_tasks_v4").WillReturnRows(sqlmock.NewRows([]string{"id"}).AddRow("task1"))
    mock.ExpectExec("UPDATE shared_tasks_v4").WillReturnError(sql.ErrConnDone)
    mock.ExpectRollback()
    err = orchestrator.ClaimTask(context.Background(), "org1", "agent1")
    if err == nil {
        t.Fatalf("Expected error when exec fails")
    }

    // query error other than no rows
    mock.ExpectBegin()
    mock.ExpectQuery("SELECT id FROM shared_tasks_v4").WillReturnError(sql.ErrConnDone)
    mock.ExpectRollback()
    err = orchestrator.ClaimTask(context.Background(), "org1", "agent1")
    if err == nil {
        t.Fatalf("Expected error when query fails")
    }
}
