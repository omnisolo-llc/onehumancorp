package orchestration_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestV2TeammateMesh(t *testing.T) {
	// Create a centrifuge node. In tests, we can just use NewCentrifugeNode.
	cn, err := orchestration.NewCentrifugeNode()
	if err != nil {
		t.Fatalf("failed to create centrifuge node: %v", err)
	}
	defer cn.Close()

	mesh := orchestration.NewV2TeammateMesh(cn)

	msg := orchestration.MeshMessage{
		AgentID: "agent-1",
		TaskID:  "task-1",
		Action:  "CLAIM",
		Status:  "IN_PROGRESS",
	}

	err = mesh.Broadcast(context.Background(), msg)
	if err != nil {
		t.Fatalf("failed to broadcast: %v", err)
	}
}
