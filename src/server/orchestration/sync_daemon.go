package orchestration

import (
	"bytes"
	"context"
	"database/sql"
	"fmt"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var throttleSemaphore = make(chan struct{}, 10)

var (
	syncSuccessCounter = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Name: "kairos_hybrid_sync_success_total",
			Help: "Total number of successful local-to-cloud sync operations",
		},
		[]string{"mode"},
	)
	syncErrorCounter = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Name: "kairos_hybrid_sync_errors_total",
			Help: "Total number of failed local-to-cloud sync operations",
		},
		[]string{"mode", "error_type"},
	)
	syncLatencyHistogram = promauto.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "kairos_hybrid_sync_latency_seconds",
			Help:    "Latency of local-to-cloud sync operations",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)
)

type HybridMCPRAGDaemon struct {
	db        *sql.DB
	remoteURL string
	client    *http.Client
	mode      string
}

func NewHybridMCPRAGDaemon(db *sql.DB, remoteURL string) *HybridMCPRAGDaemon {
	return &HybridMCPRAGDaemon{
		db:        db,
		remoteURL: remoteURL,
		client:    &http.Client{Timeout: 15 * time.Second},
		mode:      "Standalone SQLite",
	}
}

func (d *HybridMCPRAGDaemon) SyncPendingMissions(ctx context.Context) error {
	rows, err := d.db.QueryContext(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND (status = 'CLOUD_ESCALATION' OR status = 'BURSTING') AND (sync_error IS NULL OR last_synced_at < datetime('now', '-5 minutes')) LIMIT 100")
	if err != nil {
		syncErrorCounter.WithLabelValues(d.mode, "DB Query Error").Inc()
		return fmt.Errorf("sync_daemon: failed to query agent_missions: %w", err)
	}

	type mission struct {
		id      string
		status  string
		payload []byte
	}
	var missions []mission

	for rows.Next() {
		var m mission
		if err := rows.Scan(&m.id, &m.status, &m.payload); err != nil {
			continue
		}
		missions = append(missions, m)
	}

	if err := rows.Err(); err != nil {
		rows.Close()
		syncErrorCounter.WithLabelValues(d.mode, "DB Iteration Error").Inc()
		return fmt.Errorf("sync_daemon: rows iteration error: %w", err)
	}
	rows.Close()

	var syncedCount int

	for _, m := range missions {
		select {
		case throttleSemaphore <- struct{}{}:
		case <-ctx.Done():
			syncErrorCounter.WithLabelValues(d.mode, "Context Canceled").Inc()
			return ctx.Err()
		}

		start := time.Now()
		err = d.syncToCloud(ctx, m.id, m.payload)
		latency := time.Since(start)
		syncLatencyHistogram.WithLabelValues(d.mode).Observe(latency.Seconds())

		if err != nil {
			<-throttleSemaphore
			errorType := "API Error"
			if err.Error() == "API Timeout or network error" {
				errorType = "API Timeout"
			}
			syncErrorCounter.WithLabelValues(d.mode, errorType).Inc()
			_, _ = d.db.ExecContext(ctx, "UPDATE agent_missions SET sync_error = $1, last_synced_at = datetime('now') WHERE id = $2", err.Error(), m.id)
			continue
		}

		_, err = d.db.ExecContext(ctx, "UPDATE agent_missions SET synced_to_cloud = true, sync_error = NULL, last_synced_at = datetime('now') WHERE id = $1", m.id)

		<-throttleSemaphore
		if err != nil {
			syncErrorCounter.WithLabelValues(d.mode, "DB Lock Error").Inc()
			continue
		}

		syncSuccessCounter.WithLabelValues(d.mode).Inc()
		syncedCount++
	}

	return nil
}

func (d *HybridMCPRAGDaemon) syncToCloud(ctx context.Context, id string, payload []byte) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.remoteURL+"/api/v1/sync", bytes.NewReader(payload))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-Mission-ID", id)

	resp, err := d.client.Do(req)
	if err != nil {
		return fmt.Errorf("API Timeout or network error: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		return fmt.Errorf("cloud API returned HTTP %d", resp.StatusCode)
	}

	return nil
}
