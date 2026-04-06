package builtin

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

type fakeLLM struct {
	turns []local.AssistantMessage
	index int
}

func (f *fakeLLM) Complete(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error) {
	if f.index < len(f.turns) {
		msg := f.turns[f.index]
		f.index++
		return &msg, nil
	}
	return &local.AssistantMessage{Text: "done", StopReason: "end_turn"}, nil
}

type fakeTool struct{}

func (t *fakeTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{Name: "test_tool"}
}

func (t *fakeTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	return "tool output", nil
}

func TestAgent_Run(t *testing.T) {
	llm := &fakeLLM{
		turns: []local.AssistantMessage{
			{
				Text:       "Thinking...",
				StopReason: "tool_use",
				ToolUses: []local.ToolUseRequest{
					{ID: "call_1", Name: "test_tool", Input: map[string]interface{}{}},
				},
			},
			{
				Text:       "Task complete.",
				StopReason: "end_turn",
			},
		},
	}

	cfg := AgentConfig{
		LLM:   llm,
		Tools: []local.Tool{&fakeTool{}},
	}
	agent := NewAgent(cfg)

	result, err := agent.Run(context.Background(), "do something")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !strings.Contains(result, "Task complete.") {
		t.Errorf("unexpected result: %q", result)
	}
}

func TestAgent_TruncateTokens(t *testing.T) {
	agent := NewAgent(AgentConfig{MaxTokens: 100})
	messages := []local.ConversationMessage{
		{Role: "user", Content: []local.ContentPart{{Type: "text", Text: "prompt"}}},
		{Role: "assistant", Content: []local.ContentPart{{Type: "text", Text: "short"}}},
		{Role: "user", Content: []local.ContentPart{{Type: "text", Text: "short"}}},
		{Role: "assistant", Content: []local.ContentPart{{Type: "text", Text: "some long text that should be truncated because it is super long and exceeds the max tokens by a lot...................................................................................................................................................................................."}}},
		{Role: "user", Content: []local.ContentPart{{Type: "text", Text: "short"}}},
	}
	truncated := agent.truncateContextIfNeeded(messages, 10)
	if len(truncated) >= len(messages) {
		t.Fatalf("expected truncation, got len %d", len(truncated))
	}
	if truncated[0].Role != "user" || truncated[len(truncated)-1].Role != "user" {
		t.Fatalf("expected first and last message to remain intact")
	}
}

func TestAgent_Retries(t *testing.T) {
	llm := &fakeLLM{
		turns: []local.AssistantMessage{
			{Text: "done", StopReason: "end_turn"},
		},
	}
	agent := NewAgent(AgentConfig{LLM: llm, MaxTokens: 100})

	result, err := agent.executeWithRetry(context.Background(), local.CompletionRequest{})
	if err != nil {
		t.Fatalf("expected success, got err: %v", err)
	}
	if result.Text != "done" {
		t.Fatalf("expected 'done', got %q", result.Text)
	}
}
