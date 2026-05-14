package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/alicebob/miniredis/v2"
	"github.com/go-redis/redis/v8"
	_ "github.com/mattn/go-sqlite3"
)

func TestTaskOrchestrator(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE swarm_tasks (
		id TEXT PRIMARY KEY,
		mission_id TEXT,
		parent_plan_id TEXT,
		dependencies TEXT,
		title TEXT,
		payload TEXT,
		status TEXT,
		locked_until DATETIME,
		created_at DATETIME,
		updated_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

	task := Task{
		ID:           "task-1",
		MissionID:    "mission-1",
		ParentPlanID: "plan-1",
		Dependencies: "[]",
		Title:        "Test task",
		Payload:      "{}",
		Status:       "PENDING",
		LockedUntil:  time.Now(),
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}

	err = o.DelegateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to delegate task: %v", err)
	}

	acquiredTask, err := o.AcquireTask(ctx, "worker-1")
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if acquiredTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if acquiredTask.Status != "IN_PROGRESS" {
		t.Fatalf("expected status IN_PROGRESS, got %s", acquiredTask.Status)
	}

	err = o.CompleteTask(ctx, acquiredTask.ID)
	if err != nil {
		t.Fatalf("failed to complete task: %v", err)
	}
}

func TestAcquireTaskLockFailure(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	o := NewTaskOrchestrator(nil, redisClient)
	ctx := context.Background()

    // Simulate lock already acquired
    redisClient.SetNX(ctx, "lock:swarm_tasks", "other-worker", 5*time.Second)

	_, err = o.AcquireTask(ctx, "worker-1")
	if err == nil {
		t.Fatalf("expected error acquiring locked task, got nil")
	}
}

func TestAcquireTaskNoRows(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE swarm_tasks (
		id TEXT PRIMARY KEY,
		mission_id TEXT,
		parent_plan_id TEXT,
		dependencies TEXT,
		title TEXT,
		payload TEXT,
		status TEXT,
		locked_until DATETIME,
		created_at DATETIME,
		updated_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

	task, err := o.AcquireTask(ctx, "worker-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task, got %v", task)
	}
}

func TestDelegateTaskDBError(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()
	o := NewTaskOrchestrator(db, nil)
	ctx := context.Background()

    mock.ExpectExec("INSERT INTO swarm_tasks").WillReturnError(sql.ErrConnDone)

    task := Task{Dependencies: "[]", Payload: "{}"}
    err = o.DelegateTask(ctx, task)
    if err == nil {
        t.Fatalf("expected db error")
    }
}

func TestAcquireTaskDBError(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

    mr, _ := miniredis.Run()
    defer mr.Close()
	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

    mock.ExpectQuery("SELECT id, mission_id").WillReturnError(sql.ErrConnDone)

    _, err = o.AcquireTask(ctx, "worker-1")
    if err == nil {
        t.Fatalf("expected db error")
    }
}

func TestCompleteTaskDBError(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()
	o := NewTaskOrchestrator(db, nil)
	ctx := context.Background()

    mock.ExpectExec("UPDATE swarm_tasks SET status = 'COMPLETED'").WillReturnError(sql.ErrConnDone)

    err = o.CompleteTask(ctx, "task-1")
    if err == nil {
        t.Fatalf("expected db error")
    }
}

func TestAcquireTaskUpdateError(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

    mr, _ := miniredis.Run()
    defer mr.Close()
	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

    rows := sqlmock.NewRows([]string{"id", "mission_id", "parent_plan_id", "dependencies", "title", "payload", "status", "locked_until", "created_at", "updated_at"}).
		AddRow("task-1", "m1", "p1", "[]", "title", "{}", "PENDING", time.Now(), time.Now(), time.Now())

    mock.ExpectQuery("SELECT id, mission_id").WillReturnRows(rows)
    mock.ExpectExec("UPDATE swarm_tasks SET status = 'IN_PROGRESS'").WillReturnError(sql.ErrConnDone)

    _, err = o.AcquireTask(ctx, "worker-1")
    if err == nil {
        t.Fatalf("expected update db error")
    }
}

func TestAcquireTaskScanError(t *testing.T) {
    db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

    mr, _ := miniredis.Run()
    defer mr.Close()
	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

    // Provide bad column types to force a scan error
    rows := sqlmock.NewRows([]string{"id", "mission_id", "parent_plan_id", "dependencies", "title", "payload", "status", "locked_until", "created_at", "updated_at"}).
		AddRow("task-1", "m1", "p1", "[]", "title", "{}", "PENDING", "not-a-date", time.Now(), time.Now())

    mock.ExpectQuery("SELECT id, mission_id").WillReturnRows(rows)

    _, err = o.AcquireTask(ctx, "worker-1")
    if err == nil {
        t.Fatalf("expected scan error")
    }
}

func TestAcquireTaskRedisError(t *testing.T) {
    db, _, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

    mr, _ := miniredis.Run()
    defer mr.Close()
	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

    // shutdown to force a client error on SetNX
    mr.Close()

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

    _, err = o.AcquireTask(ctx, "worker-1")
    if err == nil {
        t.Fatalf("expected redis setnx error")
    }
}

func TestAcquireTaskScanSuccess(t *testing.T) {
	db, mock, _ := sqlmock.New()
	defer db.Close()

	mr, _ := miniredis.Run()
	defer mr.Close()

	o := NewTaskOrchestrator(db, redis.NewClient(&redis.Options{Addr: mr.Addr()}))
	ctx := context.Background()

	rows := sqlmock.NewRows([]string{"id", "mission_id", "parent_plan_id", "dependencies", "title", "payload", "status", "locked_until", "created_at", "updated_at"}).
		AddRow("task-1", "m1", "p1", "[]", "title", "{}", "PENDING", time.Now(), time.Now(), time.Now())

	mock.ExpectQuery("SELECT id, mission_id").WillReturnRows(rows)
	mock.ExpectExec("UPDATE swarm_tasks SET status = 'IN_PROGRESS'").WillReturnResult(sqlmock.NewResult(1, 1))

	task, err := o.AcquireTask(ctx, "worker-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task")
	}
}

func TestTaskCoverageDummy(t *testing.T) {
}

func TestAcquireTaskScanErrorOther(t *testing.T) {
    db, mock, _ := sqlmock.New()
	defer db.Close()

    mr, _ := miniredis.Run()
    defer mr.Close()
	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	o := NewTaskOrchestrator(db, redisClient)
	ctx := context.Background()

    // Provide bad column count to force scan error not related to NoRows
    rows := sqlmock.NewRows([]string{"id"}).AddRow("task-1")

    mock.ExpectQuery("SELECT id, mission_id").WillReturnRows(rows)

    _, err := o.AcquireTask(ctx, "worker-1")
    if err == nil {
        t.Fatalf("expected scan error")
    }
}
