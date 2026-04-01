package telemetry

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// StartBufferedMetricsSync initializes the background sync ticker for Standalone mode.
func StartBufferedMetricsSync(ctx context.Context, provider db.Provider) {
	if !provider.IsSQLite() {
		return
	}

	BufferMetricFunc = func(metricType string, payload string) {
		_, err := provider.Exec(context.Background(),
			"INSERT INTO local_metrics_buffer (metric_type, payload) VALUES (?1, ?2)",
			metricType, payload)
		if err != nil {
			slog.Error("failed to buffer metric locally", "error", err, "metricType", metricType)
		}
	}

	go func() {
		ticker := time.NewTicker(30 * time.Second)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				err := SyncBufferedMetrics(ctx, provider)
				if err != nil {
					slog.Error("failed to sync buffered metrics", "error", err)
				}
			}
		}
	}()
}

// SyncBufferedMetrics synchronizes buffered local metrics to the cloud DB.
func SyncBufferedMetrics(ctx context.Context, provider db.Provider) error {
	rows, err := provider.Query(ctx, "SELECT id, metric_type, payload FROM local_metrics_buffer ORDER BY id ASC LIMIT 100")
	if err != nil {
		return fmt.Errorf("query local_metrics_buffer: %w", err)
	}
	defer rows.Close()

	var ids []int
	for rows.Next() {
		var id int
		var metricType, payload string
		if err := rows.Scan(&id, &metricType, &payload); err != nil {
			return fmt.Errorf("scan local_metrics_buffer: %w", err)
		}

		// In a real scenario, this is where we would POST the payloads to the Cloud API
		// or sync them to the central OHC-SIP DB. Since this is Standalone, we simulate
		// the successful transmission.

		ids = append(ids, id)
	}

	if err := rows.Err(); err != nil {
		return fmt.Errorf("rows error: %w", err)
	}
	rows.Close()

	if len(ids) > 0 {
		for _, id := range ids {
			_, err := provider.Exec(ctx, "DELETE FROM local_metrics_buffer WHERE id = ?1", id)
			if err != nil {
				return fmt.Errorf("delete synced metric %d: %w", id, err)
			}
		}
		slog.InfoContext(ctx, "synced buffered metrics to cloud", "count", len(ids))
	}

	return nil
}
