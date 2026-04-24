package interop

import (
	"os"
	pb "github.com/onehumancorp/mono/src/proto"
	"context"
	"sync"
	"testing"
	"time"
)



func TestMeshPayloadValidation(t *testing.T) {
	mesh := NewTeammateMeshWithClient(nil)
	ctx := context.Background()

	tests := []struct {
		name    string
		payload *pb.MeshMessage
		wantErr bool
	}{
		{
			name:    "valid payload",
			payload: &pb.MeshMessage{AgentId: "spiffe://onehumancorp.io/agent/123", Action: "CREATE", Status: "PENDING"},
			wantErr: false,
		},
		{
			name:    "missing agent_id",
			payload: &pb.MeshMessage{Action: "CREATE", Status: "PENDING"},
			wantErr: true,
		},
		{
			name:    "missing action",
			payload: &pb.MeshMessage{AgentId: "spiffe://onehumancorp.io/agent/123", Status: "PENDING"},
			wantErr: true,
		},
		{
			name:    "missing status",
			payload: &pb.MeshMessage{AgentId: "spiffe://onehumancorp.io/agent/123", Action: "CREATE"},
			wantErr: true,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			err := mesh.Publish(ctx, "test_channel", tc.payload)
			if (err != nil) != tc.wantErr {
				t.Errorf("Publish() error = %v, wantErr %v", err, tc.wantErr)
			}
		})
	}
}

func TestMemoryMesh_PubSub(t *testing.T) {
	mesh := NewTeammateMeshWithClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	channel := "test_channel"

	sub, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	msg := &pb.MeshMessage{AgentId: "test_agent", Action: "test", Status: "ok"}
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case rcv := <-sub:
		if rcv.AgentId != msg.AgentId || rcv.Action != msg.Action || rcv.Status != msg.Status {
			t.Errorf("expected %v, got %v", msg, rcv)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timed out waiting for message")
	}
}

func TestMemoryMesh_MultipleSubscribers(t *testing.T) {
	mesh := NewTeammateMeshWithClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	channel := "multi_channel"

	sub1, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe sub1: %v", err)
	}

	sub2, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe sub2: %v", err)
	}

	msg := &pb.MeshMessage{AgentId: "test_agent", Action: "test", Status: "ok"}
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		select {
		case rcv := <-sub1:
			if rcv.AgentId != msg.AgentId || rcv.Action != msg.Action || rcv.Status != msg.Status {
				t.Errorf("sub1 expected %v, got %v", msg, rcv)
			}
		case <-time.After(1 * time.Second):
			t.Error("sub1 timed out waiting for message")
		}
	}()

	go func() {
		defer wg.Done()
		select {
		case rcv := <-sub2:
			if rcv.AgentId != msg.AgentId || rcv.Action != msg.Action || rcv.Status != msg.Status {
				t.Errorf("sub2 expected %v, got %v", msg, rcv)
			}
		case <-time.After(1 * time.Second):
			t.Error("sub2 timed out waiting for message")
		}
	}()

	wg.Wait()
}
