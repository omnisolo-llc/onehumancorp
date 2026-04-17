package builtin

import (
	"context"
	"strings"
	"time"

	"testing"
	"os"
	"encoding/json"
)

func TestCostTrackerE2E(t *testing.T) {
	// Initialize fake client that simulates a chat interaction
	// that takes multiple turns
	client := &fakeLLMClient{
		responses: []ChatResponse{
			{
				Message: Message{
					Role:    RoleAssistant,
					Content: "I need to call a tool.",
					ToolCalls: []ToolCall{
						{
							ID:        "call_1",
							Name:      "test_tool",
							Arguments: []byte(`{}`),
						},
					},
				},
				Usage: Usage{
					InputTokens:  100,
					OutputTokens: 50,
				},
			},
			{
				Message: Message{
					Role:    RoleAssistant,
					Content: "Task complete.",
				},
				Usage: Usage{
					InputTokens:  200,
					OutputTokens: 20,
				},
			},
		},
	}

	cfg := AgentConfig{
		LLM: client,
		Tools: []Tool{
			{
				Name:        "test_tool",
				Description: "test tool",
				Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
					return "tool result", nil
				},
			},
		},
		SystemPrompt: "You are a test agent.",
	}

	// Make sure telemetry doesn't panic
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	ctx := context.Background()

	state, err := SpawnTask(ctx, "Test Task", "Do something.", ".", cfg)
	if err != nil {
		t.Fatalf("failed to spawn task: %v", err)
	}

	// wait for completion
	// In our fake it's very fast, we can just poll the state
	for i := 0; i < 100; i++ {
		if state.Status() == TaskStatusCompleted || state.Status() == TaskStatusFailed {
			break
		}
		// wait a bit
		time.Sleep(10 * time.Millisecond)
	}


	if state.Status() != TaskStatusCompleted {
		t.Errorf("expected task to complete, got %s", state.Status())
	}

	// Verify that the prompt included the cost correctly for the second request
	if len(client.requests) != 2 {
		t.Fatalf("expected 2 requests, got %d", len(client.requests))
	}

	// First request has cost 0
	if !strings.Contains(client.requests[0].System, "[System] Current Session Cost: $0.0000") {
		t.Errorf("expected 0 session cost in first request, got: %s", client.requests[0].System)
	}

	// First request usage: 100 input, 50 output
	// claude-3-7-sonnet-20250219 cost: (100 / 1M) * 3 + (50 / 1M) * 15 = 0.0003 + 0.00075 = 0.00105
	if !strings.Contains(client.requests[1].System, "[System] Current Session Cost: $0.001") {
		t.Errorf("expected >0 session cost in second request, got: %s", client.requests[1].System)
	}
}

type fakeLLMClient struct {
	requests  []ChatRequest
	responses []ChatResponse
	index     int
}

func (f *fakeLLMClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	f.requests = append(f.requests, req)
	if f.index < len(f.responses) {
		resp := f.responses[f.index]
		f.index++
		return resp, nil
	}
	return ChatResponse{}, nil
}
