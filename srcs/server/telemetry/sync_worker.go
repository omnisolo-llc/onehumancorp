package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// MetricRecord represents a locally buffered telemetry metric.
type MetricRecord struct {
	ID         int64
	MetricType string
	Payload    string
}

// BufferMetricToSQLite is the interceptor that buffers metrics to the SQLite telemetry_buffer table.
func BufferMetricToSQLite(pool db.Provider) func(ctx context.Context, metricType string, payload string) error {
	return func(ctx context.Context, metricType string, payload string) error {
		// Respect the opt-in configuration for telemetry in standalone mode.
		if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
			return nil
		}

		_, err := pool.Exec(ctx, "INSERT INTO telemetry_buffer (metric_type, payload, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP)", metricType, payload)
		return err
	}
}

// StartSyncWorker periodically syncs the local telemetry_buffer with the cloud endpoint.
func StartSyncWorker(ctx context.Context, pool db.Provider, cloudEndpoint string) {
	ticker := time.NewTicker(1 * time.Minute)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				for {
					count, err := syncMetrics(ctx, pool, cloudEndpoint)
					if err != nil {
						slog.Warn("Failed to sync buffered telemetry metrics", "error", err)
						break
					}
					if count < 500 {
						break // No more batches
					}
				}
			}
		}
	}()
}

func syncMetrics(ctx context.Context, pool db.Provider, cloudEndpoint string) (int, error) {
	rows, err := pool.Query(ctx, "SELECT id, metric_type, payload FROM telemetry_buffer ORDER BY id ASC LIMIT 500")
	if err != nil {
		return 0, fmt.Errorf("failed to query telemetry_buffer: %w", err)
	}
	defer rows.Close()

	var records []MetricRecord
	for rows.Next() {
		var r MetricRecord
		if err := rows.Scan(&r.ID, &r.MetricType, &r.Payload); err != nil {
			return 0, fmt.Errorf("failed to scan record: %w", err)
		}
		records = append(records, r)
	}

	if len(records) == 0 {
		return 0, nil // Nothing to sync
	}

	payloadBytes, err := json.Marshal(records)
	if err != nil {
		return 0, fmt.Errorf("failed to marshal records: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", cloudEndpoint, bytes.NewBuffer(payloadBytes))
	if err != nil {
		return 0, fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return 0, fmt.Errorf("failed to push metrics to cloud: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return 0, fmt.Errorf("cloud endpoint returned status: %d", resp.StatusCode)
	}

	// Delete successful records
	var ids []string
	for _, r := range records {
		ids = append(ids, fmt.Sprintf("%d", r.ID))
	}

	if len(ids) > 0 {
		deleteQuery := fmt.Sprintf("DELETE FROM telemetry_buffer WHERE id IN (%s)", strings.Join(ids, ","))
		if _, err := pool.Exec(ctx, deleteQuery); err != nil {
			return 0, fmt.Errorf("failed to delete synced records: %w", err)
		}
	}

	return len(records), nil
}
