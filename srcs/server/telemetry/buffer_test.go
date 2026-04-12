package telemetry

import (
	"context"
	"testing"
	"time"
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

	RecordCacheHit(ctx, "op", "type")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordCacheMiss(ctx, "op", "type")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordApiRateLimitExceeded(ctx, "endpoint")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordSQLiteLockContention(ctx, "op")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordSQLiteRetryExhausted(ctx, "op")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordTaskQueueLength(ctx, 5)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordTaskProcessed(ctx, 100*time.Millisecond)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordAgentTransitionLatency(ctx, "type", 1.5)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordSwarmTaskQueueLength(ctx, 1)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordSwarmTaskProcessingLatency(ctx, 50.0)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordTaskEnqueued(ctx, "task1")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordTaskFailed(ctx, "task1", "err")
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordToolAutoCorrection(ctx, "agent1", "role", true)
	if !called { t.Errorf("expected buffer call") }
	called = false

	RecordDeliberationPhaseDuration(ctx, "plan1", "phase", 10.0)
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
