package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"strings"
	"time"
)

// SyncDaemon handles periodic synchronization of offline telemetry from local storage to the cloud.
type SyncDaemon struct {
	db           *sql.DB
	cloudURL     string
	syncInterval time.Duration
	batchSize    int
}

// NewSyncDaemon creates a new instance of SyncDaemon.
func NewSyncDaemon(db *sql.DB, cloudURL string, syncInterval time.Duration, batchSize int) *SyncDaemon {
	return &SyncDaemon{
		db:           db,
		cloudURL:     cloudURL,
		syncInterval: syncInterval,
		batchSize:    batchSize,
	}
}

// Start begins the background synchronization process.
func (d *SyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(d.syncInterval)
	defer ticker.Stop()

	slog.Info("Starting TelemetrySyncDaemon", "cloud_url", d.cloudURL, "interval", d.syncInterval)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := d.syncOnce(ctx); err != nil {
				slog.Error("Telemetry sync failed", "error", err)
			}
		}
	}
}

func (d *SyncDaemon) syncOnce(ctx context.Context) error {
	// Query the new offline_telemetry_buffer table
	rows, err := d.db.QueryContext(ctx, "SELECT id, metric_name, payload_bytes FROM offline_telemetry_buffer ORDER BY id ASC LIMIT ?", d.batchSize)
	if err != nil {
		return fmt.Errorf("query offline_telemetry_buffer: %w", err)
	}
	defer rows.Close()

	var payloads []map[string]interface{}
	var ids []int64

	for rows.Next() {
		var id int64
		var metricName string
		var payloadBytes []byte
		if err := rows.Scan(&id, &metricName, &payloadBytes); err != nil {
			return fmt.Errorf("scan offline_telemetry_buffer: %w", err)
		}

		var payloadData interface{}
		if err := json.Unmarshal(payloadBytes, &payloadData); err != nil {
			slog.Warn("Failed to unmarshal telemetry payload", "id", id, "error", err)
			continue
		}

		payloads = append(payloads, map[string]interface{}{
			"metric_name": metricName,
			"payload":     payloadData,
		})
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return nil
	}

	body, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

	// Send to the v1 sync endpoint
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudURL+"/api/v1/telemetry/sync", bytes.NewReader(body))
	if err != nil {
		return fmt.Errorf("new request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	// Efficient batch deletion for successfully synced records
	if len(ids) > 0 {
		idStrs := make([]string, len(ids))
		for i, id := range ids {
			idStrs[i] = fmt.Sprintf("%d", id)
		}
		query := fmt.Sprintf("DELETE FROM offline_telemetry_buffer WHERE id IN (%s)", strings.Join(idStrs, ","))
		if _, err := d.db.ExecContext(ctx, query); err != nil {
			return fmt.Errorf("batch delete offline_telemetry_buffer: %w", err)
		}
	}

	slog.Debug("Successfully synced telemetry batch", "count", len(ids))
	return nil
}

// InitStandaloneBuffer configures the telemetry system to buffer metrics locally in SQLite.
func InitStandaloneBuffer(db *sql.DB) {
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(payload), &data); err == nil {
			redactedData := RedactInterfacePII(data)
			if redactedBytes, err := json.Marshal(redactedData); err == nil {
				payload = string(redactedBytes)
			}
		}

		// Insert into offline_telemetry_buffer
		_, err := db.ExecContext(ctx, "INSERT INTO offline_telemetry_buffer (metric_name, payload_bytes) VALUES (?, ?)", metricType, []byte(payload))
		if err != nil {
			return fmt.Errorf("failed to buffer metric %s: %w", metricType, err)
		}
		return nil
	}
}
