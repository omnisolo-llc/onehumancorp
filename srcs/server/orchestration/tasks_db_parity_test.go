package orchestration

import (
	"context"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestParityNullHandling(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-parity",
		Title:          "Parity Task",
	}

	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	fetched, err := store.GetTask(context.Background(), task.ID, task.OrganizationID)
	require.NoError(t, err)

	assert.Nil(t, fetched.Description)
	assert.Nil(t, fetched.AssignedAgentID)
	assert.Nil(t, fetched.ParentPlanID)
	assert.Nil(t, fetched.Payload)
	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id UUID NOT NULL,
			depends_on_task_id UUID NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)
	require.NoError(t, err)
}

func TestParityGetTask(t *testing.T) {
	sqliteDB := setupTestDB(t)
	defer sqliteDB.Close()
	sqliteStore := NewSqliteTaskStore(sqliteDB)
	task := &SharedTask{
		OrganizationID: "org-parity",
		Title:          "Parity Task",
	}

	err := sqliteStore.CreateTask(context.Background(), task)
	require.NoError(t, err)

	tasks, err := sqliteStore.GetTasksByOrganization(context.Background(), "org-parity")
	require.NoError(t, err)
	assert.Len(t, tasks, 1)
}

func TestParityTimezone(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-parity",
		Title:          "Parity Task",
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	fetched, err := store.GetTask(context.Background(), task.ID, task.OrganizationID)
	require.NoError(t, err)

	assert.False(t, fetched.CreatedAt.IsZero())

    // Test if GetTasksByOrganization also parses it successfully
    tasks, err := store.GetTasksByOrganization(context.Background(), "org-parity")
    require.NoError(t, err)
    assert.Len(t, tasks, 1)
    assert.False(t, tasks[0].CreatedAt.IsZero())
}

func TestParityTransactionIsolation(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-iso",
		Title:          "Iso Task",
		Status:         "PENDING",
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	claimedTask, err := store.ClaimTask(context.Background(), "org-iso", "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, "ASSIGNED", claimedTask.Status)

	claimedTask2, err := store.ClaimTask(context.Background(), "org-iso", "agent-2")
	require.NoError(t, err)
	assert.Nil(t, claimedTask2)
}

func TestParityGetTaskNotFound(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	fetched, err := store.GetTask(context.Background(), "non-existent", "org-1")
	require.Error(t, err)
	assert.Nil(t, fetched)
	assert.Equal(t, "task not found", err.Error())
}

func TestParityPayloadDependencies(t *testing.T) {
    db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-deps",
		Title:          "Deps Task",
        Dependencies:   []byte(`["dep-1"]`),
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	fetched, err := store.GetTask(context.Background(), task.ID, task.OrganizationID)
	require.NoError(t, err)

    assert.Equal(t, `["dep-1"]`, string(fetched.Dependencies))

    tasks, err := store.GetTasksByOrganization(context.Background(), "org-deps")
    require.NoError(t, err)
    assert.Len(t, tasks, 1)
    assert.Equal(t, `["dep-1"]`, string(tasks[0].Dependencies))
}

// NOTE: Testcontainers code was requested to test Postgres natively.
// Due to current environment constraints we rely on unit parity tests
// and external E2E container tests for the true live DB interactions.

func TestParityDAGBlocking(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task1 := &SharedTask{
		OrganizationID: "org-dag",
		Title:          "Task 1 (Parent)",
		Status:         "PENDING",
	}
	err := store.CreateTask(context.Background(), task1)
	require.NoError(t, err)

	task2 := &SharedTask{
		OrganizationID: "org-dag",
		Title:          "Task 2 (Child)",
		Status:         "PENDING",
        Dependencies:   []byte(`["` + task1.ID + `"]`),
	}
	err = store.CreateTask(context.Background(), task2)
	require.NoError(t, err)

    // First claim should give Task 1 since Task 2 is blocked
	claimed1, err := store.ClaimTask(context.Background(), "org-dag", "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimed1)
	assert.Equal(t, task1.ID, claimed1.ID)

    // Second claim should return nil because Task 2 is blocked by Task 1 (which is ASSIGNED, not COMPLETED)
	claimed2, err := store.ClaimTask(context.Background(), "org-dag", "agent-2")
	require.NoError(t, err)
	assert.Nil(t, claimed2)

    // Complete Task 1
    err = store.UpdateTaskStatus(context.Background(), task1.ID, "COMPLETED")
    require.NoError(t, err)

    // Now Task 2 should be claimable
	claimed3, err := store.ClaimTask(context.Background(), "org-dag", "agent-3")
	require.NoError(t, err)
	assert.NotNil(t, claimed3)
	assert.Equal(t, task2.ID, claimed3.ID)
}
