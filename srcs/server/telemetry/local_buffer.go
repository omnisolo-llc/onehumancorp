package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/google/uuid"
)

// MetricPoint represents a single OpenTelemetry metric data point to be buffered
type MetricPoint struct {
	ID            string                 `json:"id"`
	MetricName    string                 `json:"metric_name"`
	Value         float64                `json:"value"`
	Attributes    map[string]interface{} `json:"attributes"`
	Timestamp     time.Time              `json:"timestamp"`
	SyncedToCloud bool                   `json:"synced_to_cloud"`
}

// TelemetrySyncEngine handles buffering telemetry data locally
// and syncing it to the cloud when online.
type TelemetrySyncEngine struct {
	db          *sql.DB
	remoteURL   string
	httpClient  *http.Client
}

func NewTelemetrySyncEngine(db *sql.DB, remoteURL string) *TelemetrySyncEngine {
	return &TelemetrySyncEngine{
		db:         db,
		remoteURL:  remoteURL,
		httpClient: &http.Client{Timeout: 5 * time.Second},
	}
}

// BufferMetric stores a metric locally in SQLite
func (e *TelemetrySyncEngine) BufferMetric(ctx context.Context, name string, value float64, attrs map[string]interface{}) error {
	id := uuid.New().String()
	attrBytes, err := json.Marshal(attrs)
	if err != nil {
		return fmt.Errorf("failed to marshal attributes: %w", err)
	}

	query := `INSERT INTO local_telemetry_metrics (id, metric_name, value, attributes, timestamp, synced_to_cloud)
	          VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, FALSE)`
	_, err = e.db.ExecContext(ctx, query, id, name, value, string(attrBytes))
	if err != nil {
		return fmt.Errorf("failed to insert metric: %w", err)
	}
	return nil
}

// SyncPendingMetrics attempts to send buffered metrics to the cloud observability endpoint
func (e *TelemetrySyncEngine) SyncPendingMetrics(ctx context.Context) error {
	rows, err := e.db.QueryContext(ctx, "SELECT id, metric_name, value, attributes, timestamp FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE LIMIT 100")
	if err != nil {
		return fmt.Errorf("failed to query pending metrics: %w", err)
	}
	defer rows.Close()

	var pending []MetricPoint
	for rows.Next() {
		var pt MetricPoint
		var attrStr string
		if err := rows.Scan(&pt.ID, &pt.MetricName, &pt.Value, &attrStr, &pt.Timestamp); err != nil {
			log.Printf("failed to scan metric row: %v", err)
			continue
		}
		if err := json.Unmarshal([]byte(attrStr), &pt.Attributes); err != nil {
			log.Printf("failed to unmarshal attributes for metric %s: %v", pt.ID, err)
			continue
		}
		pending = append(pending, pt)
	}
	if err := rows.Err(); err != nil {
		return fmt.Errorf("rows iteration error: %w", err)
	}

	for _, pt := range pending {
		if err := e.syncToCloud(ctx, pt); err != nil {
			log.Printf("failed to sync metric %s: %v", pt.ID, err)
			continue
		}
		// Mark synced
		_, err := e.db.ExecContext(ctx, "UPDATE local_telemetry_metrics SET synced_to_cloud = TRUE WHERE id = $1", pt.ID)
		if err != nil {
			log.Printf("failed to mark metric %s as synced: %v", pt.ID, err)
		}
	}

	return nil
}

func (e *TelemetrySyncEngine) syncToCloud(ctx context.Context, pt MetricPoint) error {
	payload, err := json.Marshal(pt)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, e.remoteURL, bytes.NewReader(payload))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := e.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	return nil
}

// StartSyncDaemon periodically attempts to flush the local SQLite telemetry table
func (e *TelemetrySyncEngine) StartSyncDaemon(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	e.SyncPendingMetrics(ctx)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := e.SyncPendingMetrics(ctx); err != nil {
				log.Printf("error syncing metrics: %v", err)
			}
		}
	}
}
