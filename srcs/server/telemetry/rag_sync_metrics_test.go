package telemetry

import (
	"context"
	"testing"
)

func TestRecordRAGRecordsSynced(t *testing.T) {
	ctx := context.Background()

	var bufferedMetricType string
	var bufferedPayload string

	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		bufferedMetricType = metricType
		bufferedPayload = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	RecordRAGRecordsSynced(ctx, 42)

	if bufferedMetricType != "rag_records_synced_total" {
		t.Errorf("Expected metricType 'rag_records_synced_total', got %q", bufferedMetricType)
	}
	expectedPayload := `{"count":42}`
	if bufferedPayload != expectedPayload {
		t.Errorf("Expected payload %q, got %q", expectedPayload, bufferedPayload)
	}
}

func TestRecordRAGSyncError(t *testing.T) {
	ctx := context.Background()

	var bufferedMetricType string
	var bufferedPayload string

	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		bufferedMetricType = metricType
		bufferedPayload = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	RecordRAGSyncError(ctx, "network timeout")

	if bufferedMetricType != "rag_sync_errors_total" {
		t.Errorf("Expected metricType 'rag_sync_errors_total', got %q", bufferedMetricType)
	}
	expectedPayload := `{"error":"network timeout"}`
	if bufferedPayload != expectedPayload {
		t.Errorf("Expected payload %q, got %q", expectedPayload, bufferedPayload)
	}
}
