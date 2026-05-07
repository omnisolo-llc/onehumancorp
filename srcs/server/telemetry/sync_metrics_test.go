package telemetry

import (
	"context"
	"testing"
)

func TestRecordSyncLatency(t *testing.T) {
	ctx := context.Background()
	err := RecordSyncLatency(ctx, 150.0, "Standalone")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordSyncDaemonError(t *testing.T) {
	ctx := context.Background()
	err := RecordSyncDaemonError(ctx, "Standalone", "timeout")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordSyncDaemonBatchSize(t *testing.T) {
	ctx := context.Background()
	err := RecordSyncDaemonBatchSize(ctx, 50, "Standalone")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordSyncPayloadSize(t *testing.T) {
	ctx := context.Background()
	err := RecordSyncPayloadSize(ctx, 1024, "Standalone")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordSyncEscalation(t *testing.T) {
	ctx := context.Background()
	err := RecordSyncEscalation(ctx, "Standalone")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
