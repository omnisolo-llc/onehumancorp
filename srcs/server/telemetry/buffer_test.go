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



	RecordCacheHit(ctx, "op1", "cache1")
	if !called { t.Errorf("expected buffer call for RecordCacheHit") }
	called = false

	RecordCacheMiss(ctx, "op2", "cache2")
	if !called { t.Errorf("expected buffer call for RecordCacheMiss") }
	called = false

	RecordApiRateLimitExceeded(ctx, "endpoint1")
	if !called { t.Errorf("expected buffer call for RecordApiRateLimitExceeded") }
	called = false

	RecordSQLiteLockContention(ctx, "op3")
	if !called { t.Errorf("expected buffer call for RecordSQLiteLockContention") }
	called = false

	RecordSQLiteRetryExhausted(ctx, "op4")
	if !called { t.Errorf("expected buffer call for RecordSQLiteRetryExhausted") }
	called = false

	RecordTaskQueueLength(ctx, 10)
	if !called { t.Errorf("expected buffer call for RecordTaskQueueLength") }
	called = false

	RecordTaskProcessed(ctx, 100 * time.Millisecond)
	if !called { t.Errorf("expected buffer call for RecordTaskProcessed") }
	called = false

	RecordAgentTransitionLatency(ctx, "trans1", 1.5)
	if !called { t.Errorf("expected buffer call for RecordAgentTransitionLatency") }
	called = false

	RecordSwarmTaskQueueLength(ctx, 5)
	if !called { t.Errorf("expected buffer call for RecordSwarmTaskQueueLength") }
	called = false

	RecordSwarmTaskProcessingLatency(ctx, 2.5)
	if !called { t.Errorf("expected buffer call for RecordSwarmTaskProcessingLatency") }
	called = false

	RecordTaskEnqueued(ctx, "task1")
	if !called { t.Errorf("expected buffer call for RecordTaskEnqueued") }
	called = false

	RecordTaskFailed(ctx, "task1", "err1")
	if !called { t.Errorf("expected buffer call for RecordTaskFailed") }
	called = false

	// Restore
	tokenUsageCounter = origTokenUsage
	agentApiCallsCounter = origAgentCalls
	agentApiErrorsCounter = origAgentErrors
	humanInteractionsCounter = origHuman
	meetingEventsCounter = origMeeting
	swarmTasksCompletedCounter = origSwarm
}
