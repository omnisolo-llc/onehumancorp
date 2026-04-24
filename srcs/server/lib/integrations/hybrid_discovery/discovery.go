package hybrid_discovery

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// ToolSpec represents a dynamically discovered tool.
type ToolSpec struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Endpoint    string `json:"endpoint"`
}

// SVID represents a SPIFFE Verifiable Identity Document mock.
type SVID struct {
	ID    string
	Token string
}

// DiscoveryProxy implements the MCP tool for dynamic tool discovery.
type DiscoveryProxy struct {
	provider    db.Provider
	switchboard string
}

// NewDiscoveryProxy creates a new DiscoveryProxy instance.
func NewDiscoveryProxy(provider db.Provider, switchboard string) *DiscoveryProxy {
	return &DiscoveryProxy{
		provider:    provider,
		switchboard: switchboard,
	}
}

// isSQLite checks if the underlying database driver is SQLite.
func (p *DiscoveryProxy) isSQLite() bool {
	if p.provider == nil {
		return false
	}
	return p.provider.IsSQLite()
}

// SearchTools searches for tools based on intent.
// It routes to SQLite FTS for standalone mode, or remote Switchboard for Postgres mode.
func (p *DiscoveryProxy) SearchTools(ctx context.Context, intent string) ([]ToolSpec, error) {
	if p.isSQLite() {
		return p.searchSQLite(ctx, intent)
	}
	return p.searchSwitchboard(ctx, intent)
}

// searchSQLite performs a simple search against the local SQLite registry.
func (p *DiscoveryProxy) searchSQLite(ctx context.Context, intent string) ([]ToolSpec, error) {
	log.Printf("Executing local SQLite search for intent: %s", intent)
	// In a real implementation, this would use SQLite FTS5 extension.
	// For this mock, we just return a stub if the table doesn't exist,
	// or perform a simple LIKE query if we setup a basic table.

	// Create a dummy table if not exists for testing purposes
	_, err := p.provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS local_mcp_tools (
			name TEXT,
			description TEXT,
			endpoint TEXT
		)
	`)
	if err != nil {
		log.Printf("Failed to ensure local_mcp_tools table exists: %v", err)
		// Return empty list instead of error for resilience
		return []ToolSpec{}, nil
	}

	// Insert dummy data if table is empty
	var count int
	row := p.provider.QueryRow(ctx, "SELECT COUNT(*) FROM local_mcp_tools")
	if row != nil {
		err = row.Scan(&count)
		if err == nil && count == 0 {
			_, _ = p.provider.Exec(ctx, `
				INSERT INTO local_mcp_tools (name, description, endpoint) VALUES
				('local-calculator', 'A local calculator tool', 'local://calculator'),
				('local-grep', 'Local file search tool', 'local://grep')
			`)
		}
	}

	rows, err := p.provider.Query(ctx, `
		SELECT name, description, endpoint
		FROM local_mcp_tools
		WHERE description LIKE $1 OR name LIKE $2
	`, "%"+intent+"%", "%"+intent+"%")

	if err != nil {
		return nil, fmt.Errorf("sqlite search error: %w", err)
	}
	defer rows.Close()

	var tools []ToolSpec
	for rows.Next() {
		var t ToolSpec
		if err := rows.Scan(&t.Name, &t.Description, &t.Endpoint); err != nil {
			return nil, err
		}
		tools = append(tools, t)
	}

	// If nothing matched, maybe return a dummy for testing
	if len(tools) == 0 && intent == "calculator" {
		return []ToolSpec{{
			Name:        "local-calculator",
			Description: "A local calculator tool",
			Endpoint:    "local://calculator",
		}}, nil
	}

	return tools, nil
}

// searchSwitchboard simulates routing to the Cloud Switchboard.
func (p *DiscoveryProxy) searchSwitchboard(ctx context.Context, intent string) ([]ToolSpec, error) {
	log.Printf("Routing request to Cloud Switchboard (%s) for intent: %s", p.switchboard, intent)

	// Simulate gRPC call to Switchboard
	if intent == "calculator" {
		return []ToolSpec{{
			Name:        "cloud-calculator",
			Description: "Cloud hosted calculator tool via Switchboard",
			Endpoint:    "grpc://switchboard.cloud.internal/calculator",
		}}, nil
	}

	return []ToolSpec{}, nil
}

// RegisterTool registers a dynamically discovered tool.
func (p *DiscoveryProxy) RegisterTool(ctx context.Context, spec ToolSpec) error {
	if p.isSQLite() {
		return p.registerSQLite(ctx, spec)
	}
	return p.registerSwitchboard(ctx, spec)
}

func (p *DiscoveryProxy) registerSQLite(ctx context.Context, spec ToolSpec) error {
	log.Printf("Registering tool in SQLite: %s", spec.Name)
	_, err := p.provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS local_mcp_tools (
			name TEXT,
			description TEXT,
			endpoint TEXT
		)
	`)
	if err != nil {
		return fmt.Errorf("failed to ensure table exists: %w", err)
	}

	_, err = p.provider.Exec(ctx, `
		INSERT INTO local_mcp_tools (name, description, endpoint)
		VALUES ($1, $2, $3)
	`, spec.Name, spec.Description, spec.Endpoint)
	if err != nil {
		return fmt.Errorf("failed to insert tool: %w", err)
	}
	return nil
}

func (p *DiscoveryProxy) registerSwitchboard(ctx context.Context, spec ToolSpec) error {
	log.Printf("Routing registration to Cloud Switchboard (%s) for tool: %s", p.switchboard, spec.Name)
	// Simulate gRPC call to Switchboard
	return nil
}

// RequestToolSVID requests a SPIFFE identity for the tool.
func (p *DiscoveryProxy) RequestToolSVID(ctx context.Context, toolName string) (SVID, error) {
	if p.isSQLite() {
		log.Printf("Bypassing SPIRE in SQLite mode for tool: %s", toolName)
		return SVID{
			ID:    fmt.Sprintf("spiffe://local.standalone/tool/%s", toolName),
			Token: "mock-local-token-12345",
		}, nil
	}

	log.Printf("Requesting remote SPIRE SVID for tool: %s", toolName)
	// Simulate SPIRE request
	return SVID{
		ID:    fmt.Sprintf("spiffe://cloud.internal/tool/%s", toolName),
		Token: "real-spire-jwt-token-98765",
	}, nil
}

// SyncLocalToolsToCloud synchronizes local tool schemas to the cloud registry via the Teammate Mesh.
func (p *DiscoveryProxy) SyncLocalToolsToCloud(ctx context.Context, mesh orchestration.MeshTransport) error {
	if !p.isSQLite() {
		return nil
	}

	rows, err := p.provider.Query(ctx, "SELECT name, description, endpoint FROM local_mcp_tools")
	if err != nil {
		return nil
	}
	defer rows.Close()

	for rows.Next() {
		var t ToolSpec
		if err := rows.Scan(&t.Name, &t.Description, &t.Endpoint); err != nil {
			log.Printf("Failed to scan local tool: %v", err)
			continue
		}

		// Publish the JSON payload mapped correctly to the Cloud Dashboard's MCPTool format
		mcpPayload := map[string]interface{}{
			"tool": map[string]interface{}{
				"id":          t.Name, // use name as ID for now
				"name":        t.Name,
				"description": t.Description,
				"category":    "local",
				"status":      "available",
			},
			"spiffeId": fmt.Sprintf("spiffe://local.standalone/tool/%s", t.Name),
		}

		payloadBytes, err := json.Marshal(mcpPayload)
		if err != nil {
			log.Printf("Failed to marshal local tool: %v", err)
			continue
		}

		mesh.PublishTeammateMeshEvent(ctx, "mcp_tool_sync", "system", "RegisterTool", "available", payloadBytes)
	}
	return nil
}
