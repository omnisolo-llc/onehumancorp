package memory_test

import (
	"context"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/onehumancorp/mono/src/server/services/memory"
)

func setupTestRedis(t *testing.T) (*miniredis.Miniredis, *redis.Client) {
	mr, err := miniredis.Run()
	require.NoError(t, err)

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	return mr, client
}

func TestAgentMemoryService_SaveAndRetrieve(t *testing.T) {
	mr, client := setupTestRedis(t)
	defer mr.Close()
	defer client.Close()

	svc := memory.NewAgentMemoryService(client)
	ctx := context.Background()

	err := svc.SaveEpisodicMemory(ctx, "tenant1", "session1", "test context")
	require.NoError(t, err)

	val, err := svc.RetrieveRecentMemory(ctx, "tenant1", "session1")
	require.NoError(t, err)
	assert.Equal(t, "test context", val)
}

func TestAgentMemoryService_TenantIsolation(t *testing.T) {
	mr, client := setupTestRedis(t)
	defer mr.Close()
	defer client.Close()

	svc := memory.NewAgentMemoryService(client)
	ctx := context.Background()

	// Tenant A saves memory
	err := svc.SaveEpisodicMemory(ctx, "tenantA", "session1", "Tenant A secret context")
	require.NoError(t, err)

	// Tenant B tries to retrieve same session ID
	val, err := svc.RetrieveRecentMemory(ctx, "tenantB", "session1")
	require.NoError(t, err)
	assert.Empty(t, val, "Tenant B should not see Tenant A's memory")

	// Tenant A can still see it
	valA, err := svc.RetrieveRecentMemory(ctx, "tenantA", "session1")
	require.NoError(t, err)
	assert.Equal(t, "Tenant A secret context", valA)
}

func TestAgentMemoryService_NotFound(t *testing.T) {
	mr, client := setupTestRedis(t)
	defer mr.Close()
	defer client.Close()

	svc := memory.NewAgentMemoryService(client)
	ctx := context.Background()

	val, err := svc.RetrieveRecentMemory(ctx, "tenant1", "nonexistent")
	require.NoError(t, err)
	assert.Empty(t, val)
}
