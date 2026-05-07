package orchestration

import (
	"context"
	"encoding/json"
	"sync"
	"testing"
	"time"
)

// TestMeshBroadcastStandaloneToCloud verifies a simulated broadcast
// from a standalone client reaching a cloud client via the LocalTeammateMesh.
func TestMeshBroadcastStandaloneToCloud(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "global_task_broadcast"

	// Mock "Cloud Client" subscribing to the mesh
	var wg sync.WaitGroup
	wg.Add(1)

	var receivedPayload []byte

	err := mesh.Subscribe(ctx, channel, func(data []byte) {
		receivedPayload = append([]byte(nil), data...) // copy data
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe cloud client: %v", err)
	}

	// Mock "Standalone Client" publishing to the mesh
	testMsg := MeshMessage{
		AgentID: "standalone_agent_1",
		Action:  "task_created",
		Status:  "pending",
		Payload: json.RawMessage(`{"task_id":"123","detail":"urgent"}`),
		MsgID:   "msg-001",
	}

	msgBytes, err := json.Marshal(testMsg)
	if err != nil {
		t.Fatalf("Failed to marshal message: %v", err)
	}

	// Small delay to ensure subscription is active
	time.Sleep(50 * time.Millisecond)

	err = mesh.Publish(ctx, channel, msgBytes)
	if err != nil {
		t.Fatalf("Failed to publish message: %v", err)
	}

	// Wait for the cloud client to receive the broadcast
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		// Success
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for cloud client to receive broadcast")
	}

	// Verify the payload received by the cloud client
	var receivedMsg MeshMessage
	err = json.Unmarshal(receivedPayload, &receivedMsg)
	if err != nil {
		t.Fatalf("Failed to unmarshal received payload: %v", err)
	}

	if receivedMsg.AgentID != "standalone_agent_1" {
		t.Errorf("Expected AgentID 'standalone_agent_1', got '%s'", receivedMsg.AgentID)
	}
	if receivedMsg.Action != "task_created" {
		t.Errorf("Expected Action 'task_created', got '%s'", receivedMsg.Action)
	}
	if string(receivedMsg.Payload) != `{"task_id":"123","detail":"urgent"}` {
		t.Errorf("Unexpected payload content: %s", string(receivedMsg.Payload))
	}
}
