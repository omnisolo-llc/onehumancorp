package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"log/slog"
	"net/http"
	"time"
)

// DBProvider is a local interface for the db provider to avoid circular dependencies
type DBProvider interface {
	DB() *sql.DB
}

// HTTPClient is an interface for making HTTP requests to allow mocking.
type HTTPClient interface {
	Do(req *http.Request) (*http.Response, error)
}

// McpSyncWorker periodically syncs local telemetry buffers to the cloud.
type McpSyncWorker struct {
	provider    DBProvider
	interval    time.Duration
	endpointURL string
	httpClient  HTTPClient
}

// NewMcpSyncWorker creates a new McpSyncWorker.
func NewMcpSyncWorker(provider DBProvider, interval time.Duration, endpointURL string, httpClient HTTPClient) *McpSyncWorker {
	if interval == 0 {
		interval = 5 * time.Second
	}
	if httpClient == nil {
		httpClient = http.DefaultClient
	}
	return &McpSyncWorker{
		provider:    provider,
		interval:    interval,
		endpointURL: endpointURL,
		httpClient:  httpClient,
	}
}

// Start begins the sync loop.
func (w *McpSyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			slog.Info("McpSyncWorker shutting down...")
			return
		case <-ticker.C:
			w.syncOnce(ctx)
		}
	}
}

func (w *McpSyncWorker) syncOnce(ctx context.Context) {
	// 1. Fetch pending metrics from SQLite buffer
	rows, err := w.provider.DB().QueryContext(ctx, "SELECT id, metric_name, value FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 100")
	if err != nil {
		slog.Error("McpSyncWorker failed to query telemetry_buffer", "error", err)
		return
	}
	defer rows.Close()

	type MetricPayload struct {
		ID         string  `json:"id"`
		MetricName string  `json:"metric_name"`
		Value      float64 `json:"value"`
	}

	var pendingIDs []string
	var payloads []MetricPayload
	for rows.Next() {
		var id, metricName string
		var value float64
		if err := rows.Scan(&id, &metricName, &value); err != nil {
			slog.Error("McpSyncWorker failed to scan row", "error", err)
			continue
		}
		pendingIDs = append(pendingIDs, id)
		payloads = append(payloads, MetricPayload{
			ID:         id,
			MetricName: metricName,
			Value:      value,
		})
	}

	if err := rows.Err(); err != nil {
		slog.Error("McpSyncWorker row iteration error", "error", err)
		return
	}

	if len(pendingIDs) == 0 {
		return
	}

	// 2. Send to Cloud MCP Gateway
	payloadBytes, err := json.Marshal(payloads)
	if err != nil {
		slog.Error("McpSyncWorker failed to marshal payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.endpointURL, bytes.NewReader(payloadBytes))
	if err != nil {
		slog.Error("McpSyncWorker failed to create request", "error", err)
		return
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	resp, err := w.httpClient.Do(req)
	if err != nil {
		slog.Error("McpSyncWorker failed to send request", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		slog.Error("McpSyncWorker API Gateway returned non-2xx status", "status", resp.Status)
		return
	}

	// 3. Mark as synced
	for _, id := range pendingIDs {
		_, err := w.provider.DB().ExecContext(ctx, "UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = ?", id)
		if err != nil {
			slog.Error("McpSyncWorker failed to update status", "id", id, "error", err)
		}
	}
	slog.Info("McpSyncWorker Successfully synced metrics", "count", len(pendingIDs))
}
