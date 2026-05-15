package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestAutoDreamPipelineIntegration(t *testing.T) {
	ad := NewAutoDream(nil)
	err := ad.Consolidate("test memory", []float32{1.0, 2.0, 3.0})
	if err != nil {
		t.Fatalf("Consolidate failed: %v", err)
	}

	memories, err := ad.QueryMemory([]float32{1.0, 2.0, 3.0}, 10)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	if len(memories) == 0 {
		t.Fatalf("Expected memories")
	}
}

func TestTeammateMeshLocalFallbackIntegration(t *testing.T) {
	tm := NewTeammateMesh("") // empty URL triggers local fallback
	channel := "mesh:tasks:test"

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	msgChan, err := tm.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = tm.Broadcast(channel, "hello mesh")
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	select {
	case msg := <-msgChan:
		if msg != "hello mesh" {
			t.Errorf("Expected 'hello mesh', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

func TestTeammateMeshContextCancellation(t *testing.T) {
	tm := NewTeammateMesh("")
	channel := "mesh:cancellation:test"

	ctx, cancel := context.WithCancel(context.Background())

	msgChan, err := tm.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	// Cancel the context
	cancel()

	// Give the cleanup goroutine time to run
	time.Sleep(50 * time.Millisecond)

	// Broadcast should not panic, and channel should be removed
	err = tm.Broadcast(channel, "hello mesh")
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	// Ensure channel is closed
	select {
	case _, ok := <-msgChan:
		if ok {
			t.Errorf("Channel should be closed")
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for channel close")
	}

	tm.mu.Lock()
	defer tm.mu.Unlock()
	if len(tm.subscribers[channel]) != 0 {
		t.Errorf("Expected subscriber to be removed on cancellation")
	}
}

func TestTeammateMeshMultiChannelIntegrationExtensive(t *testing.T) {
	// Parameterized logic to avoid repeating identical tests
	for i := 0; i < 50; i++ {
		t.Run(fmt.Sprintf("test_multi_channel_%d", i), func(t *testing.T) {
			tm := NewTeammateMesh("") // Local fallback

			channel := fmt.Sprintf("mesh:coordination:channel_%d", i)
			message := fmt.Sprintf("coordination_message_%d", i)

            ctx, cancel := context.WithCancel(context.Background())
	        defer cancel()

			msgChan, err := tm.Subscribe(ctx, channel)
			if err != nil {
				t.Fatalf("Subscribe failed: %v", err)
			}

			err = tm.Broadcast(channel, message)
			if err != nil {
				t.Fatalf("Broadcast failed: %v", err)
			}

			select {
			case msg := <-msgChan:
				if msg != message {
					t.Errorf("Expected '%s', got '%s'", message, msg)
				}
			case <-time.After(1 * time.Second):
				t.Fatal("Timeout waiting for message")
			}
		})
	}
}
