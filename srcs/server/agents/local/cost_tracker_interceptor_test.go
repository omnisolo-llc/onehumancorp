package local

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/billing"
)

type mockLLMClient struct {
	resp *AssistantMessage
	err  error
}

func (m *mockLLMClient) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	return m.resp, m.err
}

type mockTool struct {
	def         ToolDefinition
	executeResp string
	executeErr  error
}

func (m *mockTool) Definition() ToolDefinition {
	return m.def
}

func (m *mockTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	return m.executeResp, m.executeErr
}

func TestCostTrackerInterceptor(t *testing.T) {
	// Setup custom tracking catalog
	originalCatalog := make(map[string]billing.Price)
	for k, v := range billing.DefaultCatalog {
		originalCatalog[k] = v
	}
	defer func() {
		billing.DefaultCatalog = originalCatalog
	}()

	mock := &mockLLMClient{
		resp: &AssistantMessage{
			InputTokens:  1000,
			OutputTokens: 500,
		},
	}

	billing.DefaultCatalog["test-model"] = billing.Price{
		InputPerMillionUSD:  10.0,
		OutputPerMillionUSD: 20.0,
	}

	interceptor := NewCostTrackerInterceptor(mock, "agent-1", "org-1", "test-role", "test-model")
	resp, err := interceptor.Complete(context.Background(), CompletionRequest{})
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}
	if resp.InputTokens != 1000 || resp.OutputTokens != 500 {
		t.Fatalf("expected tokens 1000/500, got %d/%d", resp.InputTokens, resp.OutputTokens)
	}

	mockT := &mockTool{
		def:         ToolDefinition{Name: "test-tool"},
		executeResp: "done",
	}

	tInterceptor := NewToolCostTrackerInterceptor(mockT, "agent-1")
	def := tInterceptor.Definition()
	if def.Name != "test-tool" {
		t.Fatalf("expected test-tool, got %s", def.Name)
	}

	// Test successful tool execution
	toolResp, err := tInterceptor.Execute(context.Background(), ".", nil)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}
	if toolResp != "done" {
		t.Fatalf("expected tool execution to return 'done', got %s", toolResp)
	}

	// Test error propagation
	mock.err = context.Canceled
	_, err = interceptor.Complete(context.Background(), CompletionRequest{})
	if err != context.Canceled {
		t.Fatalf("expected Complete to return context.Canceled, got %v", err)
	}
}

func TestToolCostTrackerInterceptor_Error(t *testing.T) {
	mockT := &mockTool{
		def: ToolDefinition{Name: "test-tool"},
		executeErr:  context.Canceled,
	}

	tInterceptor := NewToolCostTrackerInterceptor(mockT, "agent-1")
	_, err := tInterceptor.Execute(context.Background(), ".", nil)
	if err != context.Canceled {
		t.Fatalf("expected Execute to return context.Canceled, got %v", err)
	}
}
