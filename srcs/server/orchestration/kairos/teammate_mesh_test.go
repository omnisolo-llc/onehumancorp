package kairos

import (
	"context"
	"sort"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func runTeammateMeshTests(t *testing.T, mesh TeammateMesh, isLocal bool) {
	t.Run("Publish and Subscribe", func(t *testing.T) {
		ctx := context.Background()
		topic := "test:topic"
		payload := []byte("hello, world")

		var wg sync.WaitGroup
		wg.Add(1)

		sub, err := mesh.Subscribe(ctx, topic, func(msg []byte) {
			assert.Equal(t, payload, msg)
			wg.Done()
		})
		require.NoError(t, err)

		// Brief sleep to ensure subscription is registered in Redis
		if !isLocal {
			time.Sleep(50 * time.Millisecond)
		}

		err = mesh.Publish(ctx, topic, payload)
		require.NoError(t, err)

		wg.Wait()

		err = sub.Unsubscribe()
		require.NoError(t, err)
	})

	t.Run("Acquire and Release Lock", func(t *testing.T) {
		ctx := context.Background()
		lockKey := "ohc:lock:system:task:testlock"

		acquired, err := mesh.AcquireLock(ctx, lockKey, 1*time.Second)
		require.NoError(t, err)
		assert.True(t, acquired)

		acquired, err = mesh.AcquireLock(ctx, lockKey, 1*time.Second)
		require.NoError(t, err)
		assert.False(t, acquired)

		err = mesh.ReleaseLock(ctx, lockKey)
		require.NoError(t, err)

		acquired, err = mesh.AcquireLock(ctx, lockKey, 1*time.Second)
		require.NoError(t, err)
		assert.True(t, acquired)
		mesh.ReleaseLock(ctx, lockKey)
	})

	t.Run("Register Presence and Get Active Agents", func(t *testing.T) {
		ctx := context.Background()

		err := mesh.RegisterPresence(ctx, "agent1", "IDLE")
		require.NoError(t, err)
		err = mesh.RegisterPresence(ctx, "agent2", "WORKING")
		require.NoError(t, err)

		agents, err := mesh.GetActiveAgents(ctx)
		require.NoError(t, err)

		assert.Len(t, agents, 2)

		sort.Slice(agents, func(i, j int) bool {
			return agents[i].AgentID < agents[j].AgentID
		})

		assert.Equal(t, "agent1", agents[0].AgentID)
		assert.Equal(t, "IDLE", agents[0].Status)
		assert.Equal(t, "agent2", agents[1].AgentID)
		assert.Equal(t, "WORKING", agents[1].Status)
	})

	t.Run("Acknowledge", func(t *testing.T) {
		ctx := context.Background()
		msgID := "msg123"
		ackTopic := "mesh:ack:" + msgID

		var wg sync.WaitGroup
		wg.Add(1)

		sub, err := mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
			assert.Equal(t, []byte("ack"), msg)
			wg.Done()
		})
		require.NoError(t, err)

		if !isLocal {
			time.Sleep(50 * time.Millisecond)
		}

		err = mesh.Acknowledge(ctx, msgID)
		require.NoError(t, err)

		wg.Wait()
		sub.Unsubscribe()
	})
}

func TestLocalTeammateMesh(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	runTeammateMeshTests(t, mesh, true)
}

func TestRedisTeammateMesh(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisTeammateMesh(client)
	runTeammateMeshTests(t, mesh, false)
}

func TestLocalTeammateMesh_LockExpiry(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()
	lockKey := "ohc:lock:system:task:expirylock"

	acquired, err := mesh.AcquireLock(ctx, lockKey, 50*time.Millisecond)
	require.NoError(t, err)
	assert.True(t, acquired)

	time.Sleep(60 * time.Millisecond)

	acquired, err = mesh.AcquireLock(ctx, lockKey, 50*time.Millisecond)
	require.NoError(t, err)
	assert.True(t, acquired)
}

func TestRedisTeammateMesh_Subscribe_Error(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	mesh := NewRedisTeammateMesh(client)

	client.Close() // Force an error on Subscribe

	_, err = mesh.Subscribe(context.Background(), "test_topic", func(msg []byte) {})
	require.Error(t, err)
}

func TestRedisTeammateMesh_GetActiveAgents_Error(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	mesh := NewRedisTeammateMesh(client)

	client.Close() // Force an error on GetActiveAgents

	_, err = mesh.GetActiveAgents(context.Background())
	require.Error(t, err)
}

// Add coverage for publish error on LocalTeammateMesh if context is cancelled, though it currently ignores context
func TestLocalTeammateMesh_PublishError(t *testing.T) {
	// Not practically testable as Publish always returns nil.
}

func TestHybridTeammateMesh(t *testing.T) {
	// Test with redis
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer redisClient.Close()

	meshRedis := NewHybridTeammateMesh(redisClient, nil)
	assert.IsType(t, &RedisTeammateMesh{}, meshRedis)

	// Test without redis
	meshLocal := NewHybridTeammateMesh(nil, nil)
	assert.IsType(t, &LocalTeammateMesh{}, meshLocal)
}
