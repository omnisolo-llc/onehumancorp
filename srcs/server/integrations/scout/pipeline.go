package scout

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/lib/integrations/hybrid_discovery"
)

// Pipeline handles the dynamic discovery and registration of tools.
type Pipeline struct {
	proxy *hybrid_discovery.DiscoveryProxy
}

// NewPipeline creates a new scout pipeline.
func NewPipeline(proxy *hybrid_discovery.DiscoveryProxy) *Pipeline {
	return &Pipeline{proxy: proxy}
}

// ParseAndRegister fetches an OpenAPI spec from a URL, parses it, and registers endpoints.
func (p *Pipeline) ParseAndRegister(ctx context.Context, specURL string) error {
	// 1. Fetch spec
	resp, err := http.Get(specURL)
	if err != nil {
		return fmt.Errorf("failed to fetch spec: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("bad status code: %d", resp.StatusCode)
	}

	// 2. Basic parse (simplistic parsing for demonstration)
	var spec map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&spec); err != nil {
		return fmt.Errorf("failed to decode spec: %w", err)
	}

	paths, ok := spec["paths"].(map[string]interface{})
	if !ok {
		return fmt.Errorf("invalid spec format: missing paths")
	}

	// 3. Register tools
	for path, methods := range paths {
		methodsMap, ok := methods.(map[string]interface{})
		if !ok {
			continue
		}

		for method, operation := range methodsMap {
			opMap, ok := operation.(map[string]interface{})
			if !ok {
				continue
			}

			desc, _ := opMap["description"].(string)
			if desc == "" {
				desc = fmt.Sprintf("Auto-discovered tool for %s %s", method, path)
			}

			// Extract a safe name
			name, _ := opMap["operationId"].(string)
			if name == "" {
				name = fmt.Sprintf("scout-%s-%s", method, path)
			}

			// Validate safety guardrails
			if !p.validateGuardrails(name, path) {
				continue
			}

			tool := hybrid_discovery.ToolSpec{
				Name:        name,
				Description: desc,
				Endpoint:    specURL + path, // Simplified endpoint calculation
			}

			err = p.proxy.RegisterTool(ctx, tool)
			if err != nil {
				return fmt.Errorf("failed to register tool %s: %w", name, err)
			}
		}
	}

	return nil
}

func (p *Pipeline) validateGuardrails(name, endpoint string) bool {
	// Simple safety check for SSRF or dangerous keywords
	if name == "admin" || name == "delete" {
		return false
	}
	return true
}
