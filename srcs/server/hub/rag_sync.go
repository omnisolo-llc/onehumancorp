package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type SyncDaemon struct {
	Service RAGSyncService
}

func NewSyncDaemon(service RAGSyncService) *SyncDaemon {
	return &SyncDaemon{Service: service}
}

func (d *SyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// 1. Fetch pending syncs
			pending, err := d.Service.FetchPendingSyncs(ctx, 100)
			if err != nil {
				continue
			}
			if len(pending) == 0 {
				continue
			}

			// 2. Process incoming sync
			err = d.Service.ProcessIncomingSync(ctx, pending)
			if err != nil {
				continue
			}

			// 3. Mark synced
			var ids []string
			for _, p := range pending {
				ids = append(ids, p.ID)
			}
			_ = d.Service.MarkSynced(ctx, ids)
		}
	}
}

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal   metric.Int64Counter
	RagSyncErrorsTotal      metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		// handle errors smoothly
		_ = err
	}
	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
        _ = err
	}
}
