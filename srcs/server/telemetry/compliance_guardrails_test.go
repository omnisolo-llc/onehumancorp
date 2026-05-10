package telemetry

import (
	"context"
	"os"
	"testing"
)

func TestComplianceGuardrail_McpSyncWorker_LocalSovereignty(t *testing.T) {
	// 1. Force standalone mode but disable telemetry
	os.Setenv("STANDALONE_MODE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("STANDALONE_MODE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	ctx := context.Background()

	// 2. Try to sync using McpSyncWorker
	worker := NewMcpSyncWorker(nil) // Provider is nil to prove it fails before even needing DB
	err := worker.SyncPendingMetrics(ctx)

	if err == nil {
		t.Errorf("Expected SyncPendingMetrics to fail due to Local Sovereignty guardrail, but it succeeded")
	}

	if err.Error() != "telemetry is not enabled" {
		t.Errorf("Expected guardrail error 'telemetry is not enabled', got: %v", err)
	}
}

func TestComplianceGuardrail_LocalBuffer_LocalSovereignty(t *testing.T) {
	// 1. Force standalone mode but disable telemetry
	os.Setenv("STANDALONE_MODE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("STANDALONE_MODE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	ctx := context.Background()

	// 2. Try to sync using TelemetrySyncEngine
	engine := NewTelemetrySyncEngine(nil, "http://dummy")
	err := engine.SyncPendingMetrics(ctx)

	if err == nil {
		t.Errorf("Expected SyncPendingMetrics to fail due to Local Sovereignty guardrail, but it succeeded")
	}

	if err.Error() != "telemetry is not enabled" {
		t.Errorf("Expected guardrail error 'telemetry is not enabled', got: %v", err)
	}
}
