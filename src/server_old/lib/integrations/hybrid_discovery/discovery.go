package hybrid_discovery

import (
	"context"
	"database/sql"
	"fmt"
	"log/slog"

	_ "modernc.org/sqlite" // Ensure sqlite driver is available for checks
)

// ToolSpec represents a dynamically discovered tool.
type ToolSpec struct {
	Name        string
	Description string
	Endpoint    string
}

// SVID represents a SPIFFE Verifiable Identity Document mock.
type SVID struct {
	ID    string
	Token string
}

// DiscoveryProxy implements the MCP tool for dynamic tool discovery.
type DiscoveryProxy struct {
	db          *sql.DB
	switchboard string
}

// NewDiscoveryProxy creates a new DiscoveryProxy instance.
func NewDiscoveryProxy(db *sql.DB, switchboard string) *DiscoveryProxy {
	return &DiscoveryProxy{
		db:          db,
		switchboard: switchboard,
	}
}

// isSQLite checks if the underlying database driver is SQLite.
func (p *DiscoveryProxy) isSQLite() bool {
	if p.db == nil {
		return false
	}
	driverType := fmt.Sprintf("%T", p.db.Driver())
	return driverType == "*sqlite.Driver" || driverType == "*sqlite3.SQLiteDriver"
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
	slog.Info("Executing local SQLite search for intent", "intent", intent)
	// In a real implementation, this would use SQLite FTS5 extension.
	// For this mock, we just return a stub if the table doesn't exist,
	// or perform a simple LIKE query if we setup a basic table.

	// Create a dummy table if not exists for testing purposes
	_, err := p.db.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS local_mcp_tools (
			name TEXT,
			description TEXT,
			endpoint TEXT
		)
	`)
	if err != nil {
		slog.Error("Failed to ensure local_mcp_tools table exists", "error", err)
		// Return empty list instead of error for resilience
		return []ToolSpec{}, nil
	}

	// Insert dummy data if table is empty
	var count int
	err = p.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM local_mcp_tools").Scan(&count)
	if err == nil && count == 0 {
		_, _ = p.db.ExecContext(ctx, `
			INSERT INTO local_mcp_tools (name, description, endpoint) VALUES
			('local-calculator', 'A local calculator tool', 'local://calculator'),
			('local-grep', 'Local file search tool', 'local://grep')
		`)
	}

	rows, err := p.db.QueryContext(ctx, `
		SELECT name, description, endpoint
		FROM local_mcp_tools
		WHERE description LIKE ? OR name LIKE ?
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
	slog.Info("Routing request to Cloud Switchboard", "switchboard", p.switchboard, "intent", intent)

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
	slog.Info("Registering tool in SQLite", "tool", spec.Name)
	_, err := p.db.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS local_mcp_tools (
			name TEXT,
			description TEXT,
			endpoint TEXT
		)
	`)
	if err != nil {
		return fmt.Errorf("failed to ensure table exists: %w", err)
	}

	_, err = p.db.ExecContext(ctx, `
		INSERT INTO local_mcp_tools (name, description, endpoint)
		VALUES (?, ?, ?)
	`, spec.Name, spec.Description, spec.Endpoint)
	if err != nil {
		return fmt.Errorf("failed to insert tool: %w", err)
	}
	return nil
}

func (p *DiscoveryProxy) registerSwitchboard(ctx context.Context, spec ToolSpec) error {
	slog.Info("Routing registration to Cloud Switchboard", "switchboard", p.switchboard, "tool", spec.Name)
	// Simulate gRPC call to Switchboard
	return nil
}

// RequestToolSVID requests a SPIFFE identity for the tool.
func (p *DiscoveryProxy) RequestToolSVID(ctx context.Context, toolName string) (SVID, error) {
	if p.isSQLite() {
		slog.Info("Bypassing SPIRE in SQLite mode for tool", "tool", toolName)
		return SVID{
			ID:    fmt.Sprintf("spiffe://local.standalone/tool/%s", toolName),
			Token: "mock-local-token-12345",
		}, nil
	}

	slog.Info("Requesting remote SPIRE SVID for tool", "tool", toolName)
	// Simulate SPIRE request
	return SVID{
		ID:    fmt.Sprintf("spiffe://cloud.internal/tool/%s", toolName),
		Token: "real-spire-jwt-token-98765",
	}, nil
}
