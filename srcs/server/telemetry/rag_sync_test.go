package telemetry

import (
	"context"
	"testing"
)

func TestRAGSyncMetrics(t *testing.T) {
	InitRAGSyncMetrics(nil)

	ctx := context.Background()
	RecordRAGRecordSynced(ctx, 1, "standalone")
	RecordRAGSyncError(ctx, 1, "standalone")
}
