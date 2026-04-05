package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MetricSyncDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewMetricSyncDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *MetricSyncDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CLOUD_TELEMETRY_ENDPOINT")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}

	return &MetricSyncDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *MetricSyncDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		// Only run in standalone/SQLite mode
		slog.Debug("sync_daemon: MetricSyncDaemon disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessSync(ctx)
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-ctx.Done():
				d.ticker.Stop()
				return
			}
		}
	}()
}

func (d *MetricSyncDaemon) Stop() {
	close(d.quit)
}

func (d *MetricSyncDaemon) ProcessSync(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}

	for {
		start := time.Now()

		// Query buffered metrics. Since we're syncing them, we can just use a regular query.
		// We'll limit it to 500 rows at a time.
		rows, err := d.dbWrapper.Query(ctx, "SELECT id, metric_type, payload FROM telemetry_buffer ORDER BY id ASC LIMIT 500")
		if err != nil {
			slog.Error("sync_daemon: failed to query telemetry_buffer", "error", err)
			return
		}

		var records []struct {
			id         int64
			metricType string
			payload    string
		}
		for rows.Next() {
			var id int64
			var metricType string
			var payload string
			if err := rows.Scan(&id, &metricType, &payload); err != nil {
				slog.Error("sync_daemon: failed to scan telemetry_buffer row", "error", err)
				continue
			}
			records = append(records, struct {
				id         int64
				metricType string
				payload    string
			}{id, metricType, payload})
		}

		// Ensure we close rows before proceeding with HTTP and further DB calls.
		rows.Close()

		if len(records) == 0 {
			return
		}

		var payloadBuilder strings.Builder
		payloadBuilder.WriteString("[")
		var idsToDelete []string
		for i, rec := range records {
			if i > 0 {
				payloadBuilder.WriteString(",")
			}
			// Try to inject metric_type if it's JSON object
			var obj map[string]interface{}
			if err := json.Unmarshal([]byte(rec.payload), &obj); err == nil {
				obj["metric_type"] = rec.metricType
				// Ensure it is redacted, although BufferMetricFunc already did this.
				redactedObj := RedactInterfacePII(obj)
				b, _ := json.Marshal(redactedObj)
				payloadBuilder.Write(b)
			} else {
				// fallback
				payloadBuilder.WriteString(RedactPII(rec.payload))
			}
			idsToDelete = append(idsToDelete, fmt.Sprintf("%d", rec.id))
		}
		payloadBuilder.WriteString("]")

		payloadBytes := []byte(payloadBuilder.String())

		if err := d.sendToCloud(ctx, payloadBytes); err != nil {
			slog.Error("sync_daemon: failed to send metrics to cloud", "error", err)
			return
		}

		// Delete successfully synced records
		if len(idsToDelete) > 0 {
			idList := strings.Join(idsToDelete, ",")
			_, err := d.dbWrapper.Exec(ctx, fmt.Sprintf("DELETE FROM telemetry_buffer WHERE id IN (%s)", idList))
			if err != nil {
				slog.Error("sync_daemon: failed to delete synced telemetry_buffer records", "error", err)
				return
			}
		}

		RecordSyncDaemonBatchSize(ctx, int64(len(records)))
		RecordSyncLatency(ctx, float64(time.Since(start).Milliseconds()))

		slog.Debug("sync_daemon: successfully synced metrics", "count", len(records))

		if len(records) < 500 {
			break
		}
	}
}

func (d *MetricSyncDaemon) sendToCloud(ctx context.Context, payloadBytes []byte) error {
	RecordSyncPayloadSize(ctx, int64(len(payloadBytes)))

	syncEndpoint := fmt.Sprintf("%s/api/telemetry/sync", d.cloudAPIURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(payloadBytes))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}
