package builtin

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestBuiltinAgent(t *testing.T) {
	// A mock LLM client for testing.
	mockClient := &MockClient{
		Response: ChatResponse{
			Message: Message{
				Role:    RoleAssistant,
				Content: "Hello, world!",
			},
		},
	}

	agent := &BuiltinAgent{
		Client:      mockClient,
		Model:       "mock-model",
		System:      "You are a helpful assistant.",
		Tools:       []Tool{SendMessageTool, TodoWriteTool},
		MaxTokens:   100,
		Temperature: 0,
	}

	messages, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Say hi"}})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if len(messages) < 2 {
		t.Fatalf("expected at least 2 messages, got %d", len(messages))
	}

	if messages[1].Content != "Hello, world!" {
		t.Fatalf("expected 'Hello, world!', got %q", messages[1].Content)
	}
}

func TestBuiltinAgentTelemetry(t *testing.T) {
	// Initialize telemetry with a mock meter to capture metrics
	_, _ = telemetry.InitTelemetry()

	mockClient := &MockClient{
		Response: ChatResponse{
			Message: Message{
				Role:    RoleAssistant,
				Content: "Hello!",
			},
			Usage: Usage{
				InputTokens:  10,
				OutputTokens: 20,
			},
		},
	}

	agent := &BuiltinAgent{
		AgentID:        "test-agent",
		OrganizationID: "test-org",
		Role:           "tester",
		Client:         mockClient,
		Model:          "claude-3-7-sonnet",
		System:         "test",
		MaxTokens:      100,
	}

	// We don't have an easy way to assert on the telemetry side without complex mocks,
	// but we can ensure it doesn't panic and we can look at the coverage/logs if needed.
	// For this test, we just ensure it executes.
	_, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "hi"}})
	if err != nil {
		t.Fatalf("agent run failed: %v", err)
	}
}

func TestTools(t *testing.T) {
	// Test a simple tool execution
	ctx := context.Background()

	// Test WebSearchTool
	res, err := WebSearchTool.Execute(ctx, []byte(`{"query":"test"}`))
	if err != nil {
		t.Fatalf("WebSearchTool err: %v", err)
	}
	if res == "" {
		t.Fatal("WebSearchTool returned empty result")
	}

	// Test TodoWriteTool — new list-based API
	res, err = TodoWriteTool.Execute(ctx, []byte(`{"todos":[{"content":"test todo","status":"pending"}]}`))
	if err != nil {
		t.Fatalf("TodoWriteTool err: %v", err)
	}
	if res == "" {
		t.Fatal("TodoWriteTool returned empty result")
	}

    // Verify all tools are correctly registered
    allTools := AllTools()
    if len(allTools) != 17 {
        t.Fatalf("Expected 17 tools, got %d", len(allTools))
    }
}

// MockClient implements LLMClient for testing.
type MockClient struct {
	Response ChatResponse
	Err      error
}

func (m *MockClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	return m.Response, m.Err
}
