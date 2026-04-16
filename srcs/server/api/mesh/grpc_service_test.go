package mesh

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/server/api/proto"
	"github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
	"google.golang.org/grpc"
)

type mockStream struct {
	grpc.ServerStream
	ctx     context.Context
	updates []*pb.StateUpdate
}

func (m *mockStream) Context() context.Context {
	return m.ctx
}

func (m *mockStream) Send(update *pb.StateUpdate) error {
	m.updates = append(m.updates, update)
	return nil
}

func TestCoordinationServer_AcquireLock(t *testing.T) {
	tm := mesh.NewLocalMesh()
	server := NewCoordinationServer(tm)

	req := &pb.LockRequest{
		AgentId:        "agent-1",
		TargetResource: "resource-1",
		TtlSeconds:     10,
	}

	resp, err := server.AcquireLock(context.Background(), req)
	if err != nil {
		t.Fatalf("AcquireLock failed: %v", err)
	}

	if !resp.Acquired {
		t.Fatalf("Expected lock to be acquired")
	}

	// Try to acquire again
	resp, err = server.AcquireLock(context.Background(), req)
	if err != nil {
		t.Fatalf("AcquireLock failed: %v", err)
	}

	if resp.Acquired {
		t.Fatalf("Expected lock to not be acquired")
	}
}

func TestCoordinationServer_StreamAgentState(t *testing.T) {
	tm := mesh.NewLocalMesh()
	server := NewCoordinationServer(tm)

	ctx, cancel := context.WithCancel(context.Background())

	stream := &mockStream{
		ctx: ctx,
	}

	go func() {
		// Wait a bit, then publish
		time.Sleep(50 * time.Millisecond)
		update := pb.StateUpdate{
			AgentId:        "agent-1",
			NewStatus:      "WORKING",
			CurrentMission: "mission-1",
		}
		data, _ := json.Marshal(update)
		_ = tm.Publish(context.Background(), "ohc.mesh.agent.status", data)

		// Wait a bit, then cancel to end the stream
		time.Sleep(50 * time.Millisecond)
		cancel()
	}()

	err := server.StreamAgentState(&pb.StateStreamRequest{}, stream)
	if err != nil {
		t.Fatalf("StreamAgentState failed: %v", err)
	}

	if len(stream.updates) != 1 {
		t.Fatalf("Expected 1 update, got %d", len(stream.updates))
	}

	if stream.updates[0].AgentId != "agent-1" || stream.updates[0].NewStatus != "WORKING" {
		t.Fatalf("Unexpected update data: %+v", stream.updates[0])
	}
}


func TestCoordinationServer_ReleaseLock(t *testing.T) {
	tm := mesh.NewLocalMesh()
	server := NewCoordinationServer(tm)

	reqLock := &pb.LockRequest{
		AgentId:        "agent-1",
		TargetResource: "resource-1",
		TtlSeconds:     10,
	}

	_, _ = server.AcquireLock(context.Background(), reqLock)

	reqRelease := &pb.ReleaseRequest{
		AgentId:        "agent-1",
		TargetResource: "resource-1",
	}

	resp, err := server.ReleaseLock(context.Background(), reqRelease)
	if err != nil {
		t.Fatalf("ReleaseLock failed: %v", err)
	}

	if !resp.Success {
		t.Fatalf("Expected lock to be released")
	}

	// Try to acquire again
	respLock, err := server.AcquireLock(context.Background(), reqLock)
	if err != nil {
		t.Fatalf("AcquireLock failed: %v", err)
	}

	if !respLock.Acquired {
		t.Fatalf("Expected lock to be acquired after release")
	}
}
