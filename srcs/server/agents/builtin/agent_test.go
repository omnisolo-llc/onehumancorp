package builtin

import (
	"context"
	"testing"
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
