package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/jmoiron/sqlx"
	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sqlx.DB {
	db, err := sqlx.Connect("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to connect to in-memory db: %v", err)
	}

	schema := `
	CREATE TABLE shared_tasks (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL DEFAULT 'PENDING',
		dependencies TEXT NOT NULL DEFAULT '[]',
		assigned_agent_id TEXT,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		payload TEXT,
		title TEXT,
		description TEXT,
		priority TEXT
	);
	`
	_, err = db.Exec(schema)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	return db
}

func TestClaimTask_SQLite(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	scheduler := NewScheduler(db, "sqlite3")
	ctx := context.Background()

	_, err := db.Exec(`INSERT INTO shared_tasks (id, status, dependencies) VALUES ('t1', 'PENDING', '[]')`)
	if err != nil {
		t.Fatal(err)
	}

	task, err := scheduler.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if task == nil || task.ID != "t1" || task.Status != "IN_PROGRESS" || *task.AssignedAgentID != "agent1" {
		t.Errorf("Unexpected task: %v", task)
	}

	task2, err := scheduler.ClaimTask(ctx, "agent2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if task2 != nil {
		t.Fatalf("Expected nil task, got %v", task2)
	}
}

func TestClaimTask_DAGBlocked_SQLite(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	scheduler := NewScheduler(db, "sqlite3")
	ctx := context.Background()

	db.Exec(`INSERT INTO shared_tasks (id, status, dependencies) VALUES ('parent', 'PENDING', '[]')`)
	db.Exec(`INSERT INTO shared_tasks (id, status, dependencies) VALUES ('child', 'PENDING', '["parent"]')`)

	task, err := scheduler.ClaimTask(ctx, "agent1")
	if err != nil || task == nil || task.ID != "parent" {
		t.Fatalf("Expected parent to be claimed")
	}

	task2, err := scheduler.ClaimTask(ctx, "agent2")
	if err != nil || task2 != nil {
		t.Fatalf("Expected child to be blocked")
	}

	db.Exec(`UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = 'parent'`)

	task3, err := scheduler.ClaimTask(ctx, "agent3")
	if err != nil || task3 == nil || task3.ID != "child" {
		t.Fatalf("Expected child to be claimed")
	}
}

func TestCheckDependencies_MissingParent(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	scheduler := NewScheduler(db, "sqlite3")
	ctx := context.Background()

	db.Exec(`INSERT INTO shared_tasks (id, status, dependencies) VALUES ('child', 'PENDING', '["missing"]')`)
	task, _ := scheduler.ClaimTask(ctx, "agent1")
	if task != nil {
		t.Fatalf("Expected child to be blocked")
	}
}

func TestCheckDependencies_InvalidJSON(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	scheduler := NewScheduler(db, "sqlite3")
	ctx := context.Background()

	db.Exec(`INSERT INTO shared_tasks (id, status, dependencies) VALUES ('child', 'PENDING', 'invalid-json')`)
	_, err := scheduler.ClaimTask(ctx, "agent1")
	if err == nil {
		t.Fatalf("Expected error for invalid json, got nil")
	}
}

func TestPool_IsSQLite(t *testing.T) {
	p := Pool{DriverName: "sqlite3"}
	if !p.IsSQLite() {
		t.Fatal("Expected IsSQLite to be true")
	}
	p.DriverName = "postgres"
	if p.IsSQLite() {
		t.Fatal("Expected IsSQLite to be false")
	}
}

func TestClaimTaskPostgres_Mocked(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

	// 1. Success scenario
	mock.ExpectBegin()
	mock.ExpectQuery(`(?s).*SELECT t.id, t.dependencies.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t1", "[]"))
	mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "status", "dependencies"}).AddRow("pg_t1", "IN_PROGRESS", "[]"))
	mock.ExpectCommit()

	task, err := scheduler.ClaimTask(context.Background(), "agent1")
	if err != nil || task == nil || task.ID != "pg_t1" {
		t.Errorf("expected success, got err: %v, task: %v", err, task)
	}

	// 2. No rows scenario
	mock.ExpectBegin()
	mock.ExpectQuery(`.*`).WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}))
	mock.ExpectCommit()

	task, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err != nil || task != nil {
		t.Errorf("expected nil for no rows")
	}

	// 3. Blocked by dependencies scenario
	mock.ExpectBegin()
	mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t2", `["pg_parent"]`))
	mock.ExpectQuery(`.*`).
		WithArgs("pg_parent").
		WillReturnRows(sqlmock.NewRows([]string{"count"}).AddRow(0))
	mock.ExpectCommit()

	task, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err != nil || task != nil {
		t.Errorf("expected nil task for blocked")
	}

    // 4. Update failure
    mock.ExpectBegin()
	mock.ExpectQuery(`.*`).WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t1", "[]"))
	mock.ExpectQuery(`.*`).WillReturnError(errors.New("update err"))
	mock.ExpectRollback()
    _, err = scheduler.ClaimTask(context.Background(), "agent1")
    if err == nil {
        t.Errorf("expected update err")
    }
}

func TestClaimTaskSQLite_Mocked(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")

	// SQLite Tx begin failure
	mock.ExpectBegin().WillReturnError(errors.New("begin error"))
	_, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected begin error")
	}

    // SQLite Update failure
    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("sq_t1", "[]"))
    mock.ExpectQuery(`.*`).WillReturnError(errors.New("update error"))
    mock.ExpectRollback()
    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected update error")
	}

	// SQLite Race condition NoRows
	mock.ExpectBegin()
    mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("sq_t1", "[]"))
    mock.ExpectQuery(`.*`).WillReturnError(sql.ErrNoRows)
    mock.ExpectCommit()
    task, err := scheduler.ClaimTask(context.Background(), "agent1")
	if err != nil || task != nil {
		t.Errorf("expected graceful nil for sqlite update norows race condition")
	}
}

func TestCheckDependenciesPostgres(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")
    ctx := context.Background()

    mock.ExpectBegin()
    tx, _ := sqlxDB.BeginTxx(ctx, nil)

    // missing parent
    mock.ExpectQuery(`.*`).WithArgs("pg_parent").WillReturnRows(sqlmock.NewRows([]string{"count"}).AddRow(0))
    unblocked, _ := scheduler.checkDependenciesPostgres(ctx, tx, `["pg_parent"]`, "shared_tasks")
    if unblocked {
        t.Errorf("expected blocked on missing parent")
    }

    // sql query error
    mock.ExpectQuery(`.*`).WithArgs("pg_parent").WillReturnError(errors.New("db error"))
    _, err = scheduler.checkDependenciesPostgres(ctx, tx, `["pg_parent"]`, "shared_tasks")
    if err == nil {
        t.Errorf("expected error")
    }

    // invalid json
    _, err = scheduler.checkDependenciesPostgres(ctx, tx, `invalid`, "shared_tasks")
    if err == nil {
        t.Errorf("expected error")
    }
}

func TestCheckDependenciesPostgres_LenZero(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()
	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")
    mock.ExpectBegin()
    tx, _ := sqlxDB.BeginTxx(context.Background(), nil)
    unblocked, _ := scheduler.checkDependenciesPostgres(context.Background(), tx, "[]", "shared_tasks")
    if !unblocked { t.Errorf("expected true") }
}

func TestCheckDependenciesSQLite_LenZero(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()
	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")
    mock.ExpectBegin()
    tx, _ := sqlxDB.BeginTxx(context.Background(), nil)
    unblocked, _ := scheduler.checkDependenciesSQLite(context.Background(), tx, "[]", "shared_tasks")
    if !unblocked { t.Errorf("expected true") }
}

func TestCheckDependenciesSQLite_Err(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()
	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")
    mock.ExpectBegin()
    tx, _ := sqlxDB.BeginTxx(context.Background(), nil)
    mock.ExpectQuery(`.*`).WillReturnError(errors.New("sqlite mock err"))
    _, err = scheduler.checkDependenciesSQLite(context.Background(), tx, `["dep1"]`, "shared_tasks")
    if err == nil { t.Errorf("expected err") }
}

func TestClaimTaskPostgres_CommitErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

    mock.ExpectBegin()
	mock.ExpectQuery(`(?s).*SELECT t.id, t.dependencies.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t1", "[]"))
	mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "status", "dependencies"}).AddRow("pg_t1", "IN_PROGRESS", "[]"))
	mock.ExpectCommit().WillReturnError(errors.New("commit error"))

	_, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected commit err")
	}

    // Blocked commit err
    mock.ExpectBegin()
	mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t2", `["pg_parent"]`))
	mock.ExpectQuery(`.*`).
		WithArgs("pg_parent").
		WillReturnRows(sqlmock.NewRows([]string{"count"}).AddRow(0))
	mock.ExpectCommit().WillReturnError(errors.New("commit error"))

	_, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err != nil {
		t.Errorf("expected no err for blocked commit err since we ignore it")
	}
}

func TestClaimTaskSQLite_CommitErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("sq_t1", "[]"))
    mock.ExpectQuery(`.*`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "status", "dependencies"}).AddRow("sq_t1", "IN_PROGRESS", "[]"))
    mock.ExpectCommit().WillReturnError(errors.New("commit error"))

	_, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected commit err")
	}
}

func TestClaimTaskSQLite_RowsErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).WillReturnError(errors.New("rows err"))
    mock.ExpectRollback()

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected db query err")
	}
}

func TestClaimTaskPostgres_QueryErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).WillReturnError(errors.New("db query err"))
    mock.ExpectRollback()

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected db query err")
	}
}

func TestClaimTaskPostgres_TxBeginErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

    mock.ExpectBegin().WillReturnError(errors.New("tx begin err"))

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected tx begin err")
	}
}

func TestClaimTaskPostgres_DepsErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies"}).AddRow("pg_t1", `["missing"]`))
    mock.ExpectQuery(`.*`).WillReturnError(errors.New("deps err"))
    mock.ExpectRollback()

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected deps err")
	}
}

func TestClaimTaskSQLite_StructScanErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "sqlite3")
	scheduler := NewScheduler(sqlxDB, "sqlite3")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies", "bad_col"}).AddRow("sq_t1", "[]", "bad"))
    mock.ExpectRollback()

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected db scan err")
	}
}

func TestClaimTaskPostgres_StructScanErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("error opening stub db: %s", err)
	}
	defer db.Close()

	sqlxDB := sqlx.NewDb(db, "postgres")
	scheduler := NewScheduler(sqlxDB, "postgres")

    mock.ExpectBegin()
    mock.ExpectQuery(`.*`).WillReturnRows(sqlmock.NewRows([]string{"id", "dependencies", "bad_col"}).AddRow("sq_t1", "[]", "bad"))
    mock.ExpectRollback()

    _, err = scheduler.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Errorf("expected db scan err")
	}
}

func TestCheckDependenciesSQLite_InQueryErr(t *testing.T) {
}
