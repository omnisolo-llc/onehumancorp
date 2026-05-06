package orchestration

import (
	"github.com/DATA-DOG/go-sqlmock"
	"time"
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	return db
}

func TestSqliteTaskStore_CreateTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-123",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P1",
		Dependencies:   json.RawMessage(`["task-1", "task-2"]`),
	}

	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, task.ID)
	require.NoError(t, err)

	assert.Equal(t, task.ID, savedTask.ID)
	assert.Equal(t, task.Title, savedTask.Title)
	assert.Equal(t, task.Status, savedTask.Status)
	assert.Equal(t, task.Dependencies, savedTask.Dependencies)
}

func TestSqliteTaskStore_ClaimTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Test Task to Claim",
		Status:         "PENDING",
	}
	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

	claimedTask, err := store.ClaimTask(ctx, "org-1", "agent-x")
	require.NoError(t, err)
	require.NotNil(t, claimedTask)

	assert.Equal(t, task.ID, claimedTask.ID)
	assert.Equal(t, "ASSIGNED", claimedTask.Status)
	assert.Equal(t, "agent-x", *claimedTask.AgentID)

	// Try to claim again
	secondClaim, err := store.ClaimTask(ctx, "org-1", "agent-y")
	require.NoError(t, err)
	assert.Nil(t, secondClaim, "Task should already be claimed")
}

func TestSqliteTaskStore_UpdateTaskStatus(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task to update",
		Status:         "PENDING",
	}
	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

	err = store.UpdateTaskStatus(ctx, task.ID, "COMPLETED")
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, task.ID)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", savedTask.Status)
}

// Mocks for Postgres are complex due to sql.DB, but we can hit some lines with mock errors if needed,
// however, sqlite testing is preferred for covering the logic since the SQL semantics are similar.
// For full >90% coverage as requested, we will use a sqlmock library or we can increase sqlite coverage first.


func TestPostgresTaskStore_CreateTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P1",
	}

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WithArgs(task.OrganizationID).WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("INSERT INTO shared_tasks").
		WithArgs(task.OrganizationID, task.Title, task.Description, task.Status, task.AgentID, task.Priority, sqlmock.AnyArg(), task.ParentPlanID, []byte("[]")).
		WillReturnRows(sqlmock.NewRows([]string{"id", "created_at", "updated_at"}).AddRow("uuid-123", time.Now(), time.Now()))

	mock.ExpectCommit()
	err = store.CreateTask(ctx, task)
	require.NoError(t, err)
	mock.ExpectCommit()
	assert.Equal(t, "uuid-123", task.ID)
}

func TestPostgresTaskStore_ClaimTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("SET LOCAL app.current_tenant = \\$1").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at").
		WithArgs("org-1").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "title", "description", "status", "agent_id", "priority", "payload", "parent_plan_id", "dependencies", "created_at", "updated_at"}).
			AddRow("uuid-1", "org-1", "Title", nil, "PENDING", nil, "P2", []byte(`{"k":"v"}`), nil, []byte("[]"), time.Now(), time.Now()))
	mock.ExpectExec("UPDATE shared_tasks").
		WithArgs("agent-x", "uuid-1").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	task, err := store.ClaimTask(ctx, "org-1", "agent-x")
	require.NoError(t, err)
	require.NotNil(t, task)
	assert.Equal(t, "uuid-1", task.ID)
	assert.Equal(t, "ASSIGNED", task.Status)
}

func TestPostgresTaskStore_GetTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at").
		WithArgs("uuid-1").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "title", "description", "status", "agent_id", "priority", "payload", "parent_plan_id", "dependencies", "created_at", "updated_at"}).
			AddRow("uuid-1", "org-1", "Title", nil, "PENDING", nil, "P2", nil, nil, []byte("[]"), time.Now(), time.Now()))

	mock.ExpectCommit()
	task, err := store.GetTask(ctx, "uuid-1")
	require.NoError(t, err)
	assert.Equal(t, "uuid-1", task.ID)
}

func TestPostgresTaskStore_UpdateTaskStatus(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("UPDATE shared_tasks SET status =").
		WithArgs("COMPLETED", "uuid-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectCommit()
	err = store.UpdateTaskStatus(ctx, "uuid-1", "COMPLETED")
	require.NoError(t, err)
}

func TestPostgresTaskStore_ClaimTask_NoTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("SET LOCAL app.current_tenant = \\$1").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at").
		WithArgs("org-1").
		WillReturnError(sql.ErrNoRows)
	mock.ExpectRollback()

	task, err := store.ClaimTask(ctx, "org-1", "agent-x")
	require.NoError(t, err)
	assert.Nil(t, task)
}

func TestPostgresTaskStore_GetTask_NoTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at").
		WithArgs("uuid-none").
		WillReturnError(sql.ErrNoRows)
	mock.ExpectRollback()

	task, err := store.GetTask(ctx, "uuid-none")
	require.Error(t, err)
	assert.Nil(t, task)
}

func TestSqliteTaskStore_ClaimTask_NoTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	claimedTask, err := store.ClaimTask(ctx, "org-none", "agent-x")
	require.NoError(t, err)
	assert.Nil(t, claimedTask)
}

func TestSqliteTaskStore_GetTask_NoTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task, err := store.GetTask(ctx, "task-none")
	require.Error(t, err)
	assert.Nil(t, task)
}

func TestPostgresTaskStore_ClaimTask_Errors(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	// Error on begin
	mock.ExpectBegin().WillReturnError(sql.ErrConnDone)
	_, err = store.ClaimTask(ctx, "org-1", "agent-x")
	require.Error(t, err)

	// Error on query
	mock.ExpectBegin()
	mock.ExpectExec("SET LOCAL app.current_tenant = \\$1").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("SELECT id").WillReturnError(sql.ErrConnDone)
	mock.ExpectRollback()
	_, err = store.ClaimTask(ctx, "org-1", "agent-x")
	require.Error(t, err)
}

func TestPostgresTaskStore_CreateTask_WithPayloads(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

    rawPayload := json.RawMessage(`{"key":"value"}`)
	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P1",
        Payload:        &rawPayload,
        Dependencies:   json.RawMessage(`["dep-1"]`),
	}

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WithArgs(task.OrganizationID).WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("INSERT INTO shared_tasks").
		WithArgs(task.OrganizationID, task.Title, task.Description, task.Status, task.AgentID, task.Priority, sqlmock.AnyArg(), task.ParentPlanID, sqlmock.AnyArg()).
		WillReturnRows(sqlmock.NewRows([]string{"id", "created_at", "updated_at"}).AddRow("uuid-123", time.Now(), time.Now()))

	mock.ExpectCommit()
	err = store.CreateTask(ctx, task)
	require.NoError(t, err)
}

func TestSqliteTaskStore_ClaimTask_Errors(t *testing.T) {
	db := setupTestDB(t)
	store := NewSqliteTaskStore(db)
	ctx := context.Background()

    // Close DB to force an error on BeginTx
    db.Close()
	_, err := store.ClaimTask(ctx, "org-1", "agent-x")
	require.Error(t, err)
}

func TestSqliteTaskStore_GetTask_ErrorParsing(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	_, err := db.Exec(`INSERT INTO shared_tasks (id, organization_id, title, created_at) VALUES ('bad-date', 'org-1', 'title', 'not-a-date')`)
	require.NoError(t, err)

    // We expect it to still return the task, but parsing of time might silently fail or fallback
    task, err := store.GetTask(ctx, "bad-date")
    require.NoError(t, err)
    require.NotNil(t, task)
}

func TestPostgresTaskStore_GetTask_Errors(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectQuery("SELECT id").
		WithArgs("uuid-err").
		WillReturnError(sql.ErrConnDone)
	mock.ExpectRollback()

	task, err := store.GetTask(ctx, "uuid-err")
	require.Error(t, err)
	assert.Nil(t, task)
}

func TestPostgresTaskStore_UpdateTaskStatus_Errors(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("UPDATE shared_tasks SET status").
		WithArgs("COMPLETED", "uuid-1").
		WillReturnError(sql.ErrConnDone)
	mock.ExpectRollback()

	err = store.UpdateTaskStatus(ctx, "uuid-1", "COMPLETED")
	require.Error(t, err)
}

func TestSqliteTaskStore_CreateTask_WithPayloads(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

    rawPayload := json.RawMessage(`{"key":"value"}`)
	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P1",
        Payload:        &rawPayload,
	}

	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

    savedTask, err := store.GetTask(ctx, task.ID)
    require.NoError(t, err)
    assert.Equal(t, task.Payload, savedTask.Payload)
}
