package builtin

import (
	"context"
	"fmt"
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


func (m *MockClient) ChatStream(ctx context.Context, req ChatRequest, chunkChan chan<- ChatResponseChunk) error {
    chunkChan <- ChatResponseChunk{Delta: m.Response.Message.Content, StopReason: m.Response.StopReason, ToolCalls: m.Response.Message.ToolCalls}
    return m.Err
}

func TestBuiltinAgent_FallbackModel(t *testing.T) {
	mockClient := &MockClient{
		Response: ChatResponse{
			Message: Message{
				Role:    RoleAssistant,
				Content: "Hello, fallback!",
			},
		},
		Err: fmt.Errorf("simulated error"),
	}

	agent := &BuiltinAgent{
		Client:        mockClient,
		Model:         "primary-model",
		FallbackModel: "fallback-model",
		System:        "You are a helpful assistant.",
		Tools:         []Tool{},
		MaxTokens:     100,
		Temperature:   0,
	}

	mockClientWithFallbackLogic := &MockClientWithLogic{
		Fn: func(req ChatRequest) (ChatResponse, error) {
			if req.Model == "primary-model" {
				return ChatResponse{}, fmt.Errorf("primary model failed")
			}
			return ChatResponse{
				Message: Message{
					Role:    RoleAssistant,
					Content: "Hello, fallback!",
				},
			}, nil
		},
	}
	agent.Client = mockClientWithFallbackLogic

	messages, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Say hi"}})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if len(messages) < 2 {
		t.Fatalf("expected at least 2 messages, got %d", len(messages))
	}
	lastMessage := messages[len(messages)-1]
	if lastMessage.Content != "Hello, fallback!" {
		t.Fatalf("expected 'Hello, fallback!', got %q", lastMessage.Content)
	}
}

type MockClientWithLogic struct {
	Fn func(req ChatRequest) (ChatResponse, error)
}

func (m *MockClientWithLogic) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	return m.Fn(req)
}

func (m *MockClientWithLogic) ChatStream(ctx context.Context, req ChatRequest, chunkChan chan<- ChatResponseChunk) error {
	resp, err := m.Fn(req)
	chunkChan <- ChatResponseChunk{Delta: resp.Message.Content, StopReason: resp.StopReason, ToolCalls: resp.Message.ToolCalls}
	return err
}

func TestBuiltinAgent_MaxTokensEscalation(t *testing.T) {
	calls := 0
	mockClient := &MockClientWithLogic{
		Fn: func(req ChatRequest) (ChatResponse, error) {
			calls++
			if calls == 1 {
				return ChatResponse{
					Message: Message{
						Role:    RoleAssistant,
						Content: "Partial response...",
					},
					StopReason: "max_tokens",
				}, nil
			}
			return ChatResponse{
				Message: Message{
					Role:    RoleAssistant,
					Content: "Finished response.",
				},
				StopReason: "stop",
			}, nil
		},
	}

	agent := &BuiltinAgent{
		Client:            mockClient,
		Model:             "mock-model",
		System:            "You are a helpful assistant.",
		Tools:             []Tool{},
		MaxTokens:         100,
		MaxOutputEscalate: 200,
		Temperature:       0,
	}

	messages, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Say hi"}})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if len(messages) < 4 {
		t.Fatalf("expected at least 4 messages, got %d", len(messages))
	}
}
