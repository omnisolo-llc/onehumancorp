package telemetry

import "context"

func RecordRagSyncSuccess(ctx context.Context, count int64) {
	if RagRecordsSyncedTotal != nil {
		RagRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRagSyncError(ctx context.Context) {
	if RagSyncErrorsTotal != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
	}
}
