package hub

import (
	"context"
	"log"
	"time"

	"go.opentelemetry.io/otel/metric"
)

// RAGSyncWorker periodically fetches pending sync records and pushes them.
type RAGSyncWorker struct {
	svc          RAGSyncService
	syncInterval time.Duration
	batchSize    int
	workerErrors metric.Int64Counter
}

func NewRAGSyncWorker(svc RAGSyncService, meter metric.Meter, syncInterval time.Duration, batchSize int) (*RAGSyncWorker, error) {
	workerErrors, err := meter.Int64Counter("rag_sync_worker_errors_total", metric.WithDescription("Total number of errors encountered by the RAG sync worker"))
	if err != nil {
		return nil, err
	}
	return &RAGSyncWorker{
		svc:          svc,
		syncInterval: syncInterval,
		batchSize:    batchSize,
		workerErrors: workerErrors,
	}, nil
}

func (w *RAGSyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.syncInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			log.Println("RAGSyncWorker stopping...")
			return
		case <-ticker.C:
			w.runSync(ctx)
		}
	}
}

func (w *RAGSyncWorker) runSync(ctx context.Context) {
	records, err := w.svc.FetchPendingSyncs(ctx, w.batchSize)
	if err != nil {
		log.Printf("RAGSyncWorker failed to fetch pending syncs: %v", err)
		w.workerErrors.Add(ctx, 1)
		return
	}

	if len(records) == 0 {
		return
	}

	// Mock pushing to the cloud.
	err = w.pushToCloud(ctx, records)
	if err != nil {
		log.Printf("RAGSyncWorker failed to push records to cloud: %v", err)
		w.workerErrors.Add(ctx, 1)
		return
	}

	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	err = w.svc.MarkSynced(ctx, ids)
	if err != nil {
		log.Printf("RAGSyncWorker failed to mark records as synced: %v", err)
		w.workerErrors.Add(ctx, 1)
	}
}

// pushToCloud simulates pushing data to the cloud gateway.
// In a real implementation, this would involve sending an HTTP request with mutual TLS authentication.
func (w *RAGSyncWorker) pushToCloud(ctx context.Context, records []RAGSyncRecord) error {
	// Simulate cloud push via the process incoming sync logic.
	// For testing the worker we just call ProcessIncomingSync on the same service
	// which will be a no-op if the records exist and were updated by the worker.
	return w.svc.ProcessIncomingSync(ctx, records)
}
