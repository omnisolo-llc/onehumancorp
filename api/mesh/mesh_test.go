package mesh

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
)

// setupTestRedis initializes a mock-friendly or real Redis connection for testing.
// Assuming Redis is available at localhost:6379 for integration tests, or use a miniredis.
func setupTestRedis() *redis.Client {
	return redis.NewClient(&redis.Options{
		Addr: "localhost:6379",
	})
}

func TestTeammateMesh_PublishSubscribe(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Ensure Redis is reachable, otherwise skip the test.
	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	mesh := NewTeammateMesh(client)
	topic := "test_topic"

	msgReceived := make(chan MeshMessage, 1)
	handler := func(msg MeshMessage) {
		msgReceived <- msg
	}

	mesh.Subscribe(ctx, topic, handler)

	// Allow subscription to be established
	time.Sleep(100 * time.Millisecond)

	expectedMsg := MeshMessage{
		SenderID: "test_agent",
		Topic:    topic,
		Payload:  json.RawMessage(`{"status":"OK"}`),
	}

	if err := mesh.Publish(ctx, expectedMsg); err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case <-ctx.Done():
		t.Fatal("Timeout waiting for message")
	case msg := <-msgReceived:
		if msg.SenderID != expectedMsg.SenderID {
			t.Errorf("Expected SenderID %s, got %s", expectedMsg.SenderID, msg.SenderID)
		}
		if msg.Topic != expectedMsg.Topic {
			t.Errorf("Expected Topic %s, got %s", expectedMsg.Topic, msg.Topic)
		}
		if string(msg.Payload) != string(expectedMsg.Payload) {
			t.Errorf("Expected Payload %s, got %s", expectedMsg.Payload, msg.Payload)
		}
	}
}
