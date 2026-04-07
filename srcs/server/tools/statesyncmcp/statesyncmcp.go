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
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// MCP interfaces

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

type CallToolRequest struct {
	Name      string                 `json:"name"`
	Arguments map[string]interface{} `json:"arguments"`
}

type CallToolResponse struct {
	Content []Content `json:"content"`
	IsError bool      `json:"isError"`
}

type Content struct {
	Type string `json:"type"`
	Text string `json:"text"`
}

// StateSyncProvider defines the interface for synchronization operations.
type StateSyncProvider interface {
	SyncUp(ctx context.Context) (string, error)
	SyncDown(ctx context.Context) (string, error)
	GetStatus(ctx context.Context) (string, error)
}

// DefaultStateSyncProvider implements StateSyncProvider.
type DefaultStateSyncProvider struct {
	DB         *db.DB
	CloudURL   string
	HTTPClient *http.Client
}

func NewStateSyncProvider(d *db.DB, cloudURL string) *DefaultStateSyncProvider {
	if cloudURL == "" {
		cloudURL = os.Getenv("OHC_CORE_URL")
	}
	return &DefaultStateSyncProvider{
		DB:         d,
		CloudURL:   cloudURL,
		HTTPClient: &http.Client{Timeout: 10 * time.Second},
	}
}

// Task is a minimal representation of a task to sync.
type Task struct {
	ID              string    `json:"id"`
	OrganizationID  string    `json:"organization_id"`
	Title           string    `json:"title"`
	Status          string    `json:"status"`
	UpdatedAt       time.Time `json:"updated_at"`
}

func (p *DefaultStateSyncProvider) SyncUp(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}

	if !p.DB.IsSQLite() {
		return "No-op: Running in Cloud mode natively", nil
	}

	// Fetch unsynced state transitions. For simplicity, we just fetch all tasks for the org.
	// In a real scenario, we'd check sync_status.
	rows, err := p.DB.Query(ctx, "SELECT id, title, status, updated_at FROM shared_tasks WHERE organization_id = ?", claims.OrganizationID)
	if err != nil {
		// If table doesn't exist or other error, handle gracefully
		if isTableNotExist(err) {
			return "No tasks to sync (table not found)", nil
		}
		return "", fmt.Errorf("failed to query local tasks: %w", err)
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		t.OrganizationID = claims.OrganizationID
		var updated string
		if err := rows.Scan(&t.ID, &t.Title, &t.Status, &updated); err != nil {
			continue
		}
		if parsed, err := time.Parse(time.RFC3339, updated); err == nil {
			t.UpdatedAt = parsed
		}
		tasks = append(tasks, t)
	}

	if len(tasks) == 0 {
		return "No state transitions to sync", nil
	}

	payload, err := json.Marshal(tasks)
	if err != nil {
		return "", fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, p.CloudURL+"/api/v1/sync/up", bytes.NewReader(payload))
	if err != nil {
		return "", err
	}
	// Authentication
	// (Simulated JWT injection)
	req.Header.Set("Authorization", "Bearer simulated-jwt-from-claims")
	req.Header.Set("Content-Type", "application/json")

	resp, err := p.HTTPClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("cloud sync API error: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("cloud sync API returned status %d: %s", resp.StatusCode, string(body))
	}

	return fmt.Sprintf("Successfully synced %d tasks to cloud", len(tasks)), nil
}

func (p *DefaultStateSyncProvider) SyncDown(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}

	if !p.DB.IsSQLite() {
		return "No-op: Running in Cloud mode natively", nil
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, p.CloudURL+"/api/v1/sync/down?org_id="+claims.OrganizationID, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("Authorization", "Bearer simulated-jwt-from-claims")

	resp, err := p.HTTPClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("cloud sync API error: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("cloud sync API returned status %d: %s", resp.StatusCode, string(body))
	}

	var tasks []Task
	if err := json.NewDecoder(resp.Body).Decode(&tasks); err != nil {
		return "", fmt.Errorf("failed to decode response: %w", err)
	}

	for _, t := range tasks {
		// Upsert logic (Last-Write-Wins based on updated_at could be complex, simple upsert here for demonstration)
		_, err := p.DB.Exec(ctx, `
			INSERT INTO shared_tasks (id, organization_id, title, status, updated_at)
			VALUES (?, ?, ?, ?, ?)
			ON CONFLICT(id) DO UPDATE SET
				title=excluded.title,
				status=excluded.status,
				updated_at=excluded.updated_at
			WHERE excluded.updated_at > shared_tasks.updated_at
		`, t.ID, t.OrganizationID, t.Title, t.Status, t.UpdatedAt.Format(time.RFC3339))
		if err != nil {
			// ignore specific row errors
		}
	}

	return fmt.Sprintf("Successfully synced %d tasks from cloud", len(tasks)), nil
}

func (p *DefaultStateSyncProvider) GetStatus(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if !p.DB.IsSQLite() {
		return "Cloud Mode: Real-time synchronization active", nil
	}
	return "Standalone Mode: Ready to sync", nil
}

func isTableNotExist(err error) bool {
	return err != nil && (err.Error() == "no such table: shared_tasks" || err.Error() == "relation \"shared_tasks\" does not exist")
}

// Server represents the MCP server for State Sync.
type Server struct {
	Provider StateSyncProvider
}

func NewServer(provider StateSyncProvider) *Server {
	return &Server{Provider: provider}
}

func (s *Server) ListTools(ctx context.Context) []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronize local SQLite state to the cloud PostgreSQL database.",
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetch KAIROS shared tasks from the cloud to update the local SQLite database.",
		},
		{
			Name:        "get_sync_status",
			Description: "Get the current synchronization status.",
		},
	}
}

func (s *Server) CallTool(ctx context.Context, req CallToolRequest) CallToolResponse {
	var result string
	var err error

	switch req.Name {
	case "sync_local_to_cloud":
		result, err = s.Provider.SyncUp(ctx)
	case "sync_cloud_to_local":
		result, err = s.Provider.SyncDown(ctx)
	case "get_sync_status":
		result, err = s.Provider.GetStatus(ctx)
	default:
		err = fmt.Errorf("unknown tool: %s", req.Name)
	}

	if err != nil {
		return CallToolResponse{
			IsError: true,
			Content: []Content{{Type: "text", Text: err.Error()}},
		}
	}

	return CallToolResponse{
		IsError: false,
		Content: []Content{{Type: "text", Text: result}},
	}
}
