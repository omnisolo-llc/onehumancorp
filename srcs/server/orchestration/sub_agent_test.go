package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSubAgentSpawner_Spawn_Standalone(t *testing.T) {
	provider, err := db.NewSQLiteProvider("sqlite://file::memory:?cache=shared")
	require.NoError(t, err)

	err = provider.ExecuteMigrations(context.Background())
	require.NoError(t, err)

	tm := &TaskManager{db: provider}
	hub := NewCentrifugeNode(nil)

	spawner := NewDefaultSubAgentSpawner(provider, tm, hub)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	})

	task, err := tm.CreateTaskWithPlan(ctx, "org-1", nil, "Sub-agent test", "test sub agent", "P1")
	require.NoError(t, err)

	err = spawner.Spawn(ctx, task)
	require.NoError(t, err)

	// Wait for async spawn and run logic
	time.Sleep(200 * time.Millisecond)

	// Fetch task again to check if it's completed
	tasks, err := tm.PeekTasks(ctx, 10)
	require.NoError(t, err)

	// Task should not be in pending anymore if it was completed
	found := false
	for _, t := range tasks {
		if t.ID == task.ID {
			found = true
			break
		}
	}
	assert.False(t, found, "Task should not be in pending state")
}

func TestSubAgentSpawner_Monitor(t *testing.T) {
	provider, err := db.NewSQLiteProvider("sqlite://file::memory:?cache=shared")
	require.NoError(t, err)

	err = provider.ExecuteMigrations(context.Background())
	require.NoError(t, err)

	tm := &TaskManager{db: provider}
	hub := NewCentrifugeNode(nil)

	spawner := NewDefaultSubAgentSpawner(provider, tm, hub)

	ctx, cancel := context.WithCancel(context.Background())

	errCh := make(chan error)
	go func() {
		errCh <- spawner.Monitor(ctx)
	}()

	// Let it run a bit
	time.Sleep(50 * time.Millisecond)
	cancel()

	err = <-errCh
	assert.NoError(t, err)
}

func TestTaskOrchestrator_SubAgentPolling(t *testing.T) {
	provider, err := db.NewSQLiteProvider("sqlite://file::memory:?cache=shared")
	require.NoError(t, err)

	err = provider.ExecuteMigrations(context.Background())
	require.NoError(t, err)

	hub := NewCentrifugeNode(nil)
	to := NewTaskOrchestrator(provider, nil, hub, nil)
	defer to.(*DefaultTaskOrchestrator).Stop()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	})

	// Create a regular task
	tm := &TaskManager{db: provider, hub: hub}
	regularTask, err := tm.CreateTaskWithPlan(ctx, "org-1", nil, "Regular", "reg", "P2")
	require.NoError(t, err)

	// Create a sub-agent task
	subAgentTask, err := tm.CreateTaskWithPlan(ctx, "org-1", nil, "Sub-Agent Task", "sub", "P1")
	require.NoError(t, err)

	// Update payload directly to mimic DELEGATED
	payloadBytes, _ := json.Marshal(map[string]interface{}{
		"sub_agent_type": "IMPLEMENTER",
	})
	_, err = provider.Exec(ctx, "UPDATE shared_tasks SET payload = $1 WHERE id = $2", string(payloadBytes), subAgentTask.ID)
	require.NoError(t, err)

	// Trigger worker poll immediately instead of waiting for timer
	to.(*DefaultTaskOrchestrator).pollAndSpawnDelegatedTasks()

	// Regular task should still be pending
	var status1 string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", regularTask.ID).Scan(&status1)
	require.NoError(t, err)
	assert.Equal(t, "PENDING", status1)

	// Sub agent task should be in progress or completed
	var status2 string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", subAgentTask.ID).Scan(&status2)
	require.NoError(t, err)
	assert.NotEqual(t, "PENDING", status2)
}
