package orchestration

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewLocalTeammateMesh()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test_channel"
	message := []byte("hello, world")

	var wg sync.WaitGroup
	var receivedCount int32
	numSubscribers := 5

	for i := 0; i < numSubscribers; i++ {
		wg.Add(1)
		err := mesh.Subscribe(ctx, channel, func(data []byte) {
			assert.Equal(t, message, data)
			atomic.AddInt32(&receivedCount, 1)
			wg.Done()
		})
		require.NoError(t, err)
	}

	err := mesh.Publish(ctx, channel, message)
	require.NoError(t, err)

	// Wait for all subscribers to receive the message
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(1 * time.Second):
		t.Fatalf("Timeout waiting for messages, received %d/%d", atomic.LoadInt32(&receivedCount), numSubscribers)
	}

	assert.Equal(t, int32(numSubscribers), atomic.LoadInt32(&receivedCount))
}

func TestLocalTeammateMesh_PublishNoSubscribers(t *testing.T) {
	mesh := NewLocalTeammateMesh()

	err := mesh.Publish(context.Background(), "no_subs", []byte("hello"))
	require.NoError(t, err)
}

func TestLocalTeammateMesh_UnsubscribeOnContextDone(t *testing.T) {
	mesh := NewLocalTeammateMesh()

	ctx, cancel := context.WithCancel(context.Background())
	channel := "test_channel"

	err := mesh.Subscribe(ctx, channel, func(data []byte) {
		t.Error("Should not have received message after context cancellation")
	})
	require.NoError(t, err)

	cancel()

	// Yield to let the goroutine clean up
	time.Sleep(10 * time.Millisecond)

	err = mesh.Publish(context.Background(), channel, []byte("hello"))
	require.NoError(t, err)

	// Wait a bit to ensure handler isn't called
	time.Sleep(50 * time.Millisecond)

	s := mesh.shards[getShardIndex(channel)]
	s.mu.RLock()
	defer s.mu.RUnlock()
	assert.Empty(t, mesh.shards[getShardIndex(channel)].subscribers[channel])
}

func TestCentrifugeMesh_PublishSubscribe(t *testing.T) {
	mesh := NewCentrifugeMesh()

	ctx := context.Background()
	channel := "test_channel"

	err := mesh.Subscribe(ctx, channel, func(data []byte) {})
	require.NoError(t, err)

	err = mesh.Publish(ctx, channel, []byte("hello"))
	require.NoError(t, err)
}
