package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestMemoryPubSub(t *testing.T) {
	pubsub := NewMemoryPubSub()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	topic := "test-topic"
	msg := []byte("hello world")

	ch, err := pubsub.Subscribe(ctx, topic)
	assert.NoError(t, err)

	err = pubsub.Publish(ctx, topic, msg)
	assert.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

// Ensure you have a local redis instance running, or mock it for unit tests.
// Skipping the redis test unless explicitly enabled to avoid CI failures without redis.
func TestRedisPubSub(t *testing.T) {
	t.Skip("Skipping Redis PubSub test requiring local redis instance")

	client := redis.NewClient(&redis.Options{
		Addr: "localhost:6379",
	})
	pubsub := NewRedisPubSub(client)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	topic := "test-redis-topic"
	msg := []byte("hello redis")

	ch, err := pubsub.Subscribe(ctx, topic)
	assert.NoError(t, err)

	// Sleep slightly to ensure subscription is active
	time.Sleep(100 * time.Millisecond)

	err = pubsub.Publish(ctx, topic, msg)
	assert.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
