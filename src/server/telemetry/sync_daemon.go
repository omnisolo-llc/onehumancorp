package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"time"
)

type SyncDaemon struct {
	db           *sql.DB
	cloudURL     string
	syncInterval time.Duration
	batchSize    int
}

func NewSyncDaemon(db *sql.DB, cloudURL string, syncInterval time.Duration, batchSize int) *SyncDaemon {
	return &SyncDaemon{
		db:           db,
		cloudURL:     cloudURL,
		syncInterval: syncInterval,
		batchSize:    batchSize,
	}
}

func (d *SyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(d.syncInterval)
	defer ticker.Stop()

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
	rows, err := d.db.QueryContext(ctx, "SELECT id, metric_type, payload FROM telemetry_buffer ORDER BY id ASC LIMIT $1", d.batchSize)
	if err != nil {
		return fmt.Errorf("query telemetry_buffer: %w", err)
	}
	defer rows.Close()

	var payloads []map[string]interface{}
	var ids []int64

	for rows.Next() {
		var id int64
		var metricType, payload string
		if err := rows.Scan(&id, &metricType, &payload); err != nil {
			return fmt.Errorf("scan telemetry_buffer: %w", err)
		}
		payloads = append(payloads, map[string]interface{}{
			"metric_type": metricType,
			"payload":     payload,
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

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudURL+"/api/telemetry/sync", bytes.NewReader(body))
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

	// Delete synced rows
	for _, id := range ids {
		if _, err := d.db.ExecContext(ctx, "DELETE FROM telemetry_buffer WHERE id = $1", id); err != nil {
			return fmt.Errorf("delete telemetry_buffer id=%d: %w", id, err)
		}
	}

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

		_, err := db.ExecContext(ctx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)", metricType, payload)
		if err != nil {
			return fmt.Errorf("failed to buffer metric %s: %w", metricType, err)
		}
		return nil
	}
}
