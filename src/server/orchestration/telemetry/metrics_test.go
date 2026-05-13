package telemetry

import (
	"testing"
)

func TestMetricsRegistration(t *testing.T) {
	// Simple test to ensure functions do not panic
	RecordSyncDaemonBatchSize("Standalone", 10)
	RecordSyncLatency("Standalone", 100.5)
	RecordSyncPayloadSize("Standalone", 1024.0)
	RecordSyncDaemonError("Standalone", "timeout")
}
