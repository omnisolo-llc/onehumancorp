package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"
)

type FieldMatchFilter struct {
	Field    string
	Expected string
}

func (f *FieldMatchFilter) Evaluate(payload []byte) bool {
	var data map[string]interface{}
	if err := json.Unmarshal(payload, &data); err != nil {
		return false
	}
	if val, ok := data[f.Field]; ok {
		if valStr, isStr := val.(string); isStr {
			return valStr == f.Expected
		}
	}
	return false
}

func TestMemoryMeshTransport_MeshEventsWithFilter(t *testing.T) {
	mesh := NewMemoryMeshTransport(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	topic := "tasks"
	filter := &FieldMatchFilter{Field: "agent_id", Expected: "agent-123"}

	ch, err := mesh.SubscribeMeshEventsWithFilter(ctx, topic, filter)
	if err != nil {
		t.Fatalf("Failed to subscribe with filter: %v", err)
	}

	payload1 := []byte(`{"task_id": "1", "agent_id": "agent-456"}`) // Should be filtered out
	payload2 := []byte(`{"task_id": "2", "agent_id": "agent-123"}`) // Should pass

	_ = mesh.BroadcastMeshEvent(ctx, topic, payload1)
	_ = mesh.BroadcastMeshEvent(ctx, topic, payload2)

	select {
	case msg := <-ch:
		var data map[string]interface{}
		_ = json.Unmarshal(msg, &data)
		if data["task_id"] != "2" {
			t.Errorf("Expected task_id 2, got %v", data["task_id"])
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for filtered event")
	}

	// Verify no more messages
	select {
	case msg := <-ch:
		t.Fatalf("Received unexpected message: %s", msg)
	case <-time.After(100 * time.Millisecond):
		// Success
	}
}
