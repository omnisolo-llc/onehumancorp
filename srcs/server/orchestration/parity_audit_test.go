package orchestration

import (
	"context"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// Tests mode parity for Transaction Isolation using SQLite where SKIP LOCKED is not supported
// but table lock acts as the isolation mechanism.
func TestParityAudit_SqliteTransactionIsolation(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-iso-parity",
		Title:          "Iso Task SQLite",
		Status:         "PENDING",
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	// Simulate concurrent access where one agent claims the task
	claimedTask, err := store.ClaimTask(context.Background(), "org-iso-parity", "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, "ASSIGNED", claimedTask.Status)

	// The second agent should get nil back (no task available)
	claimedTask2, err := store.ClaimTask(context.Background(), "org-iso-parity", "agent-2")
	require.NoError(t, err)
	assert.Nil(t, claimedTask2)
}

func TestParityAudit_SqliteNullHandling(t *testing.T) {
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

func TestParityAudit_SqliteTimezoneHandling(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	store := NewSqliteTaskStore(db)

	task := &SharedTask{
		OrganizationID: "org-parity-tz",
		Title:          "Parity Task TZ",
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	fetched, err := store.GetTask(context.Background(), task.ID)
	require.NoError(t, err)
	assert.False(t, fetched.CreatedAt.IsZero())

	// Wait briefly
	time.Sleep(1100 * time.Millisecond)

	err = store.UpdateTaskStatus(context.Background(), task.ID, "DONE")
	require.NoError(t, err)

	fetchedUpdated, err := store.GetTask(context.Background(), task.ID)
	require.NoError(t, err)
	assert.True(t, fetchedUpdated.UpdatedAt.After(fetched.UpdatedAt) || fetchedUpdated.UpdatedAt.Unix() > fetched.UpdatedAt.Unix())
}
