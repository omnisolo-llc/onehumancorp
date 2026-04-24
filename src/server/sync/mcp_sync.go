package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

// MCPSyncClient handles synchronizing deltas back to the cloud.
type MCPSyncClient struct {
	cloudAPIURL string
}

// NewMCPSyncClient creates a new MCPSyncClient.
func NewMCPSyncClient(cloudAPIURL string) *MCPSyncClient {
	return &MCPSyncClient{
		cloudAPIURL: cloudAPIURL,
	}
}

type syncDeltasPayload struct {
	Deltas []SyncDelta `json:"deltas"`
}

// SyncDeltas pushes CRDT deltas to the cloud API endpoint.
// It also tracks telemetry if enabled and running in standalone mode.
func (c *MCPSyncClient) SyncDeltas(ctx context.Context, deltas []SyncDelta) error {
	if len(deltas) == 0 {
		return nil
	}

	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	telemetryEnabled := os.Getenv("OHC_TELEMETRY_ENABLED") == "true"

	payload := syncDeltasPayload{Deltas: deltas}
	jsonData, err := json.Marshal(payload)
	if err != nil {
		if isStandalone && telemetryEnabled && telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(deltas)))
		}
		return fmt.Errorf("marshal payloads: %w", err)
	}

	syncEndpoint := fmt.Sprintf("%s/api/v1/sync/mcp-deltas", c.cloudAPIURL)
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		if isStandalone && telemetryEnabled && telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(deltas)))
		}
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		if isStandalone && telemetryEnabled && telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(deltas)))
		}
		return fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		if isStandalone && telemetryEnabled && telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(deltas)))
		}
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	if isStandalone && telemetryEnabled && telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(deltas)))
	}

	return nil
}
