package e2e

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/tasks"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestE2E_TaskDecompositionService(t *testing.T) {
	pool := db.NewTestProvider(t)
	ctx := context.Background()



	var err error
	if dbImpl, ok := pool.(*db.DB); ok {
		err = dbImpl.RunMigrations(ctx)
		if err != nil {
			t.Logf("migrations run result: %v", err)
		}
	} else {
		t.Logf("could not cast pool to *db.DB to run migrations")
	}



	svc := tasks.NewTaskDecompositionService(pool)

	// Create a new task
	taskID, err := svc.Create(ctx, tasks.TaskDecomposition{
		OrganizationID: "test-org",
		Title:          "E2E Test Task",
		Status:         "PENDING",
		Priority:       "P1",
	})
	assert.NoError(t, err)
	assert.NotEmpty(t, taskID)

	// Claim the task
	claimedTask, err := svc.Claim(ctx, "test-org", "agent-x")
	assert.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, taskID, claimedTask.ID)
	assert.Equal(t, "CLAIMED", claimedTask.Status)
	assert.Equal(t, "agent-x", *claimedTask.AssignedAgentID)

	// Verify no more tasks to claim
	claimedTask2, err := svc.Claim(ctx, "test-org", "agent-y")
	assert.NoError(t, err)
	assert.Nil(t, claimedTask2)

	// Update state
	err = svc.UpdateState(ctx, taskID, "COMPLETED")
	assert.NoError(t, err)

	// Verify final state
	finalTask, err := svc.Get(ctx, taskID)
	assert.NoError(t, err)
	assert.NotNil(t, finalTask)
	assert.Equal(t, "COMPLETED", finalTask.Status)
}
