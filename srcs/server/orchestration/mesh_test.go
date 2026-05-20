package orchestration

import (
	"context"
	"encoding/json"
	"sync"
	"testing"
	"time"
)

type TestMessage struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
}

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()
	channel := "test_channel"

	var wg sync.WaitGroup
	wg.Add(2) // Wait for two subscribers

	var received1, received2 TestMessage

	// Subscriber 1
	err := mesh.Subscribe(ctx, channel, func(data []byte) {
		defer wg.Done()
		if err := json.Unmarshal(data, &received1); err != nil {
			t.Errorf("Failed to unmarshal data: %v", err)
		}
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	// Subscriber 2
	err = mesh.Subscribe(ctx, channel, func(data []byte) {
		defer wg.Done()
		if err := json.Unmarshal(data, &received2); err != nil {
			t.Errorf("Failed to unmarshal data: %v", err)
		}
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := TestMessage{
		AgentID: "agent-1",
		Action:  "test_action",
		Status:  "SUCCESS",
	}

	// Publish message
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	// Wait for handlers to finish, with a timeout
	c := make(chan struct{})
	go func() {
		defer close(c)
		wg.Wait()
	}()

	select {
	case <-c:
		// Completed normally
	case <-time.After(1 * time.Second):
		t.Fatal("Test timed out waiting for subscribers")
	}

	// Verify received messages
	if received1 != msg {
		t.Errorf("Subscriber 1 received incorrect message. Expected %v, got %v", msg, received1)
	}
	if received2 != msg {
		t.Errorf("Subscriber 2 received incorrect message. Expected %v, got %v", msg, received2)
	}
}

func TestLocalTeammateMesh_PublishNoSubscribers(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()
	channel := "empty_channel"

	msg := TestMessage{
		AgentID: "agent-1",
		Action:  "test_action",
		Status:  "SUCCESS",
	}

	// Publish to channel with no subscribers should not error
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}
}
