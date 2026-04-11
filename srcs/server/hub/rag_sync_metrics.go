package hub

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func InitMetrics(meter metric.Meter) error {
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		return err
	}
	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		return err
	}
	return nil
}

type metricsRAGSyncService struct {
	next RAGSyncService
}

func NewMetricsRAGSyncService(next RAGSyncService) RAGSyncService {
	return &metricsRAGSyncService{next: next}
}

func (s *metricsRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return s.next.FetchPendingSyncs(ctx, limit)
}

func (s *metricsRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	err := s.next.MarkSynced(ctx, ids)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
	} else {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return err
}

func (s *metricsRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	err := s.next.ProcessIncomingSync(ctx, records)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
	} else {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return err
}
