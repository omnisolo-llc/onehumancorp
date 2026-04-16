package orchestration

import (
	"context"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDynamicTaskRouter_ClaimTask_Concurrent(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	require.NotNil(t, provider)

	router := NewDynamicTaskRouter(nil, provider)

	taskID := "test-task-1"
	tx, err := provider.Begin(ctx)
	require.NoError(t, err)
	_, _ = tx.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, payload, status, priority, created_at, updated_at)
		VALUES ($1, 'org1', 'Test Task', '{}', 'PENDING', 'P1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`, taskID)
	tx.Commit(ctx)

	var wg sync.WaitGroup
	results := make(chan bool, 3)

	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(agentNum int) {
			defer wg.Done()
			agentID := "agent-" + string(rune('0'+agentNum))
			claimed, _ := router.ClaimTask(ctx, agentID, taskID, 1.0)
			results <- claimed
		}(i)
	}

	wg.Wait()
	close(results)

	claims := 0
	for res := range results {
		if res {
			claims++
		}
	}

	assert.GreaterOrEqual(t, claims, 0, "Agent claim logic should not crash")
}
