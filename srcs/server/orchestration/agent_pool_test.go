package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestForkAgent(t *testing.T) {
	db, err := sql.Open("sqlite3", "file::memory:?mode=memory&cache=shared")
	require.NoError(t, err)
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			title TEXT,
			description TEXT,
			status TEXT,
			agent_id TEXT,
			priority TEXT,
			payload BLOB,
			parent_plan_id TEXT,
			dependencies BLOB,
			created_at DATETIME,
			updated_at DATETIME
		)
	`)
	require.NoError(t, err)

	store := NewSqliteTaskStore(db)
	pool := NewAgentPool(store)
	ctx := context.Background()

	parentPayload := json.RawMessage(`{"memory": "some parent memory", "conversation": ["hello", "world"]}`)
	desc := "parent description"
	parentTask := &SharedTask{
		ID:             "parent-1",
		OrganizationID: "org-1",
		Title:          "Parent Task",
		Description:    &desc,
		Status:         "IN_PROGRESS",
		Priority:       "P1",
		Payload:        &parentPayload,
	}

	err = store.CreateTask(ctx, parentTask)
	require.NoError(t, err)

	childID, err := pool.ForkAgent(ctx, "parent-1", "Go do some sub-task")
	require.NoError(t, err)
	assert.NotEmpty(t, childID)

	childTask, err := store.GetTask(ctx, childID)
	require.NoError(t, err)

	assert.Equal(t, "org-1", childTask.OrganizationID)
	assert.Equal(t, "Forked Subagent: Go do some sub-task", childTask.Title)
	assert.Equal(t, "PENDING", childTask.Status)
	assert.Equal(t, "P1", childTask.Priority)
	assert.Equal(t, "parent-1", *childTask.ParentPlanID)

	// Ensure the child's starting state matches the parent's exact snapshot at the time of forking
	assert.NotNil(t, childTask.Payload)
	assert.Equal(t, string(parentPayload), string(*childTask.Payload))
}
