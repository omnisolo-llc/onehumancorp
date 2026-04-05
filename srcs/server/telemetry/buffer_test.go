package telemetry

import (
	"context"
	"testing"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestBufferMetricFunc(t *testing.T) {
	called := false
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		called = true
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	ctx := context.Background()

	// Capture originals
	origTokenUsage := tokenUsageCounter
	origAgentCalls := agentApiCallsCounter
	origAgentErrors := agentApiErrorsCounter
	origHuman := humanInteractionsCounter
	origMeeting := meetingEventsCounter
	origSwarm := swarmTasksCompletedCounter

	// Initialize so they aren't nil
	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	RecordTokenUsage(ctx, "agent1", "role", "model", "type", 10)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordAgentApiCall(ctx, "agent1", "role", "api")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordAgentApiError(ctx, "agent1", "role", "api")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordHumanInteraction(ctx, "type")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordMeetingEvent(ctx, "type")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordSwarmTaskCompleted(ctx, "mission1")
	if !called { t.Errorf("expected buffer call") }
	called = false

	// Restore
	tokenUsageCounter = origTokenUsage
	agentApiCallsCounter = origAgentCalls
	agentApiErrorsCounter = origAgentErrors
	humanInteractionsCounter = origHuman
	meetingEventsCounter = origMeeting
	swarmTasksCompletedCounter = origSwarm
}

func newTestProvider(t *testing.T) db.Provider {
    t.Setenv("DATABASE_URL", "sqlite://:memory:")
    t.Setenv("OHC_STANDALONE", "true")
    prov, err := db.New(context.Background())
    if err != nil {
        t.Fatalf("failed to create db provider: %v", err)
    }
    if err := prov.RunMigrations(context.Background()); err != nil {
        t.Fatalf("failed to run migrations: %v", err)
    }
    return prov
}

func TestDefaultBufferMetricFunc(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	provider := newTestProvider(t)
	defer provider.Close()

	SetLocalDB(provider)

	err := defaultBufferMetricFunc(ctx, "test_metric", "{\"key\":\"value\"}")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to count telemetry_buffer: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record in telemetry_buffer, got %d", count)
	}
}

func TestDefaultBufferMetricFunc_Disabled(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	ctx := context.Background()
	provider := newTestProvider(t)
	defer provider.Close()

	SetLocalDB(provider)

	err := defaultBufferMetricFunc(ctx, "test_metric", "{\"key\":\"value\"}")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to count telemetry_buffer: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 records in telemetry_buffer, got %d", count)
	}
}
