package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

func SyncLoop(ctx context.Context, svc RAGSyncService) error {
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		return err
	}
	if len(records) == 0 {
		return nil
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	var ids []string
	for _, rec := range records {
		ids = append(ids, rec.ID)
	}

	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced to cloud"))
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total errors encountered during RAG sync"))
	if err != nil {
		panic(err)
	}
}
