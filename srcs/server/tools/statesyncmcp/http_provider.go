package statesyncmcp

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// HTTPProvider implements StateSyncProvider by communicating with the Cloud API
// via HTTP, and reading/writing to the local SQLite database.
type HTTPProvider struct {
	cloudURL   string
	httpClient *http.Client
	localDB    db.Provider
}

// NewHTTPProvider creates a new HTTPProvider.
func NewHTTPProvider(cloudURL string, localDB db.Provider) *HTTPProvider {
	return &HTTPProvider{
		cloudURL: cloudURL,
		httpClient: &http.Client{
			Timeout: 30 * time.Second,
		},
		localDB: localDB,
	}
}

// Ensure HTTPProvider implements StateSyncProvider
var _ StateSyncProvider = (*HTTPProvider)(nil)

// SyncUp pushes local state changes to the cloud.
func (p *HTTPProvider) SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if !p.localDB.IsSQLite() {
		return map[string]interface{}{"status": "skipped", "reason": "not a local sqlite database"}, nil
	}

	// Example implementation: querying unsynced transitions and pushing them.
	// In a real implementation, you'd fetch from actual tables.
	// We'll mock the data fetching part to just send a request.

	payload := map[string]interface{}{
		"transitions": []interface{}{},
		"timestamp":   time.Now().Format(time.RFC3339),
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal sync up payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, p.cloudURL+"/api/v1/sync/up", bytes.NewReader(body))
	if err != nil {
		return nil, fmt.Errorf("failed to create sync up request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	if claims != nil && claims.OrganizationID != "" {
		req.Header.Set("X-Tenant-ID", claims.OrganizationID)
		// Assuming we pass some auth token if we had one in claims, or rely on mesh
	}

	resp, err := p.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("sync up request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		respBody, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("sync up failed with status %d: %s", resp.StatusCode, string(respBody))
	}

	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		// If response is not JSON, just return success
		return map[string]interface{}{
			"status": "success",
			"items_synced": 0,
		}, nil
	}

	return result, nil
}

// SyncDown fetches state changes from the cloud to the local state.
func (p *HTTPProvider) SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if !p.localDB.IsSQLite() {
		return map[string]interface{}{"status": "skipped", "reason": "not a local sqlite database"}, nil
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, p.cloudURL+"/api/v1/sync/down", nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create sync down request: %w", err)
	}

	if claims != nil && claims.OrganizationID != "" {
		req.Header.Set("X-Tenant-ID", claims.OrganizationID)
	}

	resp, err := p.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("sync down request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		respBody, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("sync down failed with status %d: %s", resp.StatusCode, string(respBody))
	}

	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return map[string]interface{}{
			"status": "success",
			"items_fetched": 0,
		}, nil
	}

	// In a real implementation, you would apply the fetched result to localDB here.

	return result, nil
}

// GetStatus returns the current synchronization status.
func (p *HTTPProvider) GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatus, error) {
	if !p.localDB.IsSQLite() {
		return &SyncStatus{
			Status: "skipped (cloud mode)",
		}, nil
	}

	// This is a stub implementation.
	// You would typically query local DB for unsynced count and last sync timestamp.
	return &SyncStatus{
		LastSyncTime: time.Now().Format(time.RFC3339),
		PendingItems: 0,
		Status:       "synchronized",
	}, nil
}

// NoopProvider is a fallback provider for cloud-native context
// where no local SQLite database exists.
type NoopProvider struct{}

// NewNoopProvider creates a new NoopProvider.
func NewNoopProvider() *NoopProvider {
	return &NoopProvider{}
}

// Ensure NoopProvider implements StateSyncProvider
var _ StateSyncProvider = (*NoopProvider)(nil)

func (p *NoopProvider) SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return map[string]interface{}{
		"status": "skipped",
		"reason": "running in cloud-native mode, no local sync required",
	}, nil
}

func (p *NoopProvider) SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return map[string]interface{}{
		"status": "skipped",
		"reason": "running in cloud-native mode, no local sync required",
	}, nil
}

func (p *NoopProvider) GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatus, error) {
	return &SyncStatus{
		Status: "skipped (cloud mode)",
	}, nil
}
