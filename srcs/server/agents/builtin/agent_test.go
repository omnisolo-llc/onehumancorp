package builtin

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestBuiltinAgent_RunLoop(t *testing.T) {
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

	config := DefaultRunConfig()
	messages, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Say hi"}}, &config)
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

func TestBuiltinAgent_MaxTurns(t *testing.T) {
	mockClient := &MockClient{
		Response: ChatResponse{
			Message: Message{
				Role: RoleAssistant,
				ToolCalls: []ToolCall{
					{
						ID:        "1",
						Name:      "SendMessage",
						Arguments: []byte(`{"message":"test"}`),
					},
				},
			},
		},
	}

	agent := &BuiltinAgent{
		Client:      mockClient,
		Model:       "mock-model",
		Tools:       []Tool{SendMessageTool},
		MaxTokens:   100,
	}

	config := DefaultRunConfig()
	config.MaxTurns = 2

	_, err := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Loop me"}}, &config)
	if err == nil {
		t.Fatal("expected max turns error, got none")
	}
	if err.Error() != "reached maximum turns (2)" {
		t.Fatalf("expected max turns message, got %v", err)
	}
}

func TestBuiltinAgent_AuthDeny(t *testing.T) {
	authTool := Tool{
		Name:         "SecretTool",
		Description:  "Needs auth",
		RequiresAuth: true,
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			return "secret", nil
		},
	}

	mockClient := &MockClient{
		Response: ChatResponse{
			Message: Message{
				Role: RoleAssistant,
				ToolCalls: []ToolCall{
					{
						ID:        "1",
						Name:      "SecretTool",
						Arguments: []byte(`{}`),
					},
				},
			},
		},
	}

	agent := &BuiltinAgent{
		Client:      mockClient,
		Model:       "mock-model",
		Tools:       []Tool{authTool},
		MaxTokens:   100,
	}

	config := DefaultRunConfig()
	config.CanUseTool = func(name string, args json.RawMessage) bool { return false } // Deny

	messages, _ := agent.Run(context.Background(), []Message{{Role: RoleUser, Content: "Use secret"}}, &config)

	// Should have executed the tool and gotten "Permission denied by user"
	foundDeny := false
	for _, m := range messages {
		if m.Role == RoleTool {
			for _, tr := range m.ToolResults {
				if tr.Error == "Permission denied by user" {
					foundDeny = true
				}
			}
		}
	}

	if !foundDeny {
		t.Fatal("expected permission denied error in tool results")
	}
}

func TestTools(t *testing.T) {
	ctx := context.Background()

	res, err := WebSearchTool.Execute(ctx, []byte(`{"query":"test"}`))
	if err != nil {
		t.Fatalf("WebSearchTool err: %v", err)
	}
	if res == "" {
		t.Fatal("WebSearchTool returned empty result")
	}

	defer os.RemoveAll(".agent-task")
	os.MkdirAll(".agent-task", 0755)

	res, err = TodoWriteTool.Execute(ctx, []byte(`{"todo":"test todo"}`))
	if err != nil {
		t.Fatalf("TodoWriteTool err: %v", err)
	}
	if res == "" {
		t.Fatal("TodoWriteTool returned empty result")
	}
}

type MockClient struct {
	Response ChatResponse
	Err      error
}

func (m *MockClient) Chat(req ChatRequest) (ChatResponse, error) {
	return m.Response, m.Err
}
