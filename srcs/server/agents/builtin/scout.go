package builtin

import (
	"context"
	"fmt"
	"strings"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/lib/integrations/hybrid_discovery"
)

// ScoutAgent represents the specialized Scout agent for finding and integrating tools
type ScoutAgent struct {
	Proxy *hybrid_discovery.DiscoveryProxy
}

// NewScoutAgent creates a new Scout agent
func NewScoutAgent(db *sql.DB, switchboard string) *ScoutAgent {
	return &ScoutAgent{
		Proxy: hybrid_discovery.NewDiscoveryProxy(db, switchboard),
	}
}

// SystemPrompt returns the persona definition and core instructions for Scout
func (s *ScoutAgent) SystemPrompt() string {
	return `You are Scout, a specialized OHC agent dedicated to finding external resources, APIs, and tools,
and seamlessly integrating them into the swarm's available capabilities.
Your primary role is to act as the bridge between the external tool ecosystem and OHC's internal Dynamic Tool Discovery MCP.

You utilize Web Surfing/Scraping capabilities to find resources.
You leverage API Schema Parsing (OpenAPI/Swagger) to understand tool capabilities.
You interface with the Dynamic Tool Discovery MCP (Switchboard) to register new tools dynamically.

SECURITY MANDATE:
- All dynamically discovered tools must undergo safety checks (Agentic Guardrails) before registration.
- You must authenticate via SPIFFE/SPIRE for all internal MCP interactions.`
}

// RegisterParsedAPI simulates parsing an OpenAPI spec and registering it as a new tool via the DiscoveryProxy
func (s *ScoutAgent) RegisterParsedAPI(ctx context.Context, specURL string) error {
	// Simulate scraping and parsing the OpenAPI spec
	if !strings.HasPrefix(specURL, "http") {
		return fmt.Errorf("invalid spec URL: %s", specURL)
	}

	// Simulated parsed data
	toolName := "parsed-tool-" + "random-id"
	if strings.Contains(specURL, "dummy") {
		toolName = "dummy-api-tool"
	}

	// 1. Authenticate via SPIFFE/SPIRE
	svid, err := s.Proxy.RequestToolSVID(ctx, toolName)
	if err != nil {
		return fmt.Errorf("failed to get SVID: %w", err)
	}

	// 2. Validate against safety guardrails (simulated)
	if !s.validateSafetyGuardrails(specURL, toolName) {
		return fmt.Errorf("tool failed safety guardrails validation: %s", toolName)
	}

	// 3. Register the tool
	// For the current mock DiscoveryProxy implementation, we don't have a direct "RegisterTool" method,
	// but we simulate the process. In a real system, this would write to Switchboard or local SQLite.
	// We'll log the successful registration using the SVID
	_ = svid // In real use, attach SVID token to registration request

	return nil
}

// validateSafetyGuardrails simulates checking a tool against OHC's safety policies
func (s *ScoutAgent) validateSafetyGuardrails(specURL, toolName string) bool {
	// Explicitly reject known malicious or unsafe patterns
	if strings.Contains(specURL, "malicious") || strings.Contains(toolName, "unsafe") {
		return false
	}
	// Pass all other simulated checks
	return true
}
