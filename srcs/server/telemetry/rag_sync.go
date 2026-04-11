package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	if meter == nil {
		return
	}

	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		// Log or handle error? Usually init errors for telemetry just panic or are ignored
	}
	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		// Log or handle error?
	}
}

// RecordRagRecordsSynced increments the rag_records_synced_total counter.
func RecordRagRecordsSynced(ctx context.Context, count int) {
	if RagRecordsSyncedTotal != nil {
		RagRecordsSyncedTotal.Add(ctx, int64(count))
	}
}

// RecordRagSyncError increments the rag_sync_errors_total counter.
func RecordRagSyncError(ctx context.Context) {
	if RagSyncErrorsTotal != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
	}
}
