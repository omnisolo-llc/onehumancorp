package telemetry

import (
	"context"
	"testing"
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
	origCacheHits := cacheHitsCounter
	origCacheMisses := cacheMissesCounter
	origRateLimit := RateLimitExceededCount
	origSqliteLock := sqliteLockContentionCounter
	origSqliteRetry := sqliteRetryExhaustedCounter
	origTaskQueue := TaskQueueLengthGauge
	origTaskLatency := TaskProcessingLatency
	origAgentLatency := AgentTransitionLatency
	origSwarmQueue := swarmTaskQueueLengthGauge
	origSwarmLatency := swarmTaskProcessingLatency
	origTaskEnq := taskEnqueuedCounter
	origTaskFail := taskFailedCounter

	// Initialize so they aren't nil
	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	RecordTokenUsage(ctx, "agent1", "role", "model", "type", 10)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordAgentApiCall(ctx, "agent1", "role", "api")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordAgentApiError(ctx, "agent1", "role", "api")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordHumanInteraction(ctx, "type")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordMeetingEvent(ctx, "type")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskCompleted(ctx, "mission1")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	// Additional calls
	RecordCacheHit(ctx, "op", "type")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordCacheMiss(ctx, "op", "type")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordApiRateLimitExceeded(ctx, "ep")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSQLiteLockContention(ctx, "op")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSQLiteRetryExhausted(ctx, "op")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskQueueLength(ctx, 5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskProcessed(ctx, 5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordAgentTransitionLatency(ctx, "type", 1.5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskQueueLength(ctx, 1)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskProcessingLatency(ctx, 1.5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskEnqueued(ctx, "id")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskFailed(ctx, "id", "err")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false
	// Restore
	tokenUsageCounter = origTokenUsage
	agentApiCallsCounter = origAgentCalls
	agentApiErrorsCounter = origAgentErrors
	humanInteractionsCounter = origHuman
	meetingEventsCounter = origMeeting
	swarmTasksCompletedCounter = origSwarm
	cacheHitsCounter = origCacheHits
	cacheMissesCounter = origCacheMisses
	RateLimitExceededCount = origRateLimit
	sqliteLockContentionCounter = origSqliteLock
	sqliteRetryExhaustedCounter = origSqliteRetry
	TaskQueueLengthGauge = origTaskQueue
	TaskProcessingLatency = origTaskLatency
	AgentTransitionLatency = origAgentLatency
	swarmTaskQueueLengthGauge = origSwarmQueue
	swarmTaskProcessingLatency = origSwarmLatency
	taskEnqueuedCounter = origTaskEnq
	taskFailedCounter = origTaskFail
}
