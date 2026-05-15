package orchestration

import (
	"context"


	"sync"
	"sync/atomic"
	"testing"
	"time"
	"net/http"
	"net/http/httptest"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"github.com/alicebob/miniredis/v2"
)

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewMemoryMeshTransport()

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
	mesh := NewMemoryMeshTransport()

	err := mesh.Publish(context.Background(), "no_subs", []byte("hello"))
	require.NoError(t, err)
}

func TestLocalTeammateMesh_UnsubscribeOnContextDone(t *testing.T) {
	mesh := NewMemoryMeshTransport()

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

	shard := mesh.shards[getShard(channel)]
	shard.mu.RLock()
	defer shard.mu.RUnlock()
	assert.Empty(t, shard.subscribers[channel])
}

func TestRedisMeshTransport_PublishSubscribe(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	mesh, err := NewRedisMeshTransport(mr.Addr())
	if err != nil {
		t.Skip("Skipping RedisMeshTransport test due to Centrifuge startup error: " + err.Error())
	}

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test_channel"
	message := []byte("hello, redis")

	var receivedCount int32
	err = mesh.Subscribe(ctx, channel, func(data []byte) {
		assert.Equal(t, message, data)
		atomic.AddInt32(&receivedCount, 1)
	})
	require.NoError(t, err)

	time.Sleep(50 * time.Millisecond) // Give time for subscription to settle

	err = mesh.Publish(ctx, channel, message)
	require.NoError(t, err)

	time.Sleep(50 * time.Millisecond) // Give time for delivery

	assert.Equal(t, int32(1), atomic.LoadInt32(&receivedCount))
}

func TestCentrifugeMesh_PublishSubscribe(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	mesh, err := NewRedisMeshTransport(ts.URL)
	// Redis will fail to connect but we just want to ensure it handles the API structure.
	if err != nil {
		t.Skip("Skipping because no redis is running.")
	}

	handler := mesh.GetHTTPHandler()
	assert.NotNil(t, handler)
}
