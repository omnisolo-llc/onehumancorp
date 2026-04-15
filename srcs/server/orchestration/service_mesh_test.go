package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/grpc/metadata"
)

// MockStream for DiscoverAgents
type mockDiscoverAgentsStream struct {
	ctx      context.Context
	sent     []*pb.AgentCapabilities
	sendErr  error
	recvErr  error
}

func (m *mockDiscoverAgentsStream) Send(caps *pb.AgentCapabilities) error {
	if m.sendErr != nil {
		return m.sendErr
	}
	m.sent = append(m.sent, caps)
	return nil
}

func (m *mockDiscoverAgentsStream) SetHeader(metadata.MD) error  { return nil }
func (m *mockDiscoverAgentsStream) SendHeader(metadata.MD) error { return nil }
func (m *mockDiscoverAgentsStream) SetTrailer(metadata.MD)       {}
func (m *mockDiscoverAgentsStream) Context() context.Context     { return m.ctx }
func (m *mockDiscoverAgentsStream) SendMsg(m_ interface{}) error { return nil }
func (m *mockDiscoverAgentsStream) RecvMsg(m_ interface{}) error { return nil }

// MockStream for StreamMeshEvents
type mockStreamMeshEventsStream struct {
	ctx      context.Context
	sent     []*pb.MeshEvent
	sendErr  error
	recvErr  error
}

func (m *mockStreamMeshEventsStream) Send(event *pb.MeshEvent) error {
	if m.sendErr != nil {
		return m.sendErr
	}
	m.sent = append(m.sent, event)
	return nil
}

func (m *mockStreamMeshEventsStream) SetHeader(metadata.MD) error  { return nil }
func (m *mockStreamMeshEventsStream) SendHeader(metadata.MD) error { return nil }
func (m *mockStreamMeshEventsStream) SetTrailer(metadata.MD)       {}
func (m *mockStreamMeshEventsStream) Context() context.Context     { return m.ctx }
func (m *mockStreamMeshEventsStream) SendMsg(m_ interface{}) error { return nil }
func (m *mockStreamMeshEventsStream) RecvMsg(m_ interface{}) error { return nil }


func TestHubServiceServer_AdvertiseCapabilities(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	cn, _ := NewCentrifugeNode()
	mt := NewMemoryMeshTransport(nil)
	cn.SetMeshTransport(mt)
	hub.SetCentrifugeNode(cn)

	srv := NewHubServiceServer(hub, mt)

	// Test missing agent ID
	req := &pb.AgentCapabilities{}
	_, err := srv.AdvertiseCapabilities(context.Background(), req)
	if err == nil {
		t.Errorf("expected error for missing agent_id")
	}

	// Test success
	req = &pb.AgentCapabilities{
		AgentId: "spiffe://onehumancorp.io/agent/test-agent",
	}
	resp, err := srv.AdvertiseCapabilities(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !resp.Success {
		t.Errorf("expected success")
	}
}

func TestHubServiceServer_DiscoverAgents(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	cn, _ := NewCentrifugeNode()
	mt := NewMemoryMeshTransport(nil)
	cn.SetMeshTransport(mt)
	hub.SetCentrifugeNode(cn)

	srv := NewHubServiceServer(hub, mt)

	// Background worker to publish capability
	go func() {
		time.Sleep(10 * time.Millisecond)
		mt.AdvertiseCapabilities(context.Background(), pb.AgentCapabilities{
			AgentId: "spiffe://onehumancorp.io/agent/123",
		})
	}()

	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	stream := &mockDiscoverAgentsStream{ctx: ctx}
	err := srv.DiscoverAgents(&pb.Query{}, stream)

	// Should return nil when context is done
	if err != nil && !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(stream.sent) == 0 {
		t.Errorf("expected to receive capabilities, got none")
	} else if stream.sent[0].AgentId != "spiffe://onehumancorp.io/agent/123" {
		t.Errorf("expected spiffe://onehumancorp.io/agent/123, got %v", stream.sent[0].AgentId)
	}
}

func TestHubServiceServer_StreamMeshEvents(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	cn, _ := NewCentrifugeNode()
	mt := NewMemoryMeshTransport(nil)
	cn.SetMeshTransport(mt)
	hub.SetCentrifugeNode(cn)

	srv := NewHubServiceServer(hub, mt)

	// Test missing topic
	req := &pb.EventStreamRequest{}
	err := srv.StreamMeshEvents(req, &mockStreamMeshEventsStream{ctx: context.Background()})
	if err == nil {
		t.Errorf("expected error for missing topic")
	}

	// Background worker to publish event
	go func() {
		time.Sleep(10 * time.Millisecond)
		mt.BroadcastMeshEvent(context.Background(), "tasks", []byte("test-payload"))
	}()

	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	stream := &mockStreamMeshEventsStream{ctx: ctx}
	req = &pb.EventStreamRequest{Topic: "tasks"}
	err = srv.StreamMeshEvents(req, stream)

	// Should return nil when context is done
	if err != nil && !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(stream.sent) == 0 {
		t.Errorf("expected to receive events, got none")
	} else if string(stream.sent[0].Payload) != "test-payload" {
		t.Errorf("expected test-payload, got %v", string(stream.sent[0].Payload))
	}
}

func TestHubServiceServer_Errors(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub, nil)

	// No CentrifugeNode
	_, err := srv.AdvertiseCapabilities(context.Background(), &pb.AgentCapabilities{AgentId: "test"})
	if err == nil {
		t.Errorf("expected error for missing CentrifugeNode")
	}

	err = srv.DiscoverAgents(&pb.Query{}, &mockDiscoverAgentsStream{ctx: context.Background()})
	if err == nil {
		t.Errorf("expected error for missing CentrifugeNode")
	}

	err = srv.StreamMeshEvents(&pb.EventStreamRequest{Topic: "test"}, &mockStreamMeshEventsStream{ctx: context.Background()})
	if err == nil {
		t.Errorf("expected error for missing CentrifugeNode")
	}

	// With CentrifugeNode, but missing MeshTransport
	cn, _ := NewCentrifugeNode()
	hub.SetCentrifugeNode(cn)

	_, err = srv.AdvertiseCapabilities(context.Background(), &pb.AgentCapabilities{AgentId: "test"})
	if err == nil {
		t.Errorf("expected error for missing MeshTransport")
	}

	err = srv.DiscoverAgents(&pb.Query{}, &mockDiscoverAgentsStream{ctx: context.Background()})
	if err == nil {
		t.Errorf("expected error for missing MeshTransport")
	}

	err = srv.StreamMeshEvents(&pb.EventStreamRequest{Topic: "test"}, &mockStreamMeshEventsStream{ctx: context.Background()})
	if err == nil {
		t.Errorf("expected error for missing MeshTransport")
	}
}
