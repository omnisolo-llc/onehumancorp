package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncDaemonPayload struct {
	ID      string `json:"id"`
	Status  string `json:"status"`
	Payload string `json:"payload"`
}

type HybridMCPRAGDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewHybridMCPRAGDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *HybridMCPRAGDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}

	return &HybridMCPRAGDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *HybridMCPRAGDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		// Only run in standalone/SQLite mode
		slog.Debug("sync_daemon: HybridMCPRAGDaemon disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-ctx.Done():
				d.ticker.Stop()
				return
			default:
				processed := d.ProcessSync(ctx)
				if !processed {
					time.Sleep(1 * time.Second)
				}
			}
		}
	}()
}

func (d *HybridMCPRAGDaemon) Stop() {
	close(d.quit)
}

func (d *HybridMCPRAGDaemon) ProcessSync(ctx context.Context) bool {
	if !d.dbWrapper.IsSQLite() {
		return false
	}
	start := time.Now()

	err := d.PushPendingMissions(ctx)
	if err != nil {
		slog.Error("sync_daemon: PushPendingMissions failed", "error", err)
	}

	err = d.PullMissionUpdates(ctx)
	if err != nil {
		slog.Error("sync_daemon: PullMissionUpdates failed", "error", err)
	}

	telemetry.RecordSyncLatency(ctx, float64(time.Since(start).Milliseconds()))
	return true
}

func (d *HybridMCPRAGDaemon) sendToCloud(ctx context.Context, payloads []SyncDaemonPayload) error {
	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}
	telemetry.RecordSyncPayloadSize(ctx, int64(len(jsonData)))

	syncEndpoint := fmt.Sprintf("%s/api/sync/missions", d.cloudAPIURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

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
