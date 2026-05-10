package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"sync"
	"testing"

	"github.com/alicebob/miniredis/v2"
	_ "github.com/mattn/go-sqlite3"
	"github.com/redis/rueidis"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"onehumancorp/srcs/server/pb"
)

func setupSMTestDB(t *testing.T) *sql.DB {
	// Need a persistent file or shared cache for concurrent sqlite memory tests
	db, err := sql.Open("sqlite3", "file:memdb1?mode=memory&cache=shared")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS ohc_tasks (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			parent_task_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			workflow_state TEXT,
			payload TEXT,
			assigned_agent_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)
	return db
}

// mockMeshTransport for testing transition broadcasting
type mockMeshTransportSM struct {
	published []struct {
		channel string
		data    []byte
	}
}

func (m *mockMeshTransportSM) Publish(ctx context.Context, channel string, data []byte) error {
	m.published = append(m.published, struct {
		channel string
		data    []byte
	}{channel: channel, data: data})
	return nil
}
func (m *mockMeshTransportSM) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error { return nil }
func (m *mockMeshTransportSM) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error { return nil }
func (m *mockMeshTransportSM) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) { return nil, nil }
func (m *mockMeshTransportSM) StartHeartbeat(ctx context.Context, agent pb.Agent) {}

func TestTaskStateMachine_ProcessEvent(t *testing.T) {
	db := setupSMTestDB(t)
	defer db.Close()

	sm := NewTaskStateMachine(db, nil, nil)
	ctx := context.Background()

	// Insert parent task
	_, err := db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('parent-1', 'tenant-1', 'DECOMPOSING')")
	require.NoError(t, err)

	err = sm.ProcessEvent(ctx, "parent-1", EventDecompositionComplete)
	require.NoError(t, err)

	var status string
	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "EXECUTING", status)

	// Insert child tasks
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES ('child-1', 'tenant-1', 'parent-1', 'PENDING')")
	require.NoError(t, err)
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES ('child-2', 'tenant-1', 'parent-1', 'PENDING')")
	require.NoError(t, err)

	// One child completes
	_, err = db.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'DONE' WHERE id = 'child-1'")
	require.NoError(t, err)
	err = sm.ProcessEvent(ctx, "parent-1", EventSubTaskCompleted)
	require.NoError(t, err)

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "EXECUTING", status) // Still executing because child-2 is pending

	// Second child completes
	_, err = db.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'DONE' WHERE id = 'child-2'")
	require.NoError(t, err)
	err = sm.ProcessEvent(ctx, "parent-1", EventSubTaskCompleted)
	require.NoError(t, err)

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "VERIFYING", status) // All children done, should transition to VERIFYING

	// Test concurrent updates
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('parent-2', 'tenant-1', 'EXECUTING')")
	require.NoError(t, err)
	for i := 0; i < 10; i++ {
		_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES (?, 'tenant-1', 'parent-2', 'PENDING')", fmt.Sprintf("c%d", i))
		require.NoError(t, err)
	}

	var wg sync.WaitGroup
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			// Must lock even the update for sqlite
			sm.mu.Lock()
			_, err := db.Exec("UPDATE ohc_tasks SET status = 'DONE' WHERE id = ?", fmt.Sprintf("c%d", idx))
			sm.mu.Unlock()
			assert.NoError(t, err)
			err = sm.ProcessEvent(context.Background(), "parent-2", EventSubTaskCompleted)
			assert.NoError(t, err)
		}(i)
	}
	wg.Wait()

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-2'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "VERIFYING", status)
}

func TestTaskStateMachine_Transition(t *testing.T) {
	db := setupSMTestDB(t)
	defer db.Close()

	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	require.NoError(t, err)
	defer client.Close()

	mockMesh := &mockMeshTransportSM{}
	sm := NewTaskStateMachine(db, client, mockMesh)
	ctx := context.Background()

	// 1. Setup a task
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('task-trans-1', 'tenant-1', 'PENDING')")
	require.NoError(t, err)

	// 2. Successful transition
	err = sm.Transition(ctx, "task-trans-1", "PENDING", "EXECUTING")
	require.NoError(t, err)

	var status string
	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'task-trans-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "EXECUTING", status)

	// Verify event broadcast
	assert.Len(t, mockMesh.published, 1)
	assert.Equal(t, "orchestration", mockMesh.published[0].channel)
	assert.Contains(t, string(mockMesh.published[0].data), "StateTransition")
	assert.Contains(t, string(mockMesh.published[0].data), "task-trans-1")

	// 3. Invalid fromState transition
	// Wait a bit to let the lock expire, miniredis supports TTL but we can manually del it
	client.Do(ctx, client.B().Del().Key("mesh:lock:task-trans-1").Build())

	err = sm.Transition(ctx, "task-trans-1", "PENDING", "DONE")
	assert.ErrorContains(t, err, "invalid state transition")

	// 4. Lock contention simulation
	// Manually set a conflicting lock
	lockCmd := client.B().Set().Key("mesh:lock:task-trans-1").Value("other_agent").Ex(10 * 1000 * 1000 * 1000).Build()
	err = client.Do(ctx, lockCmd).Error()
	require.NoError(t, err)

	err = sm.Transition(ctx, "task-trans-1", "EXECUTING", "DONE")
	assert.ErrorContains(t, err, "could not acquire lock")
}
