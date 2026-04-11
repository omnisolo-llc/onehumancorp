package hub

import (
	"context"
	"time"
	"log/slog"
)

type RAGSyncDaemon struct {
	service RAGSyncService
	ticker  *time.Ticker
	quit    chan struct{}
}

func NewRAGSyncDaemon(service RAGSyncService, interval time.Duration) *RAGSyncDaemon {
	return &RAGSyncDaemon{
		service: service,
		ticker:  time.NewTicker(interval),
		quit:    make(chan struct{}),
	}
}

func (d *RAGSyncDaemon) Start(ctx context.Context) {
	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.runSyncCycle(ctx)
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

func (d *RAGSyncDaemon) runSyncCycle(ctx context.Context) {
	records, err := d.service.FetchPendingSyncs(ctx, 100)
	if err != nil {
		slog.ErrorContext(ctx, "Failed to fetch pending syncs", "error", err)
		return
	}

	if len(records) == 0 {
		return
	}

	// In a real implementation, we would send these via SPIFFE/SPIRE over mTLS to the API Gateway.
	// For now, as this is the foundational service, we simulate the transport and immediately mark synced
	// to demonstrate the end-to-end local loop.

	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	if err := d.service.MarkSynced(ctx, ids); err != nil {
		slog.ErrorContext(ctx, "Failed to mark records as synced", "error", err)
	}
}
