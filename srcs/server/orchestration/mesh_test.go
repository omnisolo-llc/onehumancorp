package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
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

	shard := mesh.shards[getShard(channel)]
	shard.mu.RLock()
	defer shard.mu.RUnlock()
	assert.Empty(t, shard.subscribers[channel])
}

func TestCentrifugeMesh_PublishSubscribe(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	// Use miniredis for tests
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	pub, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	require.NoError(t, err)

	sub, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	require.NoError(t, err)

	mesh := &CentrifugeMesh{
		BaseURL:    ts.URL,
		HTTPClient: &http.Client{Timeout: 5 * time.Second},
		pubClient:  pub,
		subClient:  sub,
	}
	defer pub.Close()
	defer sub.Close()

	ctx := context.Background()
	channel := "test_channel"

	received := make(chan []byte, 1)

	err = mesh.Subscribe(ctx, channel, func(data []byte) {
		received <- data
	})
	require.NoError(t, err)

	// small delay to let subscription settle
	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish(ctx, channel, []byte("hello"))
	require.NoError(t, err)

	select {
	case msg := <-received:
		assert.Equal(t, []byte("hello"), msg)
	case <-time.After(time.Second):
		t.Fatal("did not receive message")
	}
}
