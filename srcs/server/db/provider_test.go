package db

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestProvider_CreateTask_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db}

	task := &Task{
		ID:       "task-1",
		TenantID: "tenant-1",
		Status:   "PENDING",
	}

	mock.ExpectBegin()
	mock.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectQuery(`INSERT INTO tasks \(id, tenant_id, status, created_at, updated_at\)`).
		WithArgs("task-1", "tenant-1", "PENDING").
		WillReturnRows(sqlmock.NewRows([]string{"created_at", "updated_at"}).
			AddRow(time.Now(), time.Now()))
	mock.ExpectCommit()


	db.Exec(`
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id UUID NOT NULL,
			depends_on_task_id UUID NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	err = provider.CreateTask(ctx, task)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestProvider_CreateTask_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db}

	task := &Task{
		ID:       "task-1",
		TenantID: "tenant-1",
		Status:   "PENDING",
	}

	mock.ExpectExec(`INSERT INTO tasks \(id, tenant_id, status, created_at, updated_at\)`).
		WithArgs("task-1", "tenant-1", "PENDING").
		WillReturnResult(sqlmock.NewResult(1, 1))

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	err = provider.CreateTask(ctx, task)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestProvider_ClaimTask_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db, RedisClient: rdb}

	mock.ExpectBegin()
	mock.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \$1 AND tenant_id = \$2 FOR UPDATE SKIP LOCKED`).
		WithArgs("task-1", "tenant-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \$1 AND tenant_id = \$2`).
		WithArgs("task-1", "tenant-1").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	err = provider.ClaimTask(ctx, "task-1")
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())

	// Test lock already acquired by another redis client
	mr.Set("task_lock:task-2", "locked")
	err = provider.ClaimTask(ctx, "task-2")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "could not acquire distributed lock")
}

func TestProvider_ClaimTask_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db}

	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \? AND status = 'PENDING' AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	err = provider.ClaimTask(ctx, "task-1")
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())

	// Test concurrent modification
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \? AND status = 'PENDING' AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnResult(sqlmock.NewResult(1, 0)) // 0 rows affected

	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "concurrent modification")
}

func TestProvider_ClaimTask_Postgres_Errors(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()
	rdb := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db, RedisClient: rdb}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")

	// Test BeginTx error
	mock.ExpectBegin().WillReturnError(errors.New("begin tx error"))
	err = provider.ClaimTask(ctx, "task-tx")
	assert.Error(t, err)

	// Test query error
	db2, mock2, _ := sqlmock.New()
	provider2 := &Provider{DB: db2, RedisClient: rdb}
	mock2.ExpectBegin()
	mock2.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock2.ExpectQuery(`SELECT status FROM tasks WHERE id = \$1 AND tenant_id = \$2 FOR UPDATE SKIP LOCKED`).
		WithArgs("task-q", "tenant-1").WillReturnError(errors.New("query err"))
	err = provider2.ClaimTask(ctx, "task-q")
	assert.Error(t, err)

	// Test task not found
	db3, mock3, _ := sqlmock.New()
	provider3 := &Provider{DB: db3, RedisClient: rdb}
	mock3.ExpectBegin()
	mock3.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock3.ExpectQuery(`SELECT status FROM tasks WHERE id = \$1 AND tenant_id = \$2 FOR UPDATE SKIP LOCKED`).
		WithArgs("task-miss", "tenant-1").WillReturnError(sql.ErrNoRows)
	err = provider3.ClaimTask(ctx, "task-miss")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "task not found")

	// Test already claimed
	db4, mock4, _ := sqlmock.New()
	provider4 := &Provider{DB: db4, RedisClient: rdb}
	mock4.ExpectBegin()
	mock4.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock4.ExpectQuery(`SELECT status FROM tasks WHERE id = \$1 AND tenant_id = \$2 FOR UPDATE SKIP LOCKED`).
		WithArgs("task-claimed", "tenant-1").WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("COMPLETED"))
	err = provider4.ClaimTask(ctx, "task-claimed")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "already claimed")
}

func TestProvider_ClaimTask_SQLite_Errors(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")

	// query error
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).WillReturnError(errors.New("db error"))
	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)

	// wrong status
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("IN_PROGRESS"))
	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "already claimed")

    // exec error
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \? AND status = 'PENDING' AND tenant_id = \?`).WillReturnError(errors.New("exec error"))
	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
}

func TestProvider_CreateTask_Postgres_Errors(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	task := &Task{ID: "task-err", TenantID: "tenant-1", Status: "PENDING"}

	mock.ExpectBegin()
	mock.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectQuery(`INSERT INTO tasks \(id, tenant_id, status, created_at, updated_at\)`).
		WithArgs("task-err", "tenant-1", "PENDING").
		WillReturnError(errors.New("insert error"))

	err = provider.CreateTask(ctx, task)
	assert.Error(t, err)
}

func TestProvider_CreateTask_SQLite_Errors(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	task := &Task{ID: "task-err", TenantID: "tenant-1", Status: "PENDING"}

	mock.ExpectExec(`INSERT INTO tasks \(id, tenant_id, status, created_at, updated_at\)`).
		WithArgs("task-err", "tenant-1", "PENDING").
		WillReturnError(errors.New("insert error"))

	err = provider.CreateTask(ctx, task)
	assert.Error(t, err)
}

func TestProvider_ClaimTask_Postgres_ExecError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()
	rdb := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db, RedisClient: rdb}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")

	mock.ExpectBegin()
	mock.ExpectExec(`SELECT set_config\('app\.current_tenant', \$1, true\)`).WithArgs("tenant-1").WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \$1 AND tenant_id = \$2 FOR UPDATE SKIP LOCKED`).
		WithArgs("task-1", "tenant-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \$1 AND tenant_id = \$2`).
		WithArgs("task-1", "tenant-1").
		WillReturnError(errors.New("exec error"))

	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
}

func TestProvider_ClaimTask_Postgres_RedisError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	// Set a bad redis address to cause an error
	rdb := redis.NewClient(&redis.Options{Addr: "localhost:1"})

	db, _, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()
	provider := &Provider{DB: db, RedisClient: rdb}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")
	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
}

func TestProvider_ClaimTask_SQLite_RowsAffectedError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db}

	ctx := context.WithValue(context.Background(), TenantKey, "tenant-1")

	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))
	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \? AND status = 'PENDING' AND tenant_id = \?`).
		WithArgs("task-1", "tenant-1").
		WillReturnResult(sqlmock.NewErrorResult(errors.New("rows affected error")))

	err = provider.ClaimTask(ctx, "task-1")
	assert.Error(t, err)
}

func TestProvider_CreateTask_NilDB(t *testing.T) {
    provider := &Provider{}
    err := provider.CreateTask(context.Background(), &Task{})
    assert.Error(t, err)
    assert.Contains(t, err.Error(), "nil")
}

func TestProvider_ClaimTask_NilDB(t *testing.T) {
    provider := &Provider{}
    err := provider.ClaimTask(context.Background(), "task-1")
    assert.Error(t, err)
    assert.Contains(t, err.Error(), "nil")
}
