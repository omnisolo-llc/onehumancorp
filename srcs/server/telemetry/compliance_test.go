package telemetry

import (
    "context"
    "strings"
    "testing"
)

func TestStandaloneLocalSovereigntyTelemetry(t *testing.T) {
    t.Run("Opt-Out Default in Standalone Mode", func(t *testing.T) {
        t.Setenv("OHC_STANDALONE", "true")
        t.Setenv("OHC_MULTITENANT", "false")
        t.Setenv("OHC_TELEMETRY_ENABLED", "") // Ensure no explicit opt-in

        cleanup, err := InitTelemetry()
        if err != nil {
            t.Fatalf("InitTelemetry failed: %v", err)
        }
        if cleanup == nil {
            t.Fatal("Expected dummy cleanup, got nil")
        }

        if BufferMetricFunc != nil {
            t.Error("BufferMetricFunc should be nil in standalone mode to prevent telemetry by default")
        }
        cleanup()
    })
}

func TestCloudHybridPrivacyAudit(t *testing.T) {
    t.Run("PII Scrubbing on Buffer Payload", func(t *testing.T) {
        var capturedPayload string
        origFunc := BufferMetricFunc
        defer func() { BufferMetricFunc = origFunc }()

        BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
            capturedPayload = payload
            return nil
        }

        ctx := context.Background()
        RecordAgentApiError(ctx, "agent_123", "test_role", "https://api.openai.com/v1/?email=test@example.com")

        if capturedPayload == "" {
            t.Fatal("Expected payload to be captured via buffer func")
        }

        if !strings.Contains(capturedPayload, "[REDACTED_EMAIL]") {
            t.Errorf("Expected payload to contain REDACTED_EMAIL, got: %s", capturedPayload)
        }
    })
}
