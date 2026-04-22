package registry_test

import (
	"context"
	"encoding/json"
	"testing"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/tools/registry"
)

type echoTool struct{}

func (t *echoTool) Name() string { return "echo" }
func (t *echoTool) Description() string { return "Echos the input message" }
func (t *echoTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{
		"type": "object",
		"properties": {
			"message": {
				"type": "string",
				"description": "Message to echo"
			}
		},
		"required": ["message"]
	}`)
}
func (t *echoTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	var params struct {
		Message string `json:"message"`
	}
	if err := json.Unmarshal(input, &params); err != nil {
		return nil, fmt.Errorf("failed to parse input: %w", err)
	}

	result := map[string]string{"response": params.Message}
	return json.Marshal(result)
}

func TestE2E_AgentDiscoversAndExecutesTool(t *testing.T) {
	// 1. Initialize UTR
	utr := registry.NewUnifiedToolRegistry()

	// 2. Register tools
	tool := &echoTool{}
	if err := utr.RegisterTool(tool); err != nil {
		t.Fatalf("Failed to register tool: %v", err)
	}

	// 3. Agent discovers tools
	manifests := utr.ListTools()
	if len(manifests) != 1 {
		t.Fatalf("Expected 1 tool manifest, got %d", len(manifests))
	}

	manifest := manifests[0]
	if manifest.Name != "echo" {
		t.Errorf("Expected tool name 'echo', got %s", manifest.Name)
	}

	// 4. Agent prepares execution payload based on schema
	// (Simulate agent generating JSON that matches schema)
	agentPayload := json.RawMessage(`{"message": "Hello from E2E"}`)

	// 5. Agent executes tool via registry
	result, err := utr.Execute(context.Background(), "echo", agentPayload)
	if err != nil {
		t.Fatalf("Failed to execute tool: %v", err)
	}

	// 6. Verify result
	expectedResult := `{"response":"Hello from E2E"}`
	if string(result) != expectedResult {
		t.Errorf("Expected result %s, got %s", expectedResult, string(result))
	}
}
