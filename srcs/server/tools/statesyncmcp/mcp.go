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

// StateSyncProvider defines the interface for synchronizing local state with the cloud.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) error
	SyncDown(ctx context.Context, claims *auth.Claims) error
	GetStatus(ctx context.Context) (map[string]interface{}, error)
}

// DBStateSyncMCP implements the MCP tools for local-to-cloud synchronization.
type DBStateSyncMCP struct {
	provider   db.Provider
	httpClient *http.Client
	cloudURL   string
}

// NewDBStateSyncMCP creates a new instance of DBStateSyncMCP.
func NewDBStateSyncMCP(provider db.Provider, httpClient *http.Client) *DBStateSyncMCP {
	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "https://api.onehumancorp.com"
	}
	if httpClient == nil {
		httpClient = http.DefaultClient
	}
	return &DBStateSyncMCP{
		provider:   provider,
		httpClient: httpClient,
		cloudURL:   cloudURL,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *DBStateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Pushes unsynced local state transitions to the cloud backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches completed tasks and updated state from the cloud to local storage.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Retrieves the current synchronization status.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *DBStateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "sync_local_to_cloud":
		err := m.SyncUp(ctx, claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "operation": "sync_up"}, nil
	case "sync_cloud_to_local":
		err := m.SyncDown(ctx, claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "operation": "sync_down"}, nil
	case "get_sync_status":
		return m.GetStatus(ctx)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// SyncUp pushes local data to the cloud.
func (m *DBStateSyncMCP) SyncUp(ctx context.Context, claims *auth.Claims) error {
	if !m.provider.IsSQLite() {
		return nil // Already in the cloud, nothing to push
	}

	// For simplicity, we define unsynced state as tasks in 'pending' or 'in_progress' state.
	// In a real implementation, you might track 'synced_at' and push records where 'updated_at' > 'synced_at'.
	query := "SELECT id, status, payload, updated_at FROM kairos_tasks WHERE status IN ('pending', 'in_progress')"
	rows, err := m.provider.Query(ctx, query)
	if err != nil {
		// If the table doesn't exist, we gracefully return nil assuming no tasks.
		return nil
	}
	defer rows.Close()

	var tasks []map[string]interface{}
	for rows.Next() {
		var id, status, updatedAt string
		var payload *string
		if err := rows.Scan(&id, &status, &payload, &updatedAt); err == nil {
			task := map[string]interface{}{
				"id":         id,
				"status":     status,
				"updated_at": updatedAt,
			}
			if payload != nil {
				task["payload"] = *payload
			}
			tasks = append(tasks, task)
		}
	}

	if len(tasks) == 0 {
		return nil
	}

	payloadBytes, err := json.Marshal(map[string]interface{}{
		"organization_id": claims.OrganizationID,
		"tasks":           tasks,
	})
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", m.cloudURL+"/api/v1/sync/up", bytes.NewReader(payloadBytes))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	// Authorize with some proxy token or JWT. Assuming the cloud trusts this request if signed.
	// We could use the claims here to create a JWT, but for simplicity we assume the caller
	// handles the Bearer token or we just pass the OrgID.

	resp, err := m.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("cloud sync up failed with status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}

// SyncDown fetches cloud data to local.
func (m *DBStateSyncMCP) SyncDown(ctx context.Context, claims *auth.Claims) error {
	if !m.provider.IsSQLite() {
		return nil // Already in the cloud, nothing to fetch
	}

	req, err := http.NewRequestWithContext(ctx, "GET", m.cloudURL+"/api/v1/sync/down?org_id="+claims.OrganizationID, nil)
	if err != nil {
		return err
	}

	resp, err := m.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("cloud sync down failed with status %d: %s", resp.StatusCode, string(body))
	}

	var data struct {
		Tasks []struct {
			ID        string  `json:"id"`
			Status    string  `json:"status"`
			Payload   *string `json:"payload"`
			UpdatedAt string  `json:"updated_at"`
		} `json:"tasks"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&data); err != nil {
		return err
	}

	// Update local database (Last-Write-Wins based on cloud).
	// In a complete implementation, you'd compare updated_at.
	for _, task := range data.Tasks {
		// Use a simple query or fallback to upsert if possible.
		// Since this is SQLite, we can use INSERT ... ON CONFLICT
		upsertQuery := `
			INSERT INTO kairos_tasks (id, status, payload, updated_at)
			VALUES (?, ?, ?, ?)
			ON CONFLICT(id) DO UPDATE SET
				status = excluded.status,
				payload = excluded.payload,
				updated_at = excluded.updated_at
			WHERE excluded.updated_at > kairos_tasks.updated_at
		`
		_, err := m.provider.Exec(ctx, upsertQuery, task.ID, task.Status, task.Payload, task.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to sync down task %s: %w", task.ID, err)
		}
	}

	return nil
}

// GetStatus retrieves sync status.
func (m *DBStateSyncMCP) GetStatus(ctx context.Context) (map[string]interface{}, error) {
	mode := "cloud"
	if m.provider.IsSQLite() {
		mode = "standalone"
	}
	return map[string]interface{}{
		"mode":   mode,
		"status": "ready",
	}, nil
}
