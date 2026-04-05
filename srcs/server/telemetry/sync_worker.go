package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type BufferedMetric struct {
	ID         int64     `json:"id"`
	MetricType string    `json:"metric_type"`
	Payload    string    `json:"payload"`
	CreatedAt  time.Time `json:"created_at"`
}

var SyncClient = &http.Client{Timeout: 10 * time.Second}

func InitBufferMetricFunc(database *db.DB) {
	if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") == "true" {
		BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
			_, err := database.Exec(ctx, `
                INSERT INTO telemetry_buffer (metric_type, payload)
                VALUES ($1, $2)
            `, metricType, payload)
			if err != nil {
				slog.ErrorContext(ctx, "failed to buffer telemetry", "error", err)
			}
			return err
		}
	} else {
		BufferMetricFunc = nil
	}
}

func StartSyncWorker(ctx context.Context, database *db.DB) {
	if os.Getenv("OHC_STANDALONE") != "true" || os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return
	}

	go func() {
		endpoint := os.Getenv("OHC_CLOUD_API_URL")
		if endpoint == "" {
			endpoint = "https://cloud.onehumancorp.com/api/telemetry/ingest"
		}

		backoff := 1 * time.Second
		maxBackoff := 60 * time.Second

		ticker := time.NewTicker(10 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				err := syncBatch(context.WithoutCancel(ctx), database, endpoint)
				if err != nil {
					slog.Warn("telemetry sync failed", "error", err)
					time.Sleep(backoff)
					backoff *= 2
					if backoff > maxBackoff {
						backoff = maxBackoff
					}
				} else {
					backoff = 1 * time.Second
				}
			}
		}
	}()
}

func syncBatch(ctx context.Context, database *db.DB, endpoint string) error {
	rows, err := database.Query(ctx, `
		SELECT id, metric_type, payload, created_at
		FROM telemetry_buffer
		ORDER BY id ASC
		LIMIT 100
	`)
	if err != nil {
		return err
	}
	defer rows.Close()

	var metrics []BufferedMetric
	for rows.Next() {
		var m BufferedMetric
		if err := rows.Scan(&m.ID, &m.MetricType, &m.Payload, &m.CreatedAt); err != nil {
			return err
		}
		metrics = append(metrics, m)
	}

	if err := rows.Err(); err != nil {
		return err
	}

	if len(metrics) == 0 {
		return nil
	}

	payloadBytes, err := json.Marshal(metrics)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", endpoint, bytes.NewBuffer(payloadBytes))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	resp, err := SyncClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return err
	}

	// Delete successful syncs
	ids := make([]int64, len(metrics))
	for i, m := range metrics {
		ids[i] = m.ID
	}

	// Simple deletion one by one since SQLite doesn't natively support easy arrays without building strings,
	// but building an IN clause is fine
	if len(ids) > 0 {
		// Use a transaction for batch deletion
		tx, err := database.Begin(ctx)
		if err != nil {
			return err
		}
		for _, id := range ids {
			_, err := tx.Exec(ctx, "DELETE FROM telemetry_buffer WHERE id = $1", id)
			if err != nil {
				tx.Rollback(ctx)
				return err
			}
		}
		if err := tx.Commit(ctx); err != nil {
			return err
		}
	}

	return nil
}
