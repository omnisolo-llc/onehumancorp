package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/db"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

type mockStreamMeshEventsServer struct {
	grpc.ServerStream
	ctx      context.Context
	events   []*pb.MeshEvent
	cancelFn context.CancelFunc
}

func (m *mockStreamMeshEventsServer) Send(event *pb.MeshEvent) error {
	m.events = append(m.events, event)
	if len(m.events) >= 1 {
		// Cancel context to exit the stream loop after receiving an event
		m.cancelFn()
	}
	return nil
}

func (m *mockStreamMeshEventsServer) Context() context.Context {
	return m.ctx
}

type mockDiscoverAgentsServer struct {
	grpc.ServerStream
	ctx      context.Context
	caps     []*pb.AgentCapabilities
	cancelFn context.CancelFunc
}

func (m *mockDiscoverAgentsServer) Send(capData *pb.AgentCapabilities) error {
	m.caps = append(m.caps, capData)
	if len(m.caps) >= 1 {
		// Cancel context to exit the stream loop after receiving an event
		m.cancelFn()
	}
	return nil
}

func (m *mockDiscoverAgentsServer) Context() context.Context {
	return m.ctx
}

func TestHubServiceServer_MeshRPCs(t *testing.T) {
	provider := db.NewTestProvider()
	tm := NewTaskManager(provider, nil)
	hub := NewHub()
	hub.taskManager = tm

	srv := NewHubServiceServer(hub)

	// Test AdvertiseCapabilities
	caps := &pb.AgentCapabilities{
		AgentId:       proto.String("agent-1"),
		SupportedSkills: []string{"skill-1", "skill-2"},
		MaxConcurrentTasks: proto.Int32(5),
	}
	resp, err := srv.AdvertiseCapabilities(context.Background(), caps)
	if err != nil {
		t.Fatalf("AdvertiseCapabilities failed: %v", err)
	}
	if !resp.GetSuccess() {
		t.Fatalf("expected success to be true")
	}

	// Test DiscoverAgents
	ctx, cancel := context.WithCancel(context.Background())
	discoverStream := &mockDiscoverAgentsServer{
		ctx:      ctx,
		cancelFn: cancel,
	}

	// We start DiscoverAgents in a goroutine because it blocks until context is cancelled
	errCh := make(chan error)
	go func() {
		errCh <- srv.DiscoverAgents(&pb.Query{}, discoverStream)
	}()

	// Give the subscription a little time to set up
	time.Sleep(50 * time.Millisecond)

	// Advertise again to trigger the stream
	_, err = srv.AdvertiseCapabilities(context.Background(), caps)
	if err != nil {
		t.Fatalf("second AdvertiseCapabilities failed: %v", err)
	}

	if err := <-errCh; err != nil && err != context.Canceled {
		t.Fatalf("DiscoverAgents returned unexpected error: %v", err)
	}

	if len(discoverStream.caps) == 0 {
		t.Fatalf("expected to receive at least 1 capability via DiscoverAgents")
	}
	if discoverStream.caps[0].GetAgentId() != "agent-1" {
		t.Errorf("expected agent-1, got %v", discoverStream.caps[0].GetAgentId())
	}

	// Test StreamMeshEvents
	ctxEvents, cancelEvents := context.WithCancel(context.Background())
	eventsStream := &mockStreamMeshEventsServer{
		ctx:      ctxEvents,
		cancelFn: cancelEvents,
	}

	go func() {
		errCh <- srv.StreamMeshEvents(&pb.EventStreamRequest{Topic: proto.String("test-topic")}, eventsStream)
	}()

	time.Sleep(50 * time.Millisecond)

	// Broadcast an event
	err = tm.meshTransport.BroadcastMeshEvent(context.Background(), "test-topic", []byte("hello mesh"))
	if err != nil {
		t.Fatalf("BroadcastMeshEvent failed: %v", err)
	}

	if err := <-errCh; err != nil && err != context.Canceled {
		t.Fatalf("StreamMeshEvents returned unexpected error: %v", err)
	}

	if len(eventsStream.events) == 0 {
		t.Fatalf("expected to receive at least 1 event via StreamMeshEvents")
	}
	if string(eventsStream.events[0].GetPayload()) != "hello mesh" {
		t.Errorf("expected payload 'hello mesh', got %v", string(eventsStream.events[0].GetPayload()))
	}
}
