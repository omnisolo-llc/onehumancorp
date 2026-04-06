package orchestration

import (
	"context"
	"strings"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/db"
	"google.golang.org/grpc"
)

type mockStreamMeshEventsServer struct {
	grpc.ServerStream
	ctx     context.Context
	events  []*pb.MeshEvent
	done    chan struct{}
}

func (m *mockStreamMeshEventsServer) Context() context.Context {
	return m.ctx
}

func (m *mockStreamMeshEventsServer) Send(event *pb.MeshEvent) error {
	m.events = append(m.events, event)
	if len(m.events) >= 1 { // allow exactly 1 for test
		m.done <- struct{}{}
	}
	return nil
}

type mockDiscoverAgentsServer struct {
	grpc.ServerStream
	ctx     context.Context
	caps    []*pb.AgentCapabilities
	done    chan struct{}
}

func (m *mockDiscoverAgentsServer) Context() context.Context {
	return m.ctx
}

func (m *mockDiscoverAgentsServer) Send(caps *pb.AgentCapabilities) error {
	m.caps = append(m.caps, caps)
	if len(m.caps) >= 1 { // allow exactly 1 for test
		m.done <- struct{}{}
	}
	return nil
}

func TestHubService_MeshTransportEndpoints(t *testing.T) {
	ResetCircuitBreakerForTest()
	hub := NewHub()

	p := db.NewTestProvider(t)
	transport := NewMemoryMeshTransport(p)

	cn, _ := NewCentrifugeNode()
	cn.SetMeshTransport(transport)
	hub.SetCentrifugeNode(cn)

	s := NewHubServiceServer(hub)

	// Test AdvertiseCapabilities
	caps := &pb.AgentCapabilities{
		AgentId:            "agent-xyz",
		SupportedSkills:    []string{"code", "test"},
		MaxConcurrentTasks: 5,
	}

	resp, err := s.AdvertiseCapabilities(context.Background(), caps)
	if err != nil {
		t.Fatalf("AdvertiseCapabilities failed: %v", err)
	}
	if !resp.GetSuccess() {
		t.Errorf("Expected AdvertiseCapabilities to be successful")
	}

	// Test StreamMeshEvents
	ctx, cancel := context.WithCancel(context.Background())
	streamEvents := &mockStreamMeshEventsServer{
		ctx:  ctx,
		done: make(chan struct{}, 1),
	}

	go func() {
		s.StreamMeshEvents(&pb.EventStreamRequest{Topic: "sys-events"}, streamEvents)
	}()

	time.Sleep(100 * time.Millisecond) // Give subscriber time to setup

	// Broadcast an event to trigger StreamMeshEvents
	transport.BroadcastMeshEvent(context.Background(), "sys-events", []byte("hello-mesh"))

	select {
	case <-streamEvents.done:
		if len(streamEvents.events) != 1 {
			t.Errorf("Expected 1 mesh event, got %d", len(streamEvents.events))
		} else {
			if string(streamEvents.events[0].GetPayload()) != "hello-mesh" {
				t.Errorf("Expected payload 'hello-mesh', got %s", string(streamEvents.events[0].GetPayload()))
			}
			if streamEvents.events[0].GetTopic() != "sys-events" {
				t.Errorf("Expected topic 'sys-events', got %s", streamEvents.events[0].GetTopic())
			}
		}
	case <-time.After(2 * time.Second):
		t.Errorf("Timed out waiting for StreamMeshEvents")
	}
	cancel()

	// Test DiscoverAgents
	ctx2, cancel2 := context.WithCancel(context.Background())
	streamDiscover := &mockDiscoverAgentsServer{
		ctx:  ctx2,
		done: make(chan struct{}, 1),
	}

	go func() {
		s.DiscoverAgents(&pb.Query{Filter: "code"}, streamDiscover)
	}()

	time.Sleep(100 * time.Millisecond) // Give subscriber time to setup

	// Advertise capabilities to trigger DiscoverAgents
	s.AdvertiseCapabilities(context.Background(), caps)

	select {
	case <-streamDiscover.done:
		if len(streamDiscover.caps) != 1 {
			t.Errorf("Expected 1 capability result, got %d", len(streamDiscover.caps))
		} else {
			if streamDiscover.caps[0].GetAgentId() != "agent-xyz" {
				t.Errorf("Expected agent-xyz, got %s", streamDiscover.caps[0].GetAgentId())
			}
		}
	case <-time.After(2 * time.Second):
		t.Errorf("Timed out waiting for DiscoverAgents")
	}
	cancel2()

}
