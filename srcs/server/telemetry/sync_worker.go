package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncWorker struct {
	db          db.Provider
	endpoint    string
	syncRunning bool
	client      *http.Client
}

func NewSyncWorker(provider db.Provider, endpoint string) *SyncWorker {
	return &SyncWorker{
		db:       provider,
		endpoint: endpoint,
		client:   &http.Client{Timeout: 10 * time.Second},
	}
}

func (w *SyncWorker) Start(ctx context.Context, interval time.Duration) {
	if w.db == nil || w.endpoint == "" || os.Getenv("OHC_STANDALONE") != "true" {
		return
	}

	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				w.sync(ctx)
			}
		}
	}()
}

func (w *SyncWorker) sync(ctx context.Context) {
	if w.syncRunning {
		return
	}
	w.syncRunning = true
	defer func() { w.syncRunning = false }()

	// Fetch up to 100 records
	rows, err := w.db.Query(ctx, "SELECT id, metric_type, payload, created_at FROM telemetry_buffer LIMIT 100")
	if err != nil {
		slog.Error("sync_worker: failed to query telemetry_buffer", "error", err)
		return
	}
	defer rows.Close()

	var records []map[string]interface{}
	var recordIDs []string

	for rows.Next() {
		var id, metricType, payload string
		var createdAt time.Time
		if err := rows.Scan(&id, &metricType, &payload, &createdAt); err != nil {
			continue
		}

		var parsedPayload interface{}
		if err := json.Unmarshal([]byte(payload), &parsedPayload); err != nil {
			parsedPayload = payload
		}

		records = append(records, map[string]interface{}{
			"id":          id,
			"metric_type": metricType,
			"payload":     parsedPayload,
			"created_at":  createdAt,
		})
		recordIDs = append(recordIDs, id)
	}

	if len(records) == 0 {
		return
	}

	batchPayload, err := json.Marshal(map[string]interface{}{
		"metrics": records,
	})
	if err != nil {
		slog.Error("sync_worker: failed to marshal batch payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.endpoint, bytes.NewBuffer(batchPayload))
	if err != nil {
		slog.Error("sync_worker: failed to create request", "error", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	// Try to get token from env for auth
	if token := os.Getenv("OHC_CLOUD_API_TOKEN"); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := w.client.Do(req)
	if err != nil {
		slog.Error("sync_worker: failed to push metrics to cloud", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		// Successfully pushed, delete from buffer
		for _, id := range recordIDs {
			query := "DELETE FROM telemetry_buffer WHERE id = $1"
			if w.db.IsSQLite() {
				query = "DELETE FROM telemetry_buffer WHERE id = ?"
			}
			_, err := w.db.Exec(ctx, query, id)
			if err != nil {
				slog.Error("sync_worker: failed to delete synced record", "id", id, "error", err)
			}
		}
		slog.Info("sync_worker: successfully synced telemetry records", "count", len(recordIDs))
	} else {
		body, _ := io.ReadAll(resp.Body)
		slog.Error("sync_worker: cloud endpoint returned error", "status", resp.Status, "body", string(body))
	}
}
