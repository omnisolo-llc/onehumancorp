package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type MockCloudClient struct {
	PushCalls        int
	PushedMemories   map[string]string
	PushedMissionIDs []string
	ErrToReturn      error
}

func (m *MockCloudClient) PushSanitizedMemory(ctx context.Context, memoryID, sanitizedContext string) (string, error) {
	m.PushCalls++
	if m.ErrToReturn != nil {
		return "", m.ErrToReturn
	}
	m.PushedMemories[memoryID] = sanitizedContext
	missionID := fmt.Sprintf("mission-%d", m.PushCalls)
	m.PushedMissionIDs = append(m.PushedMissionIDs, missionID)
	return missionID, nil
}

func setupTestDBForSync(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	provider, err := db.New(context.Background())
	require.NoError(t, err)

	ctx := context.Background()

	// Initialize tables needed for the test.
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory (
			key TEXT PRIMARY KEY,
			value TEXT NOT NULL,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS local_cloud_sync_log (
			sync_id TEXT PRIMARY KEY,
			memory_id TEXT NOT NULL,
			cloud_mission_id TEXT,
			synced_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (memory_id) REFERENCES swarm_memory(key) ON DELETE CASCADE
		)
	`)
	require.NoError(t, err)

	return provider
}

func TestSwarmSynchronizer_ProcessSyncTick(t *testing.T) {
	provider := setupTestDBForSync(t)
	defer provider.Close()

	ctx := context.Background()

	mockClient := &MockCloudClient{
		PushedMemories: make(map[string]string),
	}

	synchronizer := NewSwarmSynchronizer(provider, mockClient)

	// Insert some test data into swarm_memory
	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory (key, value) VALUES ('mem1', 'secret data 1')`)
	require.NoError(t, err)
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory (key, value) VALUES ('mem2', 'secret data 2')`)
	require.NoError(t, err)

	// Insert one of them into the sync log already to ensure it's not synced again
	_, err = provider.Exec(ctx, `INSERT INTO local_cloud_sync_log (sync_id, memory_id, cloud_mission_id) VALUES ('sync1', 'mem1', 'cloud1')`)
	require.NoError(t, err)

	// Execute ProcessSyncTick
	synchronizer.ProcessSyncTick(ctx)

	// mem1 should be skipped, mem2 should be synced
	assert.Equal(t, 1, mockClient.PushCalls)
	assert.Contains(t, mockClient.PushedMemories, "mem2")
	assert.Equal(t, "[SANITIZED] secret data 2", mockClient.PushedMemories["mem2"])

	// Check if the sync log was inserted
	rows, err := provider.Query(ctx, `SELECT memory_id, cloud_mission_id FROM local_cloud_sync_log WHERE memory_id = 'mem2'`)
	require.NoError(t, err)
	defer rows.Close()

	var count int
	var memoryID, cloudMissionID string
	for rows.Next() {
		count++
		err = rows.Scan(&memoryID, &cloudMissionID)
		require.NoError(t, err)
	}

	assert.Equal(t, 1, count, "Should have inserted 1 row into sync log")
	assert.Equal(t, "mem2", memoryID)
	assert.Equal(t, "mission-1", cloudMissionID)

	// Call again, should not sync anything
	mockClient.PushCalls = 0
	synchronizer.ProcessSyncTick(ctx)
	assert.Equal(t, 0, mockClient.PushCalls)
}

func TestSwarmSynchronizer_CloudFailure(t *testing.T) {
	provider := setupTestDBForSync(t)
	defer provider.Close()

	ctx := context.Background()

	mockClient := &MockCloudClient{
		PushedMemories: make(map[string]string),
		ErrToReturn:    fmt.Errorf("cloud error"),
	}

	synchronizer := NewSwarmSynchronizer(provider, mockClient)

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory (key, value) VALUES ('mem_fail', 'data')`)
	require.NoError(t, err)

	// Execute ProcessSyncTick
	synchronizer.ProcessSyncTick(ctx)

	// push was attempted
	assert.Equal(t, 1, mockClient.PushCalls)

	// Verify nothing was added to sync log because it failed
	rows, err := provider.Query(ctx, `SELECT count(*) FROM local_cloud_sync_log`)
	require.NoError(t, err)
	defer rows.Close()

	var count int
	if rows.Next() {
		err = rows.Scan(&count)
		require.NoError(t, err)
	}
	assert.Equal(t, 0, count)

	// Calling again should retry
	mockClient.ErrToReturn = nil
	synchronizer.ProcessSyncTick(ctx)

	assert.Equal(t, 2, mockClient.PushCalls)

	rows, err = provider.Query(ctx, `SELECT count(*) FROM local_cloud_sync_log`)
	require.NoError(t, err)
	defer rows.Close()

	if rows.Next() {
		err = rows.Scan(&count)
		require.NoError(t, err)
	}
	assert.Equal(t, 1, count)
}

func TestSwarmSynchronizer_Sanitization(t *testing.T) {
	input := "raw secret data"
	expected := "[SANITIZED] raw secret data"
	assert.Equal(t, expected, sanitizeContext(input))
}
