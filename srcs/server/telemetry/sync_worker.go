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

// SyncDaemon manages background sync of buffered metrics in standalone mode.
type SyncDaemon struct {
	db       db.Provider
	endpoint string
	client   *http.Client
}

// NewSyncDaemon creates a new SyncDaemon.
func NewSyncDaemon(provider db.Provider, endpoint string) *SyncDaemon {
	return &SyncDaemon{
		db:       provider,
		endpoint: endpoint,
		client:   &http.Client{Timeout: 10 * time.Second},
	}
}

// Start begins the sync daemon loop.
func (sd *SyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			sd.syncMetrics(ctx)
		}
	}
}

// syncMetrics reads batches from telemetry_buffer and pushes them to cloud.
func (sd *SyncDaemon) syncMetrics(ctx context.Context) {
	batchSize := 100
	query := `SELECT id, metric_type, payload, created_at FROM telemetry_buffer ORDER BY created_at ASC LIMIT $1`

	for {
		rows, err := sd.db.Query(ctx, query, batchSize)
		if err != nil {
			slog.ErrorContext(ctx, "failed to query telemetry buffer", "error", err)
			return
		}

		type bufferedMetric struct {
			ID         string    `json:"id"`
			MetricType string    `json:"metric_type"`
			Payload    string    `json:"payload"`
			CreatedAt  time.Time `json:"created_at"`
		}

		var metrics []bufferedMetric
		var ids []string

		for rows.Next() {
			var m bufferedMetric
			if err := rows.Scan(&m.ID, &m.MetricType, &m.Payload, &m.CreatedAt); err != nil {
				slog.ErrorContext(ctx, "failed to scan telemetry buffer", "error", err)
				rows.Close()
				return
			}
			metrics = append(metrics, m)
			ids = append(ids, m.ID)
		}
		rows.Close()

		if len(metrics) == 0 {
			break
		}

		start := time.Now()
		payloadBytes, err := json.Marshal(metrics)
		if err != nil {
			slog.ErrorContext(ctx, "failed to marshal sync metrics", "error", err)
			return
		}

		req, err := http.NewRequestWithContext(ctx, http.MethodPost, sd.endpoint, bytes.NewBuffer(payloadBytes))
		if err != nil {
			slog.ErrorContext(ctx, "failed to create sync request", "error", err)
			return
		}

		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

		resp, err := sd.client.Do(req)
		if err != nil || resp.StatusCode >= 400 {
			slog.WarnContext(ctx, "failed to sync telemetry, will retry later", "error", err)
			if resp != nil {
				resp.Body.Close()
			}
			return
		}
		resp.Body.Close()

		// Delete synced records
		delQuery := `DELETE FROM telemetry_buffer WHERE id = $1`
		for _, id := range ids {
			if _, err := sd.db.Exec(ctx, delQuery, id); err != nil {
				slog.ErrorContext(ctx, "failed to delete synced telemetry", "error", err, "id", id)
			}
		}

		RecordSyncDaemonBatchSize(ctx, int64(len(metrics)))
		RecordSyncPayloadSize(ctx, int64(len(payloadBytes)))
		RecordSyncLatency(ctx, float64(time.Since(start).Milliseconds()))
	}
}
