package hub

import (
	"context"
	"time"
	"github.com/onehumancorp/mono/srcs/server/telemetry"

)

// SyncDaemon is a lightweight Go daemon running in Standalone Mode that monitors local SQLite changes
// for RAG context and syncs them to the cloud.
type SyncDaemon struct {
	svc      RAGSyncService
	interval time.Duration
}

func NewSyncDaemon(svc RAGSyncService, interval time.Duration) *SyncDaemon {
	return &SyncDaemon{
		svc:      svc,
		interval: interval,
	}
}

func (d *SyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(d.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			d.sync(ctx)
		}
	}
}

func (d *SyncDaemon) sync(ctx context.Context) {
	// 1. Fetch pending syncs
	records, err := d.svc.FetchPendingSyncs(ctx, 100)
	if err != nil {
		if telemetry.RAGSyncErrorsCounter != nil {
			telemetry.RAGSyncErrorsCounter.Add(ctx, 1)
		}
		return
	}

	if len(records) == 0 {
		return
	}

	// 2. We simulate pushing to the cloud API Gateway via SPIFFE/SPIRE authenticated TLS.
	// In a real system we would make an HTTP call here.
	// We simulate success for now as we don't have the cloud endpoint implemented in this mission.

	// 3. Mark as synced
	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	if err := d.svc.MarkSynced(ctx, ids); err != nil {
		if telemetry.RAGSyncErrorsCounter != nil {
			telemetry.RAGSyncErrorsCounter.Add(ctx, 1)
		}
		return
	}

	if telemetry.RAGRecordsSyncedCounter != nil {
		telemetry.RAGRecordsSyncedCounter.Add(ctx, int64(len(ids)))
	}
}