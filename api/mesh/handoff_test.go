package mesh

import (
	"context"
	"testing"
	"time"
)

func TestStateHandoffManager(t *testing.T) {
	cloudMesh := NewTeammateMesh(nil) // Local fallback mode for tests
	localMesh := NewTeammateMesh(nil) // Local fallback mode for tests

	manager := NewStateHandoffManager(cloudMesh, localMesh)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Test HandoffToCloud
	cloudCh := make(chan MeshMessage, 1)
	cloudMesh.Subscribe(ctx, "handoff:cloud", func(msg MeshMessage) {
		cloudCh <- msg
	})

	err := manager.HandoffToCloud(ctx, "test_agent", "handoff:cloud", map[string]string{"status": "escalated"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case msg := <-cloudCh:
		if msg.SenderID != "test_agent" {
			t.Errorf("expected sender 'test_agent', got '%s'", msg.SenderID)
		}
	case <-time.After(1 * time.Second):
		t.Error("timeout waiting for cloud message")
	}

	// Test HandoffToLocal
	localCh := make(chan MeshMessage, 1)
	localMesh.Subscribe(ctx, "handoff:local", func(msg MeshMessage) {
		localCh <- msg
	})

	err = manager.HandoffToLocal(ctx, "test_agent", "handoff:local", map[string]string{"status": "downgraded"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case msg := <-localCh:
		if msg.SenderID != "test_agent" {
			t.Errorf("expected sender 'test_agent', got '%s'", msg.SenderID)
		}
	case <-time.After(1 * time.Second):
		t.Error("timeout waiting for local message")
	}
}

func TestStateHandoffManager_Unconfigured(t *testing.T) {
	manager := NewStateHandoffManager(nil, nil)
	ctx := context.Background()

	err := manager.HandoffToCloud(ctx, "test_agent", "handoff:cloud", nil)
	if err == nil {
		t.Error("expected error when cloud mesh is not configured")
	}

	err = manager.HandoffToLocal(ctx, "test_agent", "handoff:local", nil)
	if err == nil {
		t.Error("expected error when local mesh is not configured")
	}
}
