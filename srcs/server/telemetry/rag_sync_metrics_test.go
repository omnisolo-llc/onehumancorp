package telemetry

import (
	"context"
	"os"
	"testing"
)

func TestRAGSyncMetrics(t *testing.T) {
	// Need to initialize the provider or mock the meter
	// For now we test that calling it doesn't panic even when uninitialized
	os.Setenv("OHC_STANDALONE", "false")

	RecordRAGSyncSuccess(context.Background(), 5)
	RecordRAGSyncError(context.Background())
}
