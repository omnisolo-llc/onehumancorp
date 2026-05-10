package orchestration

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"onehumancorp/srcs/server/pb"
)

func TestRedisMeshTransport_PublishSubscribe(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMeshTransport(client)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test:channel"
	payload := []byte("hello, mesh")

	var wg sync.WaitGroup
	wg.Add(1)

	err = mesh.Subscribe(ctx, channel, func(msg []byte) {
		assert.Equal(t, payload, msg)
		wg.Done()
	})
	require.NoError(t, err)

	// Brief sleep to ensure subscription is registered in miniredis
	time.Sleep(50 * time.Millisecond)

	err = mesh.Publish(ctx, channel, payload)
	require.NoError(t, err)

	wg.Wait()
}

func TestRedisMeshTransport_AdvertiseAndDiscover(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMeshTransport(client)
	ctx := context.Background()

	agent1 := pb.Agent{
		ID:           "agent1",
		Capabilities: []string{"skillA", "skillB"},
		Status:       "IDLE",
	}

	agent2 := pb.Agent{
		ID:           "agent2",
		Capabilities: []string{"skillB", "skillC"},
		Status:       "WORKING",
	}

	err = mesh.AdvertiseCapabilities(ctx, agent1)
	require.NoError(t, err)

	err = mesh.AdvertiseCapabilities(ctx, agent2)
	require.NoError(t, err)

	// Discover skillA
	agentsA, err := mesh.DiscoverAgents(ctx, "skillA")
	require.NoError(t, err)
	require.Len(t, agentsA, 1)
	assert.Equal(t, "agent1", agentsA[0].ID)

	// Discover skillB
	agentsB, err := mesh.DiscoverAgents(ctx, "skillB")
	require.NoError(t, err)
	require.Len(t, agentsB, 2)
}

func TestRedisMeshTransport_Subscribe_Error(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	mesh := NewRedisMeshTransport(client)

	client.Close() // Force an error on Subscribe

	err = mesh.Subscribe(context.Background(), "test_topic", func(msg []byte) {})
	require.Error(t, err)
}
