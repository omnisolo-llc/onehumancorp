package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/centrifugal/centrifuge"
)

func TestTeammateMesh(t *testing.T) {
	node, err := centrifuge.New(centrifuge.Config{})
	if err != nil {
		t.Fatalf("failed to create centrifuge node: %v", err)
	}
	defer node.Shutdown(context.Background())

	err = node.Run()
	if err != nil {
		t.Fatalf("failed to run centrifuge node: %v", err)
	}

	mesh := NewTeammateMesh(node)

	msg := MeshMessage{
		SenderID:  "agent-1",
		Role:      "SWE",
		Content:   "Testing mesh",
		Timestamp: time.Now(),
	}

	err = mesh.Broadcast(context.Background(), "room-1", msg)
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}
}
