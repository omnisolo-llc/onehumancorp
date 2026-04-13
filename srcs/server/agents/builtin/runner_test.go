package builtin

import (
	"context"
	"net"
	"sync"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

type runnerTestHub struct {
	mu        sync.Mutex
	inbox     map[string][]HubMessage
	subs      map[string][]chan struct{}
	published []HubMessage
	phases    []string
	agents    map[string]HubAgent
}

func newRunnerTestHub() *runnerTestHub {
	return &runnerTestHub{
		inbox:  make(map[string][]HubMessage),
		subs:   make(map[string][]chan struct{}),
		agents: make(map[string]HubAgent),
	}
}

func (h *runnerTestHub) RegisterAgent(agent HubAgent) {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.agents[agent.ID] = agent
}

func (h *runnerTestHub) ReportWorkerState(state *pb.WorkerState) {
	h.mu.Lock()
	h.phases = append(h.phases, NormalizeWorkerPhase(state.GetPhase()))
	subs := append([]chan struct{}(nil), h.subs[state.GetAgentId()]...)
	h.mu.Unlock()
	for _, ch := range subs {
		select {
		case ch <- struct{}{}:
		default:
		}
	}
}

func (h *runnerTestHub) Subscribe(agentID string) (<-chan struct{}, func()) {
	ch := make(chan struct{}, 1)
	h.mu.Lock()
	h.subs[agentID] = append(h.subs[agentID], ch)
	h.mu.Unlock()
	return ch, func() {
		h.mu.Lock()
		defer h.mu.Unlock()
		subs := h.subs[agentID]
		for i, sub := range subs {
			if sub == ch {
				h.subs[agentID] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}
}

func (h *runnerTestHub) Inbox(agentID string) []HubMessage {
	h.mu.Lock()
	defer h.mu.Unlock()
	msgs := append([]HubMessage(nil), h.inbox[agentID]...)
	delete(h.inbox, agentID)
	return msgs
}

func (h *runnerTestHub) Publish(msg HubMessage) error {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.published = append(h.published, msg)
	return nil
}

func (h *runnerTestHub) deliver(agentID string, msg HubMessage) {
	h.mu.Lock()
	h.inbox[agentID] = append(h.inbox[agentID], msg)
	subs := append([]chan struct{}(nil), h.subs[agentID]...)
	h.mu.Unlock()
	for _, ch := range subs {
		select {
		case ch <- struct{}{}:
		default:
		}
	}
}

func (h *runnerTestHub) publishedMessages() []HubMessage {
	h.mu.Lock()
	defer h.mu.Unlock()
	return append([]HubMessage(nil), h.published...)
}

func (h *runnerTestHub) hasPhase(want string) bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	for _, phase := range h.phases {
		if phase == want {
			return true
		}
	}
	return false
}

type runnerTestAgentService struct {
	agentservicepb.UnimplementedAgentServiceServer
}

func (s *runnerTestAgentService) RunTask(req *agentservicepb.RunTaskRequest, stream agentservicepb.AgentService_RunTaskServer) error {
	toolCall := agentservicepb.EventType_TOOL_CALL
	if err := stream.Send(agentservicepb.RunTaskEvent_builder{Type: &toolCall}.Build()); err != nil {
		return err
	}
	complete := agentservicepb.EventType_TASK_COMPLETE
	return stream.Send(agentservicepb.RunTaskEvent_builder{
		Type:    &complete,
		Content: proto.String("done:" + req.GetTask()),
	}.Build())
}

func (s *runnerTestAgentService) Ping(context.Context, *agentservicepb.PingRequest) (*agentservicepb.PingResponse, error) {
	return agentservicepb.PingResponse_builder{AgentId: proto.String("runner-test-agent")}.Build(), nil
}

func (s *runnerTestAgentService) DispatchToSubAgent(context.Context, *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	return agentservicepb.SubAgentResponse_builder{Result: proto.String("ok")}.Build(), nil
}

func startRunnerTestServer(t *testing.T) (string, func()) {
	t.Helper()
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("listen: %v", err)
	}
	server := grpc.NewServer()
	agentservicepb.RegisterAgentServiceServer(server, &runnerTestAgentService{})
	go func() {
		_ = server.Serve(listener)
	}()
	return listener.Addr().String(), func() {
		server.Stop()
		_ = listener.Close()
	}
}

func TestRunnerProcessesProtobufTaskAssignmentAndPublishesResult(t *testing.T) {
	address, stop := startRunnerTestServer(t)
	defer stop()

	hub := newRunnerTestHub()
	runner := NewRunner(hub, HubAgent{
		ID:      "agent-1",
		Name:    "Builder",
		Role:    "SOFTWARE_ENGINEER",
		Region:  "process",
		Managed: true,
		Status:  HubStatusIdle,
	}, address)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	done := make(chan struct{})
	go func() {
		defer close(done)
		runner.Start(ctx)
	}()

	waitForRunnerTest(t, func() bool { return hub.hasPhase("READY") })
	payload, err := EncodeTaskAssignment(pb.TaskAssignment_builder{
		IssueId:   proto.String("issue-1"),
		IssueName: proto.String("Fix failing test"),
		Directive: proto.String("Implement the fix"),
	}.Build())
	if err != nil {
		t.Fatalf("EncodeTaskAssignment: %v", err)
	}

	hub.deliver("agent-1", HubMessage{
		ID:        "msg-1",
		FromAgent: "manager-1",
		ToAgent:   "agent-1",
		Type:      "TaskAssignment",
		Content:   payload,
	})

	waitForRunnerTest(t, func() bool { return len(hub.publishedMessages()) == 1 })
	cancel()
	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("runner did not stop")
	}

	published := hub.publishedMessages()
	if len(published) != 1 {
		t.Fatalf("expected one published result, got %d", len(published))
	}
	if published[0].Type != "TaskResult" {
		t.Fatalf("expected TaskResult message, got %q", published[0].Type)
	}
	result, err := DecodeTaskResultEnvelope(published[0].Content)
	if err != nil {
		t.Fatalf("DecodeTaskResultEnvelope: %v", err)
	}
	if result.GetTaskId() != "issue-1" {
		t.Fatalf("expected task id issue-1, got %q", result.GetTaskId())
	}
	if result.GetStatus() != pb.TaskStatus_TASK_STATUS_COMPLETED {
		t.Fatalf("expected completed task status, got %s", result.GetStatus().String())
	}
	if result.GetToolUseCount() != 1 {
		t.Fatalf("expected one tool use event, got %d", result.GetToolUseCount())
	}
	if result.GetResult() == "" {
		t.Fatal("expected task result content")
	}
	for _, phase := range []string{"STARTING", "READY", "BUSY"} {
		if !hub.hasPhase(phase) {
			t.Fatalf("expected worker phase %s to be reported", phase)
		}
	}
}

func TestDecodeTaskAssignmentSupportsLegacyJSON(t *testing.T) {
	assignment, err := DecodeTaskAssignment(`{"issue_id":"issue-9","issue_name":"Legacy","directive":"Use fallback"}`)
	if err != nil {
		t.Fatalf("DecodeTaskAssignment: %v", err)
	}
	if assignment.GetIssueId() != "issue-9" {
		t.Fatalf("expected issue-9, got %q", assignment.GetIssueId())
	}
}

func waitForRunnerTest(t *testing.T, condition func() bool) {
	t.Helper()
	deadline := time.Now().Add(2 * time.Second)
	for time.Now().Before(deadline) {
		if condition() {
			return
		}
		time.Sleep(10 * time.Millisecond)
	}
	t.Fatal("condition not met before timeout")
}
