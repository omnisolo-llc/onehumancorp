package builtin

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/lib/integrations/hybrid_discovery"
	"github.com/onehumancorp/mono/srcs/server/lib/integrations/scout"
)

// ScoutRegisterTool allows the Scout agent to parse an OpenAPI spec and register its tools dynamically.
var ScoutRegisterTool = Tool{
	Name: "ScoutRegister",
	Description: "Parse an OpenAPI spec from a URL and dynamically register the resulting endpoints as MCP tools. " +
		"Use this tool when you have discovered an external API and need to make its capabilities available to the swarm.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"openapi_url": {
				"type": "string",
				"description": "The URL pointing to the JSON OpenAPI specification of the external tool."
			}
		},
		"required": ["openapi_url"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			OpenAPIURL string `json:"openapi_url"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("ScoutRegister: invalid args: %w", err)
		}
		if input.OpenAPIURL == "" {
			return "", fmt.Errorf("ScoutRegister: openapi_url is required")
		}

		// Retrieve the proxy from context. In a real environment, this is injected by the agent harness.
		proxyVal := ctx.Value("discovery_proxy")
		if proxyVal == nil {
			return "", fmt.Errorf("ScoutRegister: discovery proxy not available in context")
		}
		proxy, ok := proxyVal.(*hybrid_discovery.DiscoveryProxy)
		if !ok {
			return "", fmt.Errorf("ScoutRegister: invalid discovery proxy type in context")
		}

		s := scout.NewScout(proxy)
		err := s.ParseAndRegister(ctx, input.OpenAPIURL)
		if err != nil {
			return "", fmt.Errorf("ScoutRegister: failed to parse and register tools: %w", err)
		}

		return fmt.Sprintf("Successfully parsed OpenAPI spec from %s and registered tools.", input.OpenAPIURL), nil
	},
}
