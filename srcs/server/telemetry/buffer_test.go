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

	RecordCacheHit(ctx, "op1", "type1")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordCacheMiss(ctx, "op1", "type1")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordApiRateLimitExceeded(ctx, "/test")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSQLiteLockContention(ctx, "write")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSQLiteRetryExhausted(ctx, "write")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordPostgresRetryExhausted(ctx, "write")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordPostgresLockContention(ctx, "write")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskQueueLength(ctx, 5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskProcessed(ctx, 10)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordAgentTransitionLatency(ctx, "trans1", 1.5)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskQueueLength(ctx, 3)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskProcessingLatency(ctx, 150)
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskEnqueued(ctx, "task1")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordTaskFailed(ctx, "task1", "err1")
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
}
