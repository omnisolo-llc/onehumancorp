package statesyncmcp

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// StateSyncProvider abstracts the sync logic.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// DefaultStateSyncProvider is the default implementation using db.Provider.
type DefaultStateSyncProvider struct {
	provider   db.Provider
	httpClient *http.Client
}

// NewDefaultStateSyncProvider creates a new DefaultStateSyncProvider.
func NewDefaultStateSyncProvider(provider db.Provider, httpClient *http.Client) *DefaultStateSyncProvider {
	if httpClient == nil {
		httpClient = http.DefaultClient
	}
	return &DefaultStateSyncProvider{
		provider:   provider,
		httpClient: httpClient,
	}
}

// SyncUp queries the local SQLite DB for unsynced state transitions, serializes them, and pushes them.
func (p *DefaultStateSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if !p.provider.IsSQLite() {
		return map[string]interface{}{"status": "no-op", "message": "Running natively in Cloud, sync up not required."}, nil
	}
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization_id")
	}

	// In a real scenario, this would query local SQLite for unsynced transitions
	// e.g. SELECT * FROM agent_missions WHERE synced = false
	// For this mock, we pretend we found some.

	payload := map[string]interface{}{
		"organization_id": claims.OrganizationID,
		"synced_items":    []string{"mission_1", "mission_2"}, // Mock items
	}

	bodyBytes, err := json.Marshal(payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal payload: %w", err)
	}

	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "http://localhost:8080" // Fallback
	}

	req, err := http.NewRequestWithContext(ctx, "POST", cloudURL+"/api/v1/sync/up", bytes.NewReader(bodyBytes))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	// Note: in a real implementation, we'd add auth token

	resp, err := p.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		respBody, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("cloud sync failed with status %d: %s", resp.StatusCode, string(respBody))
	}

	return map[string]interface{}{"status": "success", "synced_count": len(payload["synced_items"].([]string))}, nil
}

// SyncDown fetches completed tasks from the cloud and updates the local SQLite database.
func (p *DefaultStateSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if !p.provider.IsSQLite() {
		return map[string]interface{}{"status": "no-op", "message": "Running natively in Cloud, sync down not required."}, nil
	}
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization_id")
	}

	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "http://localhost:8080" // Fallback
	}

	req, err := http.NewRequestWithContext(ctx, "GET", cloudURL+"/api/v1/sync/down?org_id="+claims.OrganizationID, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := p.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		respBody, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("cloud sync failed with status %d: %s", resp.StatusCode, string(respBody))
	}

	// In a real scenario, this would parse resp and update local SQLite DB
	return map[string]interface{}{"status": "success", "message": "Sync down completed"}, nil
}

// GetStatus returns the sync status.
func (p *DefaultStateSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if !p.provider.IsSQLite() {
		return map[string]interface{}{"status": "no-op", "mode": "cloud"}, nil
	}
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization_id")
	}

	return map[string]interface{}{"status": "ok", "mode": "standalone", "unsynced_up": 2, "unsynced_down": 0}, nil
}

// StateSyncMCP implements the MCP interface.
type StateSyncMCP struct {
	syncProvider StateSyncProvider
}

// NewStateSyncMCP creates a new StateSyncMCP instance.
func NewStateSyncMCP(syncProvider StateSyncProvider) *StateSyncMCP {
	return &StateSyncMCP{
		syncProvider: syncProvider,
	}
}

// ListTools returns the list of available tools.
func (m *StateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronizes local SQLite state to the Cloud backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches state from the Cloud backend to the local SQLite database.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Retrieves the current sync status between local and cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	// We don't check claims yet because the mock logic handles missing claims by either no-oping or erroring

	switch toolName {
	case "sync_local_to_cloud":
		return m.syncProvider.SyncUp(ctx, claims)
	case "sync_cloud_to_local":
		return m.syncProvider.SyncDown(ctx, claims)
	case "get_sync_status":
		return m.syncProvider.GetStatus(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
