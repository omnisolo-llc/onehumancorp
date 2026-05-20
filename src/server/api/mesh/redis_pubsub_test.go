package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/go-redis/redis/v8"
)

// mockRedisClient returns a mock or real client for testing
func mockRedisClient() *redis.Client {
    // We attempt connection to a local redis for test
    // In a real environment, we would use miniredis
	return redis.NewClient(&redis.Options{Addr: "localhost:6379"})
}

func TestRedisPubSub(t *testing.T) {
	client := mockRedisClient()
	if err := client.Ping(context.Background()).Err(); err != nil {
		t.Skip("Skipping redis test without running redis server")
	}

	pubsub := &RedisPubSub{client: client}
	defer pubsub.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "test-redis-topic"
	ch, err := pubsub.Subscribe(ctx, topic)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

    // small wait for sub to activate
    time.Sleep(100 * time.Millisecond)

	msg := TeammateMeshEvent{
		EventType: "TASK_UPDATED",
		TaskID:    "task-redis-123",
	}

	go func() {
		err := pubsub.Publish(ctx, topic, msg)
		if err != nil {
			t.Errorf("Failed to publish: %v", err)
		}
	}()

	select {
	case received := <-ch:
		if received.TaskID != msg.TaskID {
			t.Errorf("Expected task ID %s, got %s", msg.TaskID, received.TaskID)
		}
	case <-time.After(time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
