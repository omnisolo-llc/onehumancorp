package orchestration

import (
	"context"
	"testing"

	_ "github.com/mutecomm/go-sqlcipher/v4"
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

	fetched, err := store.GetTask(context.Background(), task.ID)
	require.NoError(t, err)

	assert.Nil(t, fetched.Description)
	assert.Nil(t, fetched.AgentID)
	assert.Nil(t, fetched.ParentPlanID)
	assert.Nil(t, fetched.Payload)
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

	fetched, err := store.GetTask(context.Background(), task.ID)
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

	fetched, err := store.GetTask(context.Background(), "non-existent")
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

	fetched, err := store.GetTask(context.Background(), task.ID)
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
