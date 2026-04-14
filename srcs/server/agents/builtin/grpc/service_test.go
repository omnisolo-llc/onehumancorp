package agentgrpc_test

import (
	"context"
	"net"
	"testing"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	agentgrpc "github.com/onehumancorp/mono/srcs/server/agents/builtin/grpc"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/test/bufconn"
)

const bufSize = 1024 * 1024

// startTestServer creates an in-memory gRPC server wired to a bufconn listener.
// It returns the client connection and a cleanup function.
func startTestServer(t *testing.T, cfg agentgrpc.AgentConfig, llmOverride builtin.LLMClient) (*grpc.ClientConn, func()) {
	t.Helper()

	lis := bufconn.Listen(bufSize)
	srv := grpc.NewServer()
	svc := agentgrpc.NewAgentServiceServer("test-agent", cfg)
	if llmOverride != nil {
		svc.SetLLMClientOverride(llmOverride)
	}
	agentservicepb.RegisterAgentServiceServer(srv, svc)

	go func() {
		if err := srv.Serve(lis); err != nil && err != grpc.ErrServerStopped {
			t.Logf("test server error: %v", err)
		}
	}()

	conn, err := grpc.NewClient("passthrough:///bufnet",
		grpc.WithContextDialer(func(ctx context.Context, _ string) (net.Conn, error) {
			return lis.DialContext(ctx)
		}),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		t.Fatalf("failed to dial bufconn: %v", err)
	}

	cleanup := func() {
		conn.Close()
		srv.Stop()
		lis.Close()
	}
	return conn, cleanup
}

// TestPing verifies the health-check RPC.
func TestPing(t *testing.T) {
	conn, cleanup := startTestServer(t, agentgrpc.AgentConfig{}, nil)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.Ping(context.Background(), &agentservicepb.PingRequest{})
	if err != nil {
		t.Fatalf("Ping failed: %v", err)
	}
	if resp.AgentId != "test-agent" {
		t.Errorf("expected agent_id=test-agent, got %q", resp.AgentId)
	}
	if resp.Version == "" {
		t.Error("expected non-empty version")
	}
}

// TestRunTask_NoToolCalls verifies that a single LLM response with no tool calls
// completes the loop and emits TASK_COMPLETE.
func TestRunTask_NoToolCalls(t *testing.T) {
	mock := &mockLLMClient{
		responses: []builtin.ChatResponse{
			{Message: builtin.Message{Role: builtin.RoleAssistant, Content: "Hello!"}},
		},
	}

	conn, cleanup := startTestServer(t, agentgrpc.AgentConfig{MaxTokens: 100}, mock)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	stream, err := client.RunTask(context.Background(), &agentservicepb.RunTaskRequest{
		Task:  "Say hi",
		Model: "mock",
	})
	if err != nil {
		t.Fatalf("RunTask failed: %v", err)
	}

	var events []*agentservicepb.RunTaskEvent
	for {
		evt, err := stream.Recv()
		if err != nil {
			break
		}
		events = append(events, evt)
	}

	if len(events) == 0 {
		t.Fatal("expected at least one event")
	}

	last := events[len(events)-1]
	if last.Type != agentservicepb.EventType_TASK_COMPLETE {
		t.Errorf("expected TASK_COMPLETE, got %v", last.Type)
	}
	if last.Content != "Hello!" {
		t.Errorf("expected 'Hello!', got %q", last.Content)
	}
}

// TestRunTask_RunStartedEvent verifies that a RUN_STARTED event is emitted first.
func TestRunTask_RunStartedEvent(t *testing.T) {
	mock := &mockLLMClient{
		responses: []builtin.ChatResponse{
			{Message: builtin.Message{Role: builtin.RoleAssistant, Content: "Done."}},
		},
	}

	conn, cleanup := startTestServer(t, agentgrpc.AgentConfig{MaxTokens: 100}, mock)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	stream, err := client.RunTask(context.Background(), &agentservicepb.RunTaskRequest{
		Task: "Do something",
	})
	if err != nil {
		t.Fatalf("RunTask failed: %v", err)
	}

	first, err := stream.Recv()
	if err != nil {
		t.Fatalf("first Recv failed: %v", err)
	}
	if first.Type != agentservicepb.EventType_RUN_STARTED {
		t.Errorf("expected first event to be RUN_STARTED, got %v", first.Type)
	}
}

// TestDispatchToSubAgent_InProcess verifies that DispatchToSubAgent with an
// empty sub_agent_address runs the sub-agent in-process via goroutine.
func TestDispatchToSubAgent_InProcess(t *testing.T) {
	mock := &mockLLMClient{
		responses: []builtin.ChatResponse{
			{Message: builtin.Message{Role: builtin.RoleAssistant, Content: "sub-agent result"}},
		},
	}

	conn, cleanup := startTestServer(t, agentgrpc.AgentConfig{MaxTokens: 100}, mock)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.DispatchToSubAgent(context.Background(), &agentservicepb.SubAgentRequest{
		Task:            "sub task",
		SubAgentAddress: "", // empty → in-process goroutine
	})
	if err != nil {
		t.Fatalf("DispatchToSubAgent failed: %v", err)
	}
	if resp.Error != "" {
		t.Fatalf("unexpected sub-agent error: %s", resp.Error)
	}
	if resp.Result != "sub-agent result" {
		t.Errorf("expected 'sub-agent result', got %q", resp.Result)
	}
}

// mockLLMClient returns canned responses for testing.
type mockLLMClient struct {
	responses []builtin.ChatResponse
	callCount int
}

func (m *mockLLMClient) Chat(_ context.Context, _ builtin.ChatRequest) (builtin.ChatResponse, error) {
	if m.callCount < len(m.responses) {
		r := m.responses[m.callCount]
		m.callCount++
		return r, nil
	}
	return builtin.ChatResponse{
		Message: builtin.Message{Role: builtin.RoleAssistant, Content: "default response"},
	}, nil
}
