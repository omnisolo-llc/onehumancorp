package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// McpSyncWorker handles background synchronization of buffered metrics to the cloud
type McpSyncWorker struct {
	dbProvider db.Provider
	endpoint   string
	httpClient *http.Client
}

// NewMcpSyncWorker creates a new MCP sync worker
func NewMcpSyncWorker(provider db.Provider, endpoint string) *McpSyncWorker {
	return &McpSyncWorker{
		dbProvider: provider,
		endpoint:   endpoint,
		httpClient: &http.Client{Timeout: 10 * time.Second},
	}
}

// Start begins the background worker loop
func (w *McpSyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.syncMetrics(ctx)
		}
	}
}

func (w *McpSyncWorker) syncMetrics(ctx context.Context) {
	// 1. Fetch pending metrics from SQLite
	// Note: In Postgres mode, we might do FOR UPDATE SKIP LOCKED, but for Standalone SQLite
	// we will just pull them and then update them in a transaction if needed.
	rows, err := w.dbProvider.Query(ctx, "SELECT id, metric_name, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 500")
	if err != nil {
		slog.Error("failed to query pending telemetry metrics", "error", err)
		return
	}
	defer rows.Close()

	var idsToSync []string
	type metricRecord struct {
		ID         string    `json:"id"`
		MetricName string    `json:"metric_name"`
		Value      float64   `json:"value"`
		LabelsJSON string    `json:"labels_json"`
		Timestamp  time.Time `json:"timestamp"`
	}
	var records []metricRecord

	for rows.Next() {
		var r metricRecord
		if err := rows.Scan(&r.ID, &r.MetricName, &r.Value, &r.LabelsJSON, &r.Timestamp); err != nil {
			slog.Error("failed to scan metric record", "error", err)
			continue
		}
		records = append(records, r)
		idsToSync = append(idsToSync, r.ID)
	}

	if len(records) == 0 {
		return // Nothing to sync
	}

	// 2. Perform MCP upload
	slog.Info("Uploading telemetry metrics to Cloud", "count", len(records), "endpoint", w.endpoint)

	payloadBytes, err := json.Marshal(records)
	if err != nil {
		slog.Error("failed to marshal metrics payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, w.endpoint, bytes.NewBuffer(payloadBytes))
	if err != nil {
		slog.Error("failed to create upload request", "error", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	// Simulate SPIFFE/SPIRE SVID authentication by adding a placeholder header.
	// In a real environment, we would use orchestration.ExtractSPIFFEID or a secure proxy
	req.Header.Set("X-Spiffe-Id", "spiffe://onehumancorp.com/workload/standalone-agent")

	resp, err := w.httpClient.Do(req)
	if err != nil {
		slog.Warn("Failed to upload metrics to cloud, will retry later", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated && resp.StatusCode != http.StatusAccepted {
		slog.Warn("Received non-success response from cloud metrics endpoint", "status", resp.StatusCode)
		return
	}

	// 3. Mark as synced
	if len(idsToSync) > 0 {
		// Use individual updates since SQLite doesn't natively support ANY($1) without extension
		for _, id := range idsToSync {
			_, err := w.dbProvider.Exec(ctx, "UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = $1", id)
			if err != nil {
				slog.Error("failed to update sync_status", "id", id, "error", err)
			}
		}
		slog.Debug("Marked metrics as synced", "count", len(idsToSync))
	}
}
