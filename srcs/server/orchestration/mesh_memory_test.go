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

func TestMemoryMeshTransport_PublishSubscribe(t *testing.T) {
	mesh := NewMemoryMeshTransport()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test_channel"
	message := []byte(`{"event":"test"}`)

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

	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatalf("Timeout waiting for messages, received %d/%d", atomic.LoadInt32(&receivedCount), numSubscribers)
	}

	assert.Equal(t, int32(numSubscribers), atomic.LoadInt32(&receivedCount))
}

func TestMemoryMeshTransport_UnsubscribeOnContextDone(t *testing.T) {
	mesh := NewMemoryMeshTransport()

	ctx, cancel := context.WithCancel(context.Background())
	channel := "test_channel"

	received := make(chan []byte, 1)
	err := mesh.Subscribe(ctx, channel, func(data []byte) {
		received <- data
	})
	require.NoError(t, err)

	cancel()

	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish(context.Background(), channel, []byte("hello"))
	require.NoError(t, err)

	select {
	case <-received:
		t.Error("Should not have received message after context cancellation")
	case <-time.After(200 * time.Millisecond):
	}

	mesh.mu.RLock()
	defer mesh.mu.RUnlock()
	assert.Empty(t, mesh.subscribers[channel])
}

func TestMemoryMeshTransport_NoSubscribers(t *testing.T) {
	mesh := NewMemoryMeshTransport()
	err := mesh.Publish(context.Background(), "no_one", []byte("data"))
	assert.NoError(t, err)
}
