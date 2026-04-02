package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestUltraPlanManager(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	dbWrapper, err := db.New(ctx)
	require.NoError(t, err)
	defer dbWrapper.Close()

	_, err = dbWrapper.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_ultra_plans (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			status TEXT NOT NULL,
			state_machine TEXT,
			created_at DATETIME,
			updated_at DATETIME
		)
	`)
	require.NoError(t, err)

	manager := NewUltraPlanManager(dbWrapper.Provider, nil, nil)

	// Test CreatePlan
	plan, err := manager.CreatePlan(ctx, "mission-123")
	require.NoError(t, err)
	assert.NotEmpty(t, plan.ID)
	assert.Equal(t, "mission-123", plan.MissionID)
	assert.Equal(t, PlanStatusDeliberating, plan.Status)

	// Test GetPlan
	fetchedPlan, err := manager.GetPlan(ctx, plan.ID)
	require.NoError(t, err)
	require.NotNil(t, fetchedPlan)
	assert.Equal(t, plan.ID, fetchedPlan.ID)

	// Test TransitionPlan
	err = manager.TransitionPlan(ctx, plan.ID, PlanStatusExecuting)
	require.NoError(t, err)

	fetchedPlan, err = manager.GetPlan(ctx, plan.ID)
	require.NoError(t, err)
	assert.Equal(t, PlanStatusExecuting, fetchedPlan.Status)

	// Test UpdateStateMachine
	state := map[string]interface{}{"step": float64(1)}
	err = manager.UpdateStateMachine(ctx, plan.ID, state)
	require.NoError(t, err)

	fetchedPlan, err = manager.GetPlan(ctx, plan.ID)
	require.NoError(t, err)
	assert.Equal(t, float64(1), fetchedPlan.StateMachine["step"])
}
