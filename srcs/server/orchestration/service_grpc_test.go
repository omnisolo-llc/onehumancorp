package orchestration

import (
	"github.com/onehumancorp/mono/srcs/server/db"
	"google.golang.org/grpc/status"
	"google.golang.org/grpc/codes"
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
	"google.golang.org/protobuf/proto"
)

func TestRegisterAgentViaGRPC(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub)

	req := pb.RegisterAgentRequest_builder{
		Agent: pb.Agent_builder{
			Id:             proto.String("test-agent"),
			Name:           proto.String("Test Agent"),
			Role:           proto.String("QA_ENGINEER"),
			OrganizationId: proto.String("org-1"),
			Status:         proto.String("ACTIVE"),
		}.Build(),
	}.Build()

	res, err := srv.RegisterAgent(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !res.GetSuccess() {
		t.Errorf("expected success to be true")
	}

	agent, ok := hub.Agent("test-agent")
	if !ok {
		t.Fatalf("agent not registered in hub")
	}
	if agent.Name != "Test Agent" || agent.Role != "QA_ENGINEER" || agent.Status != "ACTIVE" {
		t.Errorf("agent fields mismatch: %+v", agent)
	}
}

func TestOpenMeetingViaGRPC(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	hub.RegisterAgent(Agent{ID: "p1", Name: "P1", Role: "PM", OrganizationID: "org-1"})
	hub.RegisterAgent(Agent{ID: "p2", Name: "P2", Role: "SWE", OrganizationID: "org-1"})
	srv := NewHubServiceServer(hub)

	req := pb.OpenMeetingRequest_builder{
		MeetingId:    proto.String("m-1"),
		Agenda:       proto.String("Test Agenda"),
		Participants: []string{"p1", "p2"},
	}.Build()

	res, err := srv.OpenMeeting(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res.GetId() != "m-1" || res.GetAgenda() != "Test Agenda" || len(res.GetParticipants()) != 2 {
		t.Errorf("meeting response mismatch: %+v", res)
	}

	meeting, ok := hub.Meeting("m-1")
	if !ok {
		t.Fatalf("meeting not registered in hub")
	}
	if meeting.Agenda != "Test Agenda" {
		t.Errorf("meeting agenda mismatch in hub")
	}
}

func TestPublishViaGRPC(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	hub.RegisterAgent(Agent{ID: "a1", Name: "A1", Role: "PM", OrganizationID: "org-1"})
	hub.RegisterAgent(Agent{ID: "a2", Name: "A2", Role: "SWE", OrganizationID: "org-1"})
	srv := NewHubServiceServer(hub)

	req := pb.PublishMessageRequest_builder{
		Message: pb.Message_builder{
			Id:             proto.String("msg-1"),
			FromAgent:      proto.String("a1"),
			ToAgent:        proto.String("a2"),
			Type:           proto.String("task"),
			Content:        proto.String("Do it"),
			OccurredAtUnix: proto.Int64(time.Now().Unix()),
		}.Build(),
	}.Build()

	res, err := srv.Publish(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !res.GetSuccess() {
		t.Errorf("expected success to be true")
	}

	inbox := hub.Inbox("a2")
	if len(inbox) != 1 || inbox[0].Content != "Do it" {
		t.Errorf("message not published to inbox correctly: %+v", inbox)
	}
}

// MockStreamServer implements pb.HubService_StreamMessagesServer
type MockStreamServer struct {
	ctx      context.Context
	messages []*pb.Message
}

func (m *MockStreamServer) Context() context.Context { return m.ctx }
func (m *MockStreamServer) Send(msg *pb.Message) error {
	m.messages = append(m.messages, msg)
	return nil
}
func (m *MockStreamServer) SendHeader(metadata.MD) error { return nil }
func (m *MockStreamServer) SetTrailer(metadata.MD)       {}
func (m *MockStreamServer) SetHeader(metadata.MD) error  { return nil }
func (m *MockStreamServer) SendMsg(m_ interface{}) error { return nil }
func (m *MockStreamServer) RecvMsg(m_ interface{}) error { return nil }

func TestStreamMessagesViaGRPC(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	hub.RegisterAgent(Agent{ID: "a1", Name: "A1", Role: "PM", OrganizationID: "org-1"})
	hub.RegisterAgent(Agent{ID: "a2", Name: "A2", Role: "SWE", OrganizationID: "org-1"})
	srv := NewHubServiceServer(hub)

	// Publish an initial message
	hub.Publish(Message{
		ID:         "msg-1",
		FromAgent:  "a1",
		ToAgent:    "a2",
		Type:       "task",
		Content:    "initial task",
		OccurredAt: time.Now(),
	})

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	stream := &MockStreamServer{ctx: ctx, messages: make([]*pb.Message, 0)}

	go func() {
		// Publish a message while streaming
		time.Sleep(100 * time.Millisecond)
		hub.Publish(Message{
			ID:         "msg-2",
			FromAgent:  "a1",
			ToAgent:    "a2",
			Type:       "task",
			Content:    "new task",
			OccurredAt: time.Now(),
		})
		time.Sleep(50 * time.Millisecond)
		cancel()
	}()

	err := srv.StreamMessages(pb.StreamMessagesRequest_builder{AgentId: proto.String("a2")}.Build(), stream)
	if err != nil && err != context.DeadlineExceeded && err != context.Canceled {
		t.Fatalf("StreamMessages failed: %v", err)
	}

	if len(stream.messages) != 2 {
		t.Errorf("expected 2 messages streamed, got %d", len(stream.messages))
	} else {
		if stream.messages[0].GetContent() != "initial task" {
			t.Errorf("expected msg-1, got %s", stream.messages[0].GetContent())
		}
		if stream.messages[1].GetContent() != "new task" {
			t.Errorf("expected msg-2, got %s", stream.messages[1].GetContent())
		}
	}
}

func TestHubMinimaxAPIKey(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	if hub.MinimaxAPIKey() != "" {
		t.Errorf("expected empty API key initially")
	}
	hub.SetMinimaxAPIKey("test-key")
	if hub.MinimaxAPIKey() != "test-key" {
		t.Errorf("expected 'test-key'")
	}
}

func TestReasonViaMinimaxEmptyKey(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub)

	hub.SetMinimaxAPIKey("")
	req := pb.ReasonRequest_builder{Prompt: proto.String("test prompt")}.Build()
	_, err := srv.Reason(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error due to empty API key")
	}
}

func TestReasonViaMinimaxDummyKey(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub)

	hub.SetMinimaxAPIKey("dummy-key")
	req := pb.ReasonRequest_builder{Prompt: proto.String("test prompt")}.Build()
	_, err := srv.Reason(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error due to invalid API key")
	}
}

func TestRegisterHubServiceCoverage(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := grpc.NewServer()
	RegisterHubService(srv, hub)
}

func TestPublishViaGRPCError(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub)

	req := pb.PublishMessageRequest_builder{
		Message: pb.Message_builder{
			Id:             proto.String("msg-1"),
			FromAgent:      proto.String("missing"),
			ToAgent:        proto.String("missing"),
			Type:           proto.String("task"),
			Content:        proto.String("Do it"),
			OccurredAtUnix: proto.Int64(time.Now().Unix()),
		}.Build(),
	}.Build()

	_, err := srv.Publish(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error")
	}
}

func TestStreamMessagesViaGRPCCancellation(t *testing.T) {
	hub := NewHub()
	defer hub.Close()
	srv := NewHubServiceServer(hub)

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancel immediately
	stream := &MockStreamServer{ctx: ctx, messages: make([]*pb.Message, 0)}

	err := srv.StreamMessages(pb.StreamMessagesRequest_builder{AgentId: proto.String("a2")}.Build(), stream)
	if err != nil && err != context.Canceled {
		t.Fatalf("expected graceful shutdown or context canceled, got %v", err)
	}
}

type errReader int

func (errReader) Read(p []byte) (n int, err error) {
	return 0, errors.New("test read error")
}

func TestMinimaxClientReasonDecodeError(t *testing.T) {
	// Let's not test JSON decode error by writing to http.ResponseWriter
	// because httptest.Server encodes it. But we can send malformed json
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte("{ malformed json }"))
	}))
	defer ts.Close()

	originalURL := MinimaxAPIURL
	MinimaxAPIURL = ts.URL
	defer func() { MinimaxAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	_, err := client.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error on malformed JSON")
	}
}

func TestMinimaxClientReasonInvalidRequest(t *testing.T) {
	client := NewMinimaxClient("valid-key")
	// Using a cancelled context to trigger a request creation or execution error
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err := client.Reason(ctx, "test")
	if err == nil {
		t.Fatalf("expected error on cancelled context")
	}
}



func TestHubServiceServer_AdvertiseCapabilities_NoTransport(t *testing.T) {
	hub := &Hub{} // No centrifuge node attached => MeshTransport == nil
	srv := NewHubServiceServer(hub)

	req := pb.AgentCapabilities_builder{AgentId: proto.String("agent-1")}.Build()

	_, err := srv.AdvertiseCapabilities(context.Background(), req)
	if err == nil {
		t.Fatal("expected error when MeshTransport is not configured")
	}
	if status.Code(err) != codes.Unimplemented {
		t.Fatalf("expected Unimplemented code, got %v", status.Code(err))
	}
}

func TestHubServiceServer_DiscoverAgents_NoTransport(t *testing.T) {
	hub := &Hub{}
	srv := NewHubServiceServer(hub)

	err := srv.DiscoverAgents(&pb.Query{}, &mockDiscoverAgentsServer{ctx: context.Background()})
	if err == nil {
		t.Fatal("expected error when MeshTransport is not configured")
	}
	if status.Code(err) != codes.Unimplemented {
		t.Fatalf("expected Unimplemented code, got %v", status.Code(err))
	}
}

func TestHubServiceServer_StreamMeshEvents_NoTransport(t *testing.T) {
	hub := &Hub{}
	srv := NewHubServiceServer(hub)

	err := srv.StreamMeshEvents(&pb.EventStreamRequest{}, &mockStreamMeshEventsServer{ctx: context.Background()})
	if err == nil {
		t.Fatal("expected error when MeshTransport is not configured")
	}
	if status.Code(err) != codes.Unimplemented {
		t.Fatalf("expected Unimplemented code, got %v", status.Code(err))
	}
}


// Mock for DiscoverAgents
type mockDiscoverAgentsServer struct {
	grpc.ServerStream
	ctx     context.Context
	results []*pb.AgentCapabilities
}

func (m *mockDiscoverAgentsServer) Send(c *pb.AgentCapabilities) error {
	m.results = append(m.results, c)
	return nil
}

func (m *mockDiscoverAgentsServer) Context() context.Context {
	return m.ctx
}

// Mock for StreamMeshEvents
type mockStreamMeshEventsServer struct {
	grpc.ServerStream
	ctx     context.Context
	results []*pb.MeshEvent
}

func (m *mockStreamMeshEventsServer) Send(e *pb.MeshEvent) error {
	m.results = append(m.results, e)
	return nil
}

func (m *mockStreamMeshEventsServer) Context() context.Context {
	return m.ctx
}

func TestHubServiceServer_MeshTransport_Success(t *testing.T) {
	var provider db.Provider = nil
	// defer provider.Close()

	// Create memory mesh transport
	mt := NewMemoryMeshTransport(provider)

	hub := &Hub{
		centrifugeNode: &CentrifugeNode{
			meshTransport: mt,
		},
	}
	srv := NewHubServiceServer(hub)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Test AdvertiseCapabilities
	req := pb.AgentCapabilities_builder{AgentId: proto.String("agent-1"), SupportedSkills: []string{"skill1"}}.Build()
	resp, err := srv.AdvertiseCapabilities(ctx, req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !resp.GetSuccess() {
		t.Fatal("expected success=true")
	}

	// Test DiscoverAgents
	stream := &mockDiscoverAgentsServer{ctx: ctx}

	// Start DiscoverAgents in a goroutine because it blocks
	errCh := make(chan error, 1)
	go func() {
		errCh <- srv.DiscoverAgents(pb.Query_builder{Filter: proto.String("skill1")}.Build(), stream)
	}()

	// Give it a moment to subscribe
	time.Sleep(50 * time.Millisecond)

	// Advertise again to trigger stream
	_, err = srv.AdvertiseCapabilities(ctx, req)
	if err != nil {
		t.Fatalf("failed to advertise capabilities: %v", err)
	}

	time.Sleep(50 * time.Millisecond)
	cancel() // Unblock stream

	if err := <-errCh; err != context.Canceled && err != nil {
		t.Fatalf("DiscoverAgents error: %v", err)
	}

	if len(stream.results) == 0 {
		t.Fatal("expected to discover agent-1")
	}
	if stream.results[0].GetAgentId() != "agent-1" {
		t.Fatalf("expected agent-1, got %v", stream.results[0].GetAgentId())
	}

	// Test StreamMeshEvents
	ctx2, cancel2 := context.WithCancel(context.Background())
	defer cancel2()

	evtStream := &mockStreamMeshEventsServer{ctx: ctx2}
	errCh2 := make(chan error, 1)
	go func() {
		errCh2 <- srv.StreamMeshEvents(pb.EventStreamRequest_builder{Topic: proto.String("test-topic")}.Build(), evtStream)
	}()

	time.Sleep(50 * time.Millisecond)

	mt.BroadcastMeshEvent(ctx2, "test-topic", []byte("hello"))

	time.Sleep(50 * time.Millisecond)
	cancel2()

	if err := <-errCh2; err != context.Canceled && err != nil {
		t.Fatalf("StreamMeshEvents error: %v", err)
	}

	if len(evtStream.results) == 0 {
		t.Fatal("expected to stream event")
	}
	if string(evtStream.results[0].GetPayload()) != "hello" {
		t.Fatalf("expected payload hello, got %v", string(evtStream.results[0].GetPayload()))
	}
}
