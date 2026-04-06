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
		Tools:       []Tool{},
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


// MockClient implements LLMClient for testing.
type MockClient struct {
	Response ChatResponse
	Err      error
}

func (m *MockClient) Chat(req ChatRequest) (ChatResponse, error) {
	return m.Response, m.Err
}
