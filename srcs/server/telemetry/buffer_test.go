package telemetry

import (
	"context"
	"encoding/json"
	"testing"
)

type mockSIPDB struct {
	memory map[string]string
	err    error
}

func (m *mockSIPDB) UpdateMemory(ctx context.Context, key, value string) error {
	if m.err != nil {
		return m.err
	}
	m.memory[key] = value
	return nil
}

func (m *mockSIPDB) SyncMemory(ctx context.Context, key string) (string, error) {
	if m.err != nil {
		return "", m.err
	}
	return m.memory[key], nil
}

func TestTelemetryBufferSync(t *testing.T) {
	SetStandaloneMode(true)
	defer SetStandaloneMode(false)
	ClearBufferedMetrics()

	// Ensure no SIPDB at first
	SetSIPDB(nil)

	// Since we are mocking metric interfaces, RecordTokenUsage relies on them being initialized
	// but for buffer we just test bufferMetric since RecordTokenUsage internally buffers if standalone.
	// Actually we can just call bufferMetric directly for a pure unit test.
	bufferMetric(MetricPayload{Type: "token_usage", Count: 100})
	bufferMetric(MetricPayload{Type: "agent_api_call", API: "search_tool"})

	buffered := GetBufferedMetrics()
	if len(buffered) != 2 {
		t.Fatalf("expected 2 buffered metrics, got %d", len(buffered))
	}

	// Create mock SIPDB
	mockDB := &mockSIPDB{memory: make(map[string]string)}
	SetSIPDB(mockDB)

	// Trigger flush
	err := FlushTelemetry(context.Background())
	if err != nil {
		t.Fatalf("unexpected error during flush: %v", err)
	}

	// Buffer should be empty now
	bufferedAfterFlush := GetBufferedMetrics()
	if len(bufferedAfterFlush) != 0 {
		t.Fatalf("expected buffer to be empty after flush, got %d", len(bufferedAfterFlush))
	}

	// Memory should contain the JSON serialized metrics
	syncData, ok := mockDB.memory["telemetry_sync"]
	if !ok {
		t.Fatalf("expected telemetry_sync key in SIPDB memory")
	}

	var parsed []MetricPayload
	if err := json.Unmarshal([]byte(syncData), &parsed); err != nil {
		t.Fatalf("failed to unmarshal synced data: %v", err)
	}

	if len(parsed) != 2 {
		t.Fatalf("expected 2 parsed metrics from SIPDB memory, got %d", len(parsed))
	}
}
