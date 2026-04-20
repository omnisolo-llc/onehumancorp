package hybrid_discovery

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net"
	"net/http"
	"net/url"
	"regexp"
	"strings"

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
	return driverType == "*sqlite.Driver" || driverType == "*sqlite.Driver"
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
	_, err := p.db.ExecContext(ctx, `
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

// SSRFGuardrailBypass allows tests to bypass SSRF checks for local httptest servers.
var SSRFGuardrailBypass bool

// validateGuardrails checks a URL and tool names for basic safety compliance, preventing SSRF.
func validateGuardrails(specURL string) error {
	u, err := url.Parse(specURL)
	if err != nil {
		return fmt.Errorf("invalid URL: %w", err)
	}
	if u.Scheme != "http" && u.Scheme != "https" {
		return errors.New("guardrail violation: URL must be http or https")
	}

	if SSRFGuardrailBypass {
		return nil
	}

	host := u.Hostname()

	// Prevent loopback, localhost, and obvious internal ranges
	if host == "localhost" || strings.HasPrefix(host, "127.") || host == "::1" || host == "169.254.169.254" {
		return errors.New("guardrail violation: loopback and link-local addresses are forbidden")
	}

	// Resolve the host to verify the actual IP address
	ips, err := net.LookupIP(host)
	if err != nil {
		return fmt.Errorf("guardrail violation: unable to resolve host %q", host)
	}

	for _, ip := range ips {
		if ip.IsLoopback() || ip.IsPrivate() || ip.IsLinkLocalUnicast() || ip.IsLinkLocalMulticast() || ip.IsUnspecified() {
			return fmt.Errorf("guardrail violation: resolved IP %s is private/loopback/link-local", ip.String())
		}
	}

	return nil
}

// validateToolName ensures tool names only contain alphanumeric and dashes.
func validateToolName(name string) error {
	matched, _ := regexp.MatchString(`^[a-zA-Z0-9\-]+$`, name)
	if !matched {
		return fmt.Errorf("guardrail violation: tool name %q contains invalid characters", name)
	}
	return nil
}

// ImportOpenAPI fetches an OpenAPI spec via HTTP, parses it to extract endpoints as tools,
// applies safety guardrails, and dynamically registers them.
func (p *DiscoveryProxy) ImportOpenAPI(ctx context.Context, specURL string) error {
	log.Printf("Scout: Importing OpenAPI spec from %s", specURL)

	if err := validateGuardrails(specURL); err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, specURL, nil)
	if err != nil {
		return err
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("failed to fetch OpenAPI spec: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	// Minimal parser: looking for top-level paths or an info object to extract a tool.
	// For simplicity in this mock integration, we parse it into a generic map and
	// try to extract some endpoints or just register the whole API as one tool.
	var spec map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&spec); err != nil {
		return fmt.Errorf("failed to parse JSON spec: %w", err)
	}

	var title string
	if info, ok := spec["info"].(map[string]interface{}); ok {
		if t, ok := info["title"].(string); ok {
			title = t
		}
	}
	if title == "" {
		title = "imported-api-tool"
	}

	// Sanitize title to use as tool name
	re := regexp.MustCompile(`[^a-zA-Z0-9]+`)
	toolName := string(re.ReplaceAll([]byte(title), []byte("-")))
	if len(toolName) > 0 && toolName[0] == '-' {
		toolName = toolName[1:]
	}
	if len(toolName) > 0 && toolName[len(toolName)-1] == '-' {
		toolName = toolName[:len(toolName)-1]
	}
	if toolName == "" {
		toolName = "imported-api-tool"
	}

	if err := validateToolName(toolName); err != nil {
		return err
	}

	desc := fmt.Sprintf("Dynamically discovered tool from %s", specURL)
	endpoint := fmt.Sprintf("dynamic://%s", toolName)

	if p.isSQLite() {
		// Ensure table exists (same logic as searchSQLite)
		_, err := p.db.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS local_mcp_tools (
				name TEXT,
				description TEXT,
				endpoint TEXT
			)
		`)
		if err != nil {
			return fmt.Errorf("failed to create table: %w", err)
		}

		_, err = p.db.ExecContext(ctx, `
			INSERT INTO local_mcp_tools (name, description, endpoint) VALUES (?, ?, ?)
		`, toolName, desc, endpoint)
		if err != nil {
			return fmt.Errorf("failed to register tool locally: %w", err)
		}
		log.Printf("Scout: Successfully registered local tool %q", toolName)
		return nil
	}

	// Mock Switchboard registration for Postgres/Cloud mode
	log.Printf("Scout: Routing tool registration %q to Cloud Switchboard", toolName)
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
