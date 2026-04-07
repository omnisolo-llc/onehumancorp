package hub

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncDaemon struct {
	service  RAGSyncService
	ticker   *time.Ticker
	quit     chan struct{}
	cloudURL string
	client   *http.Client
}

func NewRAGSyncDaemon(service RAGSyncService, interval time.Duration) *RAGSyncDaemon {
	// Simple default configuration
	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "https://api.onehumancorp.com"
	}

	return &RAGSyncDaemon{
		service:  service,
		ticker:   time.NewTicker(interval),
		quit:     make(chan struct{}),
		cloudURL: cloudURL,
		client:   &http.Client{Timeout: 10 * time.Second},
	}
}

func (d *RAGSyncDaemon) Start(ctx context.Context) {
	if os.Getenv("OHC_STANDALONE") != "true" {
		return // Only run in standalone mode
	}

	slog.Info("starting RAG Sync Daemon")
	go func() {
		for {
			select {
			case <-d.ticker.C:
				if err := d.syncRound(ctx); err != nil {
					slog.Error("rag sync round failed", "error", err)
				}
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

func (d *RAGSyncDaemon) Stop() {
	close(d.quit)
}

func (d *RAGSyncDaemon) syncRound(ctx context.Context) error {
	records, err := d.service.FetchPendingSyncs(ctx, 100)
	if err != nil {
		return err
	}

	if len(records) == 0 {
		return nil // Nothing to do
	}

	payload, err := json.Marshal(records)
	if err != nil {
		return fmt.Errorf("failed to marshal records: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", d.cloudURL+"/api/v1/internal/rag/sync", bytes.NewBuffer(payload))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	// Assuming SPIFFE/SPIRE mtls in actual env, here we mock basic auth
	req.Header.Set("Content-Type", "application/json")

	start := time.Now()
	resp, err := d.client.Do(req)
	if err != nil {
		telemetry.RecordSyncLatency(ctx, time.Since(start).Seconds())
		return fmt.Errorf("failed to send sync request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		telemetry.RecordSyncLatency(ctx, time.Since(start).Seconds())
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	telemetry.RecordSyncLatency(ctx, time.Since(start).Seconds())

	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	if err := d.service.MarkSynced(ctx, ids); err != nil {
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	return nil
}
