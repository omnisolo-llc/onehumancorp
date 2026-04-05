package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncWorker struct {
	db       db.Provider
	endpoint string
	client   *http.Client
	stopChan chan struct{}
	wg       sync.WaitGroup
}

func NewSyncWorker(provider db.Provider, endpoint string) *SyncWorker {
	return &SyncWorker{
		db:       provider,
		endpoint: endpoint,
		client: &http.Client{
			Timeout: 10 * time.Second,
		},
		stopChan: make(chan struct{}),
	}
}

func (w *SyncWorker) Start(ctx context.Context) {
	w.wg.Add(1)
	go func() {
		defer w.wg.Done()
		ticker := time.NewTicker(30 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-w.stopChan:
				return
			case <-ticker.C:
				w.sync(ctx)
			}
		}
	}()
}

func (w *SyncWorker) Stop() {
	close(w.stopChan)
	w.wg.Wait()
}

func (w *SyncWorker) sync(ctx context.Context) {
	if os.Getenv("OHC_STANDALONE") != "true" || os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return
	}

	// Fetch buffered metrics
	query := `SELECT id, metric_type, payload FROM telemetry_buffer ORDER BY created_at ASC LIMIT 100`
	rows, err := w.db.Query(ctx, query)
	if err != nil {
		slog.Error("Failed to fetch telemetry buffer", "error", err)
		return
	}
	defer rows.Close()

	var records []map[string]interface{}
	var ids []string

	for rows.Next() {
		var id, metricType, payloadStr string
		if err := rows.Scan(&id, &metricType, &payloadStr); err == nil {
			var payload map[string]interface{}
			if err := json.Unmarshal([]byte(payloadStr), &payload); err == nil {
				record := map[string]interface{}{
					"id":          id,
					"metric_type": metricType,
					"payload":     payload,
				}
				records = append(records, record)
				ids = append(ids, id)
			}
		}
	}

	if len(records) == 0 {
		return
	}

	// Transmit to cloud
	data, err := json.Marshal(records)
	if err != nil {
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.endpoint, bytes.NewBuffer(data))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	resp, err := w.client.Do(req)
	if err != nil {
		slog.Warn("Failed to sync telemetry to cloud", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		// Delete successfully synced records
		tx, err := w.db.Begin(ctx)
		if err != nil {
			return
		}
		defer tx.Rollback(ctx)

		if len(ids) > 0 {
			if w.db.IsSQLite() {
				query := "DELETE FROM telemetry_buffer WHERE id IN ("
				args := make([]interface{}, len(ids))
				for i, id := range ids {
					if i > 0 {
						query += ", "
					}
					query += "?"
					args[i] = id
				}
				query += ")"
				_, _ = tx.Exec(ctx, query, args...)
			} else {
				// Use ANY for PostgreSQL
				// Convert to array of interface to be safe, but driver usually takes []string fine for ANY
				_, _ = tx.Exec(ctx, "DELETE FROM telemetry_buffer WHERE id = ANY($1)", ids)
			}
		}

		_ = tx.Commit(ctx)
	} else {
		slog.Warn("Failed to sync telemetry to cloud", "status", resp.StatusCode)
	}
}

// Interceptor hook for telemetry
func SQLiteBufferMetric(provider db.Provider) func(ctx context.Context, metricType string, payload string) error {
	return func(ctx context.Context, metricType string, payload string) error {
		if os.Getenv("OHC_STANDALONE") != "true" || os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
			return nil
		}

		// In Go backgrounds, do not pass context with cancellation
		bgCtx := context.WithoutCancel(ctx)
		go func() {
			_, err := provider.Exec(bgCtx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)", metricType, payload)
			if err != nil {
				slog.Error("Failed to buffer metric", "error", err)
			}
		}()
		return nil
	}
}
