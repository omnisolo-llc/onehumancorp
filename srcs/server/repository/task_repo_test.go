package repository

import (
	"context"
	"database/sql"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_role TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE CASCADE
		);
	`)
	require.NoError(t, err)

	return db
}

func TestSQLTaskRepository_CreateTask(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.WithValue(context.Background(), OrgIDKey, "org-1")

	desc := "Test Desc"
	role := "Role-1"
	task := &Task{
		ID:                "task-1",
		OrganizationID:    "org-1",
		Title:             "Test Task",
		Description:       &desc,
		Status:            "PENDING",
		AssignedAgentRole: &role,
	}

	err := repo.CreateTask(ctx, task)
	assert.NoError(t, err)

	// Verify task created
	tasks, err := repo.GetTasksByOrg(ctx, "org-1")
	assert.NoError(t, err)
	assert.Len(t, tasks, 1)
	assert.Equal(t, "task-1", tasks[0].ID)
	assert.False(t, tasks[0].CreatedAt.IsZero())
	assert.False(t, tasks[0].UpdatedAt.IsZero())
}

func TestSQLTaskRepository_CreateTask_Nullables(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.WithValue(context.Background(), OrgIDKey, "org-1")

	task := &Task{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
	}

	err := repo.CreateTask(ctx, task)
	assert.NoError(t, err)

	// Verify task created and nulls map fine
	tasks, err := repo.GetTasksByOrg(ctx, "org-1")
	assert.NoError(t, err)
	assert.Len(t, tasks, 1)
	assert.Nil(t, tasks[0].Description)
	assert.Nil(t, tasks[0].AssignedAgentRole)
}

func TestSQLTaskRepository_GetTasksByOrg_Unauthorized(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.WithValue(context.Background(), OrgIDKey, "org-1")
	_, err := repo.GetTasksByOrg(ctx, "org-2")
	assert.Error(t, err)
	assert.Equal(t, "unauthorized organization access", err.Error())
}

func TestSQLTaskRepository_UpdateTaskStatus(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.WithValue(context.Background(), OrgIDKey, "org-1")

	task := &Task{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
	}
	err := repo.CreateTask(ctx, task)
	require.NoError(t, err)

	err = repo.UpdateTaskStatus(ctx, "task-1", "IN_PROGRESS")
	assert.NoError(t, err)

	tasks, err := repo.GetTasksByOrg(ctx, "org-1")
	assert.NoError(t, err)
	assert.Len(t, tasks, 1)
	assert.Equal(t, "IN_PROGRESS", tasks[0].Status)
}

func TestSQLTaskRepository_UpdateTaskStatus_NotFound(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.WithValue(context.Background(), OrgIDKey, "org-1")
	err := repo.UpdateTaskStatus(ctx, "non-existent", "IN_PROGRESS")
	assert.ErrorIs(t, err, ErrTaskNotFound)
}

func TestOrganizationIDFromContext_Fallback(t *testing.T) {
	ctx := context.Background()
	orgID := OrganizationIDFromContext(ctx)
	assert.Equal(t, "", orgID)

	ctx2 := context.WithValue(context.Background(), "organization_id", "org-1")
	assert.Equal(t, "org-1", OrganizationIDFromContext(ctx2))
}

func TestSQLTaskRepository_GetTasksByOrg_QueryError(t *testing.T) {
	db := setupTestDB(t)
	// Close db to force error
	db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.Background()
	_, err := repo.GetTasksByOrg(ctx, "org-1")
	assert.Error(t, err)
}

func TestSQLTaskRepository_UpdateTaskStatus_QueryError(t *testing.T) {
	db := setupTestDB(t)
	// Close db to force error
	db.Close()
	repo := NewSQLTaskRepository(db)

	ctx := context.Background()
	err := repo.UpdateTaskStatus(ctx, "task-1", "IN_PROGRESS")
	assert.Error(t, err)
}

func TestSQLTaskRepository_GetTasksByOrg_ScanError(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Create table with missing columns to force scan error
	_, err := db.Exec(`DROP TABLE tasks`)
	require.NoError(t, err)
	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL
		);
	`)
	require.NoError(t, err)

	_, err = db.Exec(`INSERT INTO tasks (id, organization_id) VALUES ('task-1', 'org-1')`)
	require.NoError(t, err)

	repo := NewSQLTaskRepository(db)
	ctx := context.Background()
	_, err = repo.GetTasksByOrg(ctx, "org-1")
	assert.Error(t, err)
}
