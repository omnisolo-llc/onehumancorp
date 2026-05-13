package orchestration

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestTaskStateMachine_ProcessEvent_Failure(t *testing.T) {
	db := setupSMTestDB(t)
	defer db.Close()

	sm := NewTaskStateMachine(db)
	ctx := context.Background()

	// Insert parent task
	_, err := db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('parent-fail', 'tenant-1', 'EXECUTING')")
	require.NoError(t, err)

	err = sm.ProcessEvent(ctx, "parent-fail", EventSubTaskFailed)
	require.NoError(t, err)

	var status string
	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-fail'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "FAILED", status)
}
