package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestV2TeammateMesh_BroadcastTask(t *testing.T) {
	node, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("Failed to create node: %v", err)
	}
	mesh := NewV2TeammateMesh(node)

	task := Task{
		AgentID: "agent-1",
		Action:  "CLAIM",
		Status:  "IN_PROGRESS",
		TaskID:  "task-1",
	}

	err = mesh.BroadcastTask(context.Background(), task)
	if err != nil {
		t.Fatalf("BroadcastTask failed: %v", err)
	}
}

func TestV2TeammateMesh_BroadcastCoordination(t *testing.T) {
	node, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("Failed to create node: %v", err)
	}
	mesh := NewV2TeammateMesh(node)

	msg := MeshMessage{
		AgentID:   "agent-1",
		Action:    "COORDINATE",
		Status:    "OK",
		Timestamp: time.Now(),
		Content:   "hello",
	}

	err = mesh.BroadcastCoordination(context.Background(), msg)
	if err != nil {
		t.Fatalf("BroadcastCoordination failed: %v", err)
	}
}
