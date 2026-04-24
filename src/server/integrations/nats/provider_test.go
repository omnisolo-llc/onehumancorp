package nats

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	integration := NewNatsIntegration()
	metadata := integration.Metadata()

	if metadata.GetId() != "nats" {
		t.Errorf("Expected ID to be 'nats', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "NATS" {
		t.Errorf("Expected Name to be 'NATS', got '%s'", metadata.GetName())
	}
}

func TestNatsIntegration_WizardSteps(t *testing.T) {
	integration := NewNatsIntegration()
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Settings" {
		t.Errorf("Expected Title to be 'Connection Settings', got '%s'", step.GetTitle())
	}

	fields := step.GetFields()
	if len(fields) != 2 {
		t.Errorf("Expected 2 fields, got %d", len(fields))
	}

	field1 := fields[0]
	if field1.GetKey() != "url" {
		t.Errorf("Expected Field 1 Key to be 'url', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "url" {
		t.Errorf("Expected Field 1 Type to be 'url', got '%v'", field1.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "credentials" {
		t.Errorf("Expected Field 2 Key to be 'credentials', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "password" {
		t.Errorf("Expected Field 2 Type to be 'password', got '%v'", field2.GetType())
	}
}

func TestNatsIntegration_EventMesh(t *testing.T) {
	ctx := context.Background()
	// Set up the "Cloud" Node (embedded)
	cloudNode := NewNatsIntegration()
	err := cloudNode.Connect("", "", true, -1) // -1 gets a random free port
	if err != nil {
		t.Fatalf("Failed to start cloud node: %v", err)
	}
	defer cloudNode.Disconnect()

	// Get the ClientURL from the embedded server
	cloudURL := cloudNode.Server.ClientURL()

	// Set up the "Standalone" Leaf Node connecting to the Cloud Node
	leafNode := NewNatsIntegration()
	err = leafNode.Connect(cloudURL, "", false, 0)
	if err != nil {
		t.Fatalf("Failed to connect leaf node to cloud node: %v", err)
	}
	defer leafNode.Disconnect()

	var wg sync.WaitGroup
	wg.Add(1)

	var receivedMsg []byte
	// Subscribe on Cloud Node
	sub, err := cloudNode.Subscribe(ctx, "test.subject", func(msg []byte) {
		receivedMsg = msg
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}
	defer sub.Unsubscribe()

	// Publish from Leaf Node
	err = leafNode.Publish(ctx, "test.subject", []byte("hello from leaf"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	// Wait for message with timeout
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		if string(receivedMsg) != "hello from leaf" {
			t.Errorf("Expected 'hello from leaf', got %q", string(receivedMsg))
		}
	case <-time.After(5 * time.Second):
		t.Fatalf("Timeout waiting for message")
	}
}

func TestNatsIntegration_Metrics(t *testing.T) {
	integration := NewNatsIntegration()
	if integration.publishedCounter == nil {
		t.Error("publishedCounter is nil")
	}
	if integration.receivedCounter == nil {
		t.Error("receivedCounter is nil")
	}
}
