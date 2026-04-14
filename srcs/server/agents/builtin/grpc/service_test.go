package agentgrpc_test

import (
	"context"
	"iter"
	"net"
	"testing"

	"google.golang.org/genai"

	"google.golang.org/adk/model"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	agentgrpc "github.com/onehumancorp/mono/srcs/server/agents/builtin/grpc"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/test/bufconn"
)

const bufSize = 1024 * 1024

// startTestServer creates an in-memory gRPC server with a bufconn listener.
// llmOverride is installed so tests do not need a real LLM provider.
func startTestServer(t *testing.T, cfg agentgrpc.AgentConfig, llmOverride model.LLM) (*grpc.ClientConn, func()) {
	t.Helper()

	lis := bufconn.Listen(bufSize)
	srv := grpc.NewServer()
	svc := agentgrpc.NewAgentServiceServer("test-agent", cfg, nil)
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

	return conn, func() {
		conn.Close()
		srv.Stop()
		lis.Close()
	}
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

// TestRunTask_NoToolCalls verifies that a single LLM response with no tool
// calls emits RUN_STARTED followed by TASK_COMPLETE.
func TestRunTask_NoToolCalls(t *testing.T) {
	mock := &mockLLM{
		responses: []*model.LLMResponse{
			{
				Content:      &genai.Content{Role: "model", Parts: []*genai.Part{{Text: "Hello!"}}},
				TurnComplete: true,
			},
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

	// First event must be RUN_STARTED.
	if events[0].Type != agentservicepb.EventType_RUN_STARTED {
		t.Errorf("expected first event RUN_STARTED, got %v", events[0].Type)
	}

	// At least one event should be TASK_COMPLETE.
	var hasComplete bool
	for _, e := range events {
		if e.Type == agentservicepb.EventType_TASK_COMPLETE {
			hasComplete = true
			if e.Content == "" {
				t.Error("TASK_COMPLETE event has empty content")
			}
		}
	}
	if !hasComplete {
		t.Errorf("no TASK_COMPLETE event received; events: %v", events)
	}
}

// TestDispatchToSubAgent_InProcess verifies in-process sub-agent dispatch
// when sub_agent_address is empty.
func TestDispatchToSubAgent_InProcess(t *testing.T) {
	mock := &mockLLM{
		responses: []*model.LLMResponse{
			{
				Content:      &genai.Content{Role: "model", Parts: []*genai.Part{{Text: "sub-agent result"}}},
				TurnComplete: true,
			},
		},
	}

	conn, cleanup := startTestServer(t, agentgrpc.AgentConfig{MaxTokens: 100}, mock)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.DispatchToSubAgent(context.Background(), &agentservicepb.SubAgentRequest{
		Task:            "sub task",
		SubAgentAddress: "",
	})
	if err != nil {
		t.Fatalf("DispatchToSubAgent failed: %v", err)
	}
	if resp.Error != "" {
		t.Fatalf("unexpected sub-agent error: %s", resp.Error)
	}
	if resp.Result == "" {
		t.Error("expected non-empty result")
	}
}

// mockLLM implements model.LLM for testing.
type mockLLM struct {
	responses []*model.LLMResponse
	callCount int
}

func (m *mockLLM) Name() string { return "mock" }

func (m *mockLLM) GenerateContent(_ context.Context, _ *model.LLMRequest, _ bool) iter.Seq2[*model.LLMResponse, error] {
	return func(yield func(*model.LLMResponse, error) bool) {
		var resp *model.LLMResponse
		if m.callCount < len(m.responses) {
			resp = m.responses[m.callCount]
			m.callCount++
		} else {
			resp = &model.LLMResponse{
				Content:      &genai.Content{Role: "model", Parts: []*genai.Part{{Text: "default response"}}},
				TurnComplete: true,
			}
		}
		yield(resp, nil)
	}
}
