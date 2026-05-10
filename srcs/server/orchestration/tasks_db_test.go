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

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies TEXT,
			priority TEXT,
			payload TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	return db
}

func TestSqliteTaskStore_CreateAndGetTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-123",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Dependencies:   json.RawMessage(`["task-1"]`),
	}

	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, task.ID, task.OrganizationID)
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
		Title:          "Task to Claim",
		Status:         "PENDING",
	}
	err := store.CreateTask(ctx, task)
	require.NoError(t, err)

	claimedTask, err := store.ClaimTask(ctx, "org-1", "agent-x")
	require.NoError(t, err)
	require.NotNil(t, claimedTask)

	assert.Equal(t, task.ID, claimedTask.ID)
	assert.Equal(t, "ASSIGNED", claimedTask.Status)
	assert.Equal(t, "agent-x", *claimedTask.AssignedAgentID)

	// Try to claim again
	secondClaim, err := store.ClaimTask(ctx, "org-1", "agent-y")
	require.NoError(t, err)
	assert.Nil(t, secondClaim, "Task should already be claimed")
}

func TestSqliteTaskStore_ClaimTask_WithDependencies(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)
	ctx := context.Background()

	task1 := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-deps",
		Title:          "Task 1 (Parent)",
		Status:         "PENDING",
	}
	err := store.CreateTask(ctx, task1)
	require.NoError(t, err)

	task2 := &SharedTask{
		ID:             "task-2",
		OrganizationID: "org-deps",
		Title:          "Task 2 (Child)",
		Status:         "PENDING",
		Dependencies:   json.RawMessage(`["task-1"]`),
	}
	err = store.CreateTask(ctx, task2)
	require.NoError(t, err)

	// Claim 1 should give task 1
	claimed1, err := store.ClaimTask(ctx, "org-deps", "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimed1)
	assert.Equal(t, "task-1", claimed1.ID)

	// Claim 2 should give nothing because task 2 is blocked by task 1 (which is ASSIGNED, not COMPLETED)
	claimed2, err := store.ClaimTask(ctx, "org-deps", "agent-2")
	require.NoError(t, err)
	assert.Nil(t, claimed2)

	// Complete task 1
	err = store.UpdateTaskStatus(ctx, "task-1", "COMPLETED")
	require.NoError(t, err)

	// Now claim should give task 2
	claimed3, err := store.ClaimTask(ctx, "org-deps", "agent-3")
	require.NoError(t, err)
	assert.NotNil(t, claimed3)
	assert.Equal(t, "task-2", claimed3.ID)
}

func TestPostgresTaskStore_ClaimTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("SET LOCAL app.current_tenant = \\$1").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at").
		WithArgs("org-1").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "parent_plan_id", "title", "description", "status", "assigned_agent_id", "dependencies", "priority", "payload", "created_at", "updated_at"}).
			AddRow("uuid-1", "org-1", nil, "Title", nil, "PENDING", nil, []byte("[]"), "P2", nil, time.Now(), time.Now()))
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

func TestPostgresTaskStore_CreateTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "uuid-123",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P2",
	}

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").WithArgs(task.OrganizationID).WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("INSERT INTO shared_tasks").
		WithArgs(task.ID, task.OrganizationID, task.ParentPlanID, task.Title, task.Description, task.Status, task.AssignedAgentID, []byte("[]"), task.Priority, sqlmock.AnyArg()).
		WillReturnRows(sqlmock.NewRows([]string{"created_at", "updated_at"}).AddRow(time.Now(), time.Now()))
	mock.ExpectCommit()

	err = store.CreateTask(ctx, task)
	require.NoError(t, err)
}

func TestPostgresTaskStore_GetTasksByOrganization(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)
	ctx := context.Background()

	mock.ExpectBegin()
	mock.ExpectExec("SET LOCAL app.current_tenant = \\$1").WithArgs("org-1").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery("SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at").
		WithArgs("org-1").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "parent_plan_id", "title", "description", "status", "assigned_agent_id", "dependencies", "priority", "payload", "created_at", "updated_at"}).
			AddRow("uuid-1", "org-1", nil, "Title", nil, "PENDING", nil, []byte("[]"), "P2", nil, time.Now(), time.Now()))

	tasks, err := store.GetTasksByOrganization(ctx, "org-1")
	require.NoError(t, err)
	assert.Len(t, tasks, 1)
}
