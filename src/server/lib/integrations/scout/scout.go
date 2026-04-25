package scout

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/src/server/lib/integrations/hybrid_discovery"
)

// Scout represents the resource scout agent.
type Scout struct {
	proxy *hybrid_discovery.DiscoveryProxy
}

// NewScout creates a new Scout agent instance.
func NewScout(proxy *hybrid_discovery.DiscoveryProxy) *Scout {
	return &Scout{proxy: proxy}
}

// OpenAPIDoc represents a simplified OpenAPI structure for parsing.
type OpenAPIDoc struct {
	Paths map[string]PathItem `json:"paths"`
}

type PathItem struct {
	Get  *Operation `json:"get"`
	Post *Operation `json:"post"`
}

type Operation struct {
	OperationID string `json:"operationId"`
	Summary     string `json:"summary"`
	Description string `json:"description"`
}

// ParseAndRegister parses an OpenAPI spec from a URL and registers its tools.
func (s *Scout) ParseAndRegister(ctx context.Context, openAPIURL string) error {
	log.Printf("Scout: Fetching OpenAPI spec from %s", openAPIURL)

	var tools []hybrid_discovery.ToolSpec

	// Real HTTP fetching and parsing
	req, err := http.NewRequestWithContext(ctx, "GET", openAPIURL, nil)
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("failed to fetch OpenAPI spec: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to fetch OpenAPI spec, status code: %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read response body: %w", err)
	}

	var doc OpenAPIDoc
	if err := json.Unmarshal(body, &doc); err != nil {
		return fmt.Errorf("failed to parse JSON OpenAPI spec: %w", err)
	}

	for path, item := range doc.Paths {
		if item.Get != nil {
			name := item.Get.OperationID
			if name == "" {
				name = "get-" + strings.ReplaceAll(path, "/", "-")
			}
			tools = append(tools, hybrid_discovery.ToolSpec{
				Name:        name,
				Description: item.Get.Summary + " " + item.Get.Description,
				Endpoint:    openAPIURL + path,
			})
		}
		if item.Post != nil {
			name := item.Post.OperationID
			if name == "" {
				name = "post-" + strings.ReplaceAll(path, "/", "-")
			}
			tools = append(tools, hybrid_discovery.ToolSpec{
				Name:        name,
				Description: item.Post.Summary + " " + item.Post.Description,
				Endpoint:    openAPIURL + path,
			})
		}
	}

	for _, tool := range tools {
		log.Printf("Scout: Validating tool %s against guardrails", tool.Name)
		// Simulate guardrail validation (e.g. deny tools with dangerous names)
		if strings.Contains(strings.ToLower(tool.Name), "dangerous") || strings.Contains(strings.ToLower(tool.Name), "delete") {
			return fmt.Errorf("tool %s failed guardrail validation", tool.Name)
		}

		log.Printf("Scout: Registering tool %s", tool.Name)
		err := s.proxy.RegisterTool(ctx, tool)
		if err != nil {
			return fmt.Errorf("failed to register tool %s: %w", tool.Name, err)
		}
	}

	return nil
}
