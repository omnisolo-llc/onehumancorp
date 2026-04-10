package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// RAGSyncWorker represents a daemon that runs in Standalone mode
// to sync local pending RAG memories to the cloud.
type RAGSyncWorker struct {
	syncService RAGSyncService
	dbProvider  db.Provider
}

// NewRAGSyncWorker initializes a new sync worker daemon.
func NewRAGSyncWorker(syncService RAGSyncService, dbProvider db.Provider) *RAGSyncWorker {
	return &RAGSyncWorker{
		syncService: syncService,
		dbProvider:  dbProvider,
	}
}

// Start begins the background sync process.
func (w *RAGSyncWorker) Start(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				w.syncToCloud(ctx)
			}
		}
	}()
}

// syncToCloud fetches pending records and simulates sending them to the Cloud API Gateway.
func (w *RAGSyncWorker) syncToCloud(ctx context.Context) {
	// 1. Fetch pending records
	records, err := w.syncService.FetchPendingSyncs(ctx, 100)
	if err != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
		return
	}

	if len(records) == 0 {
		return
	}

	// 2. Simulate Cloud API Gateway Push
	// In a complete implementation, this would involve HTTP POST to the Gateway
	// and waiting for a successful response.
	// For now, we simulate a successful push and mark them as synced.

	var ids []string
	for _, rec := range records {
		ids = append(ids, rec.ID)
	}

	// 3. Mark as Synced locally
	err = w.syncService.MarkSynced(ctx, ids)
	if err != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
}
