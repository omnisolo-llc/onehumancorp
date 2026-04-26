package edgeoffloadmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/src/server/auth"
)

// MCP tool definition matching other tools in src/server/tools/
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type Router struct {
}

func NewRouter() *Router {
	return &Router{}
}

func (r *Router) ListTools() []Tool {
	return []Tool{
		{
			Name:        "mcp_inference_router",
			Description: "Routes inference requests based on Edge LLM Offloading Protocol.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"prompt": {"type": "string"}, "is_sensitive": {"type": "boolean"}, "complexity": {"type": "string"}}, "required": ["prompt", "is_sensitive", "complexity"]}`),
		},
	}
}

func (r *Router) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if toolName != "mcp_inference_router" {
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}

	_, ok := arguments["prompt"].(string)
	if !ok {
		return nil, errors.New("missing or invalid 'prompt' argument")
	}

	isSensitive, ok := arguments["is_sensitive"].(bool)
	if !ok {
		return nil, errors.New("missing or invalid 'is_sensitive' argument")
	}

	complexity, ok := arguments["complexity"].(string)
	if !ok {
		return nil, errors.New("missing or invalid 'complexity' argument")
	}

    // Evaluate
	if isSensitive || complexity == "low" {
		return map[string]interface{}{
			"route":  "local",
			"status": "success",
			"response": "Routed to Local Inference",
		}, nil
	}

	// Need to route to cloud
	// Verify SPIFFE/SPIRE SVIDs
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		// Unauthorized fallback
        return map[string]interface{}{
			"route":  "local",
			"status": "fallback",
			"response": "Fallback to Local Inference due to unauthorized request",
		}, nil
	}

	// Mocking Cloud load check
	// Should fallback if cloud failed
	cloudError := os.Getenv("MOCK_CLOUD_ERROR")
	if cloudError == "true" {
        return map[string]interface{}{
			"route":  "local",
			"status": "fallback",
			"response": "Fallback to Local Inference due to network error",
		}, nil
	}

	return map[string]interface{}{
		"route":  "cloud",
		"status": "success",
		"response": "Routed to Cloud K8s Pod",
	}, nil
}
