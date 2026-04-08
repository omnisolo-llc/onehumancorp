package builtin

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestMaxTokensClamping(t *testing.T) {
	// Test Anthropic clamping
	anthropicServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var payload map[string]interface{}
		body, _ := io.ReadAll(r.Body)
		json.Unmarshal(body, &payload)

		maxTokens, ok := payload["max_tokens"].(float64)
		if !ok || maxTokens != 2048 {
			t.Errorf("Anthropic expected 2048 max_tokens, got %v", payload["max_tokens"])
		}

		// Also check prompt caching
		systemArr, ok := payload["system"].([]interface{})
		if !ok || len(systemArr) == 0 {
			t.Errorf("Anthropic expected system to be an array, got %T", payload["system"])
		} else {
			sysObj, ok := systemArr[0].(map[string]interface{})
			if !ok {
				t.Errorf("Anthropic expected system[0] to be object")
			} else if cacheCtrl, ok := sysObj["cache_control"].(map[string]interface{}); !ok || cacheCtrl["type"] != "ephemeral" {
				t.Errorf("Anthropic expected cache_control: {type: ephemeral}, got %v", sysObj["cache_control"])
			}
		}

		if r.Header.Get("anthropic-beta") != "prompt-caching-2024-07-31" {
			t.Errorf("Anthropic missing beta header")
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"content": [{"text": "ok"}]}`))
	}))
	defer anthropicServer.Close()

	anthropicClient := &AnthropicClient{
		APIKey: "test",
		Client: &http.Client{
			Transport: &transportOverride{anthropicServer.URL},
		},
	}

	_, _ = anthropicClient.Chat(context.Background(), ChatRequest{
		System: "test system",
		MaxTokens: 0, // Should clamp to 2048
	})

	// Test OpenAI clamping
	openaiServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var payload map[string]interface{}
		body, _ := io.ReadAll(r.Body)
		json.Unmarshal(body, &payload)

		maxTokens, ok := payload["max_tokens"].(float64)
		if !ok || maxTokens != 4096 {
			t.Errorf("OpenAI expected 4096 max_tokens, got %v", payload["max_tokens"])
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"choices": [{"message": {"content": "ok"}}]}`))
	}))
	defer openaiServer.Close()

	openaiClient := &OpenAIClient{
		APIKey: "test",
		Client: &http.Client{
			Transport: &transportOverride{openaiServer.URL},
		},
	}

	_, _ = openaiClient.Chat(context.Background(), ChatRequest{
		MaxTokens: 9000, // Should clamp to 4096
	})
}

// Helper to override the request URL so we can test the clients without modifying their hardcoded URLs
type transportOverride struct {
	BaseURL string
}

func (t *transportOverride) RoundTrip(req *http.Request) (*http.Response, error) {
	newReq := req.Clone(req.Context())
	newReq.URL.Scheme = "http"
	newReq.URL.Host = t.BaseURL[7:] // remove http://
	return http.DefaultTransport.RoundTrip(newReq)
}

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

	// Test TodoWriteTool
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

// MockClient implements LLMClient for testing.
type MockClient struct {
	Response ChatResponse
	Err      error
}

func (m *MockClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	return m.Response, m.Err
}
