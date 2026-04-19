package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"
	"github.com/stretchr/testify/require"
)

func TestSharedTasksDAG(t *testing.T) {
	ctx := context.Background()
	ctx = auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "org-test", UserID: "user-1"})

	// Ensure SQLite memory cache is shared as requested.
	provider, err := db.NewSqliteProviderForTest("sqlite://file::memory:?cache=shared")
	require.NoError(t, err)

	dbConn := &db.DB{Provider: provider}
	err = dbConn.RunMigrations(ctx)
	require.NoError(t, err)

	sm := statemachine.NewTaskStateMachine(provider, nil)
	tm := NewTaskManager(dbConn, nil, nil, nil, sm)

	// Create Tasks
	t1 := "task-1"
	t2 := "task-2"
	_, err = dbConn.Exec(ctx, `INSERT INTO shared_tasks (id, title, status, organization_id) VALUES ($1, 't1', 'PENDING', 'org-test')`, t1)
	require.NoError(t, err)
	_, err = dbConn.Exec(ctx, `INSERT INTO shared_tasks (id, title, status, organization_id) VALUES ($1, 't2', 'PENDING', 'org-test')`, t2)
	require.NoError(t, err)

	// Dependency: t2 depends on t1
	_, err = dbConn.Exec(ctx, `INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)`, t2, t1)
	require.NoError(t, err)

	// Attempt to claim t2 (should fail or return nil because t1 is not COMPLETED)
	claimedTask2, err := tm.ClaimTask(ctx, t2, "agent-1")
	require.NoError(t, err)
	require.Nil(t, claimedTask2) // Blocked by dependency

	// Claim t1 (should succeed)
	claimedTask1, err := tm.ClaimTask(ctx, t1, "agent-1")
	require.NoError(t, err)
	require.NotNil(t, claimedTask1)
	require.Equal(t, t1, claimedTask1.ID)

	// Complete t1
	err = sm.Transition(ctx, t1, statemachine.EventTaskCompleted)
	require.NoError(t, err)

	// Claim t2 (should now succeed)
	claimedTask2After, err := tm.ClaimTask(ctx, t2, "agent-1")
	require.NoError(t, err)
	require.NotNil(t, claimedTask2After)
	require.Equal(t, t2, claimedTask2After.ID)
}

func TestClaimTaskUpdatesAssignedAgent(t *testing.T) {
	ctx := context.Background()
	ctx = auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "org-test", UserID: "user-1"})

	provider, err := db.NewSqliteProviderForTest("sqlite://file::memory:?cache=shared")
	require.NoError(t, err)

	dbConn := &db.DB{Provider: provider}
	err = dbConn.RunMigrations(ctx)
	require.NoError(t, err)

	sm := statemachine.NewTaskStateMachine(provider, nil)
	tm := NewTaskManager(dbConn, nil, nil, nil, sm)

	t1 := "task-1"
	_, err = dbConn.Exec(ctx, `INSERT INTO shared_tasks (id, title, status, organization_id) VALUES ($1, 't1', 'PENDING', 'org-test')`, t1)
	require.NoError(t, err)

	claimedTask, err := tm.ClaimTask(ctx, t1, "agent-5")
	require.NoError(t, err)
	require.NotNil(t, claimedTask)

	var assigned string
	err = dbConn.QueryRow(ctx, "SELECT assigned_agent_id FROM shared_tasks WHERE id = $1", t1).Scan(&assigned)
	require.NoError(t, err)
	require.Equal(t, "agent-5", assigned)
}
