package queue

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

func setupTestProvider(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqliteDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_id TEXT,
			epic_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent TEXT,
			payload TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)
	return provider
}

func TestKairosQueueService_PushTask(t *testing.T) {
	provider := setupTestProvider(t)
	defer provider.Close()
	svc := NewTaskQueueService(provider)

	t.Run("success push", func(t *testing.T) {
		task := &SharedTask{
			Title: "Test Task",
			OrganizationID: "tenant-1",
		}
		err := svc.PushTask(context.Background(), task)
		assert.NoError(t, err)

		rows, err := provider.Query(context.Background(), "SELECT id, title, status FROM shared_tasks")
		require.NoError(t, err)
		defer rows.Close()

		count := 0
		for rows.Next() {
			var id, title, status string
			err := rows.Scan(&id, &title, &status)
			assert.NoError(t, err)
			assert.NotEmpty(t, id)
			assert.Equal(t, "Test Task", title)
			assert.Equal(t, "PENDING", status)
			count++
		}
		assert.Equal(t, 1, count)
	})
}

func TestKairosQueueService_ClaimTask(t *testing.T) {
	provider := setupTestProvider(t)
	defer provider.Close()
	svc := NewTaskQueueService(provider)

	task := &SharedTask{Title: "Claim Me", OrganizationID: "tenant-1"}
	err := svc.PushTask(context.Background(), task)
	require.NoError(t, err)

	claimed, err := svc.ClaimTask(context.Background(), "agent-1")
	assert.NoError(t, err)
	require.NotNil(t, claimed)
	assert.Equal(t, "Claim Me", claimed.Title)
	assert.Equal(t, "IN_PROGRESS", claimed.Status)
	assert.NotNil(t, claimed.AssignedAgent)
	assert.Equal(t, "agent-1", *claimed.AssignedAgent)

	claimed2, err := svc.ClaimTask(context.Background(), "agent-2")
	assert.NoError(t, err)
	assert.Nil(t, claimed2)
}

func TestKairosQueueService_CompleteTask(t *testing.T) {
	provider := setupTestProvider(t)
	defer provider.Close()
	svc := NewTaskQueueService(provider)

	task := &SharedTask{Title: "To Complete", OrganizationID: "tenant-1"}
	err := svc.PushTask(context.Background(), task)
	require.NoError(t, err)

	claimed, err := svc.ClaimTask(context.Background(), "agent-1")
	require.NoError(t, err)
	require.NotNil(t, claimed)

	err = svc.CompleteTask(context.Background(), claimed.ID)
	assert.NoError(t, err)

	completed, err := svc.GetCompletedTasks(context.Background(), 10)
	assert.NoError(t, err)
	assert.Len(t, completed, 1)
	assert.Equal(t, "COMPLETED", completed[0].Status)
}
