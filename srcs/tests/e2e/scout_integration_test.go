package e2e

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/lib/integrations/hybrid_discovery"
	_ "modernc.org/sqlite"
)

// mockLLMClient is a dummy LLM client that returns a direct tool call to ScoutRegister.
type mockLLMClient struct {
	serverURL string
}

func (m *mockLLMClient) Chat(ctx context.Context, req builtin.ChatRequest) (builtin.ChatResponse, error) {
	// Stop condition: if we already received a tool result message.
	if len(req.Messages) > 1 && req.Messages[len(req.Messages)-1].Role == builtin.RoleTool {
	    return builtin.ChatResponse{
            Message: builtin.Message{
                Role: builtin.RoleAssistant,
                Content: "Registration complete.",
            },
        }, nil
    }

	// First message is user prompt. Return an assistant message calling the tool.
	return builtin.ChatResponse{
		Message: builtin.Message{
			Role:    builtin.RoleAssistant,
			Content: "I will register the API now.",
			ToolCalls: []builtin.ToolCall{
				{
					ID:   "call_scout_1",
					Name: "ScoutRegister",
					Arguments: func() json.RawMessage {
						args, _ := json.Marshal(map[string]string{
							"openapi_url": m.serverURL,
						})
						return args
					}(),
				},
			},
		},
	}, nil
}

func TestScoutIntegrationE2E(t *testing.T) {
	// Start a local dummy OpenAPI server
	openAPIJSON := `{
		"paths": {
			"/customers": {
				"get": {
					"operationId": "getCustomers",
					"summary": "Get all customers",
					"description": "Returns a list of customers"
				}
			}
		}
	}`

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(openAPIJSON))
	}))
	defer server.Close()

	// Setup context with DiscoveryProxy
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	proxy := hybrid_discovery.NewDiscoveryProxy(db, "")
	ctx := context.WithValue(context.Background(), "discovery_proxy", proxy)

	// Create Scout agent config
	cfg := builtin.AgentConfig{
		LLM:              &mockLLMClient{serverURL: server.URL},
		Tools:            builtin.AllTools(),
		SystemPrompt:     "You are the Scout agent.",
		MaxTurns:         5,
		MaxTokensPerTurn: 100,
	}

	// Create and run the agent synchronously for testing
	agent := &builtin.BuiltinAgent{
		Client:    cfg.LLM,
		Model:     "test-model",
		System:    cfg.SystemPrompt,
		Tools:     cfg.Tools,
		MaxTokens: cfg.MaxTokensPerTurn,
	}

	messages := []builtin.Message{{Role: builtin.RoleUser, Content: "Register the API"}}
	_, err = agent.Run(ctx, messages)
	if err != nil {
		t.Fatalf("Agent run failed: %v", err)
	}

	// Verify that the tool was registered
	ctxTimeout, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()

	tools, err := proxy.SearchTools(ctxTimeout, "getCustomers")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Fatalf("Expected tool 'getCustomers' to be registered, but found none")
	}

	found := false
	for _, tool := range tools {
		if tool.Name == "getCustomers" {
			found = true
			expectedEndpoint := server.URL + "/customers"
			if tool.Endpoint != expectedEndpoint {
				t.Errorf("expected endpoint %s, got %s", expectedEndpoint, tool.Endpoint)
			}
		}
	}

	if !found {
		t.Errorf("getCustomers tool was not found in search results")
	}
}
