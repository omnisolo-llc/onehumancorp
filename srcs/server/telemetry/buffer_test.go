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

	RecordSQLiteThrottledRequest(ctx, "write")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSQLiteRetryEvent(ctx, "write", "standalone")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordPostgresRetryExhausted(ctx, "write")
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

	RecordSwarmTaskQueueLength(ctx, 3, "standalone")
	if !called {
		t.Errorf("expected buffer call")
	}
	called = false

	RecordSwarmTaskProcessingLatency(ctx, 150, "standalone")
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

	RecordTeammateMeshBroadcast(ctx, "ch1")
	if !called {
		t.Errorf("expected buffer call for RecordTeammateMeshBroadcast")
	}
	called = false

	RecordTeammateMeshDirectMessage(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordTeammateMeshDirectMessage")
	}
	called = false

	RecordSyncEscalation(ctx, 1)
	if !called {
		t.Errorf("expected buffer call for RecordSyncEscalation")
	}
	called = false

	RecordSyncLatency(ctx, 1.5)
	if !called {
		t.Errorf("expected buffer call for RecordSyncLatency")
	}
	called = false

	RecordSyncPayloadSize(ctx, 100)
	if !called {
		t.Errorf("expected buffer call for RecordSyncPayloadSize")
	}
	called = false

	RecordSyncDaemonBatchSize(ctx, 10)
	if !called {
		t.Errorf("expected buffer call for RecordSyncDaemonBatchSize")
	}
	called = false

	RecordSwarmTaskTransition(ctx, "task1", "old", "new")
	if !called {
		t.Errorf("expected buffer call for RecordSwarmTaskTransition")
	}
	called = false

	RecordAutoDreamSyncLatency(ctx, 1.2, "mode1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamSyncLatency")
	}
	called = false

	RecordAutoDreamQueryLatency(ctx, 1.5, "mode2")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamQueryLatency")
	}
	called = false

	RecordSIPSyncLatency(ctx, 100)
	if !called {
		t.Errorf("expected buffer call for RecordSIPSyncLatency")
	}
	called = false

	RecordSIPSyncPayloadSize(ctx, 200)
	if !called {
		t.Errorf("expected buffer call for RecordSIPSyncPayloadSize")
	}
	called = false

	RecordMeshBroadcast(ctx, "mode")
	if !called {
		t.Errorf("expected buffer call for RecordMeshBroadcast")
	}
	called = false

	RecordMeshLatency(ctx, "op", 150)
	if !called {
		t.Errorf("expected buffer call for RecordMeshLatency")
	}
	called = false

	RecordSubAgentExecutionDuration(ctx, 2.5)
	if !called {
		t.Errorf("expected buffer call for RecordSubAgentExecutionDuration")
	}
	called = false

	RecordSubAgentFailure(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordSubAgentFailure")
	}
	called = false

	RecordIdentityVerification(ctx, true)
	if !called {
		t.Errorf("expected buffer call for RecordIdentityVerification")
	}
	called = false

	RecordSyncConflictResolved(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordSyncConflictResolved")
	}
	called = false

	RecordOmniContextBytes(ctx, 500)
	if !called {
		t.Errorf("expected buffer call for RecordOmniContextBytes")
	}
	called = false

	RecordRagEscalation(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordRagEscalation")
	}
	called = false

	RecordBubblewrapSpawn(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordBubblewrapSpawn")
	}
	called = false

	RecordBubblewrapExecutionLatency(ctx, 0.5)
	if !called {
		t.Errorf("expected buffer call for RecordBubblewrapExecutionLatency")
	}
	called = false

	RecordHarnessInitLatency(ctx, 1.0, "mode")
	if !called {
		t.Errorf("expected buffer call for RecordHarnessInitLatency")
	}
	called = false

	RecordHarnessDbIoLatency(ctx, 0.2, "mode")
	if !called {
		t.Errorf("expected buffer call for RecordHarnessDbIoLatency")
	}
	called = false

	RecordHarnessExecutionLatency(ctx, 1.5, "mode")
	if !called {
		t.Errorf("expected buffer call for RecordHarnessExecutionLatency")
	}
	called = false

	RecordCapabilityViolation(ctx, "session1", "cap1")
	if !called {
		t.Errorf("expected buffer call for RecordCapabilityViolation")
	}
	called = false

	RecordTelemetrySyncBackoff(ctx, 5.0)
	if !called {
		t.Errorf("expected buffer call for RecordTelemetrySyncBackoff")
	}
	called = false

	RecordTelemetryBatchSize(ctx, 50)
	if !called {
		t.Errorf("expected buffer call for RecordTelemetryBatchSize")
	}
	called = false

	RecordAgentTokenUsage(ctx, "ag1", "org1", "role1", "mod1", 100)
	if !called {
		t.Errorf("expected buffer call for RecordAgentTokenUsage")
	}
	called = false

	RecordAgentCost(ctx, "ag1", "org1", "role1", "mod1", 0.5)
	if !called {
		t.Errorf("expected buffer call for RecordAgentCost")
	}
	called = false

	RecordTokenBurnRatePredicted24h(ctx, "org1", 50.0)
	if !called {
		t.Errorf("expected buffer call for RecordTokenBurnRatePredicted24h")
	}
	called = false

	RecordTokenBurnRate(ctx, "org1", 1.5)
	if !called {
		t.Errorf("expected buffer call for RecordTokenBurnRate")
	}
	called = false

	RecordUSDBurnRate(ctx, "org1", 0.05)
	if !called {
		t.Errorf("expected buffer call for RecordUSDBurnRate")
	}
	called = false

	RecordTokensSaved(ctx, "op1", "c1", 500)
	if !called {
		t.Errorf("expected buffer call for RecordTokensSaved")
	}
	called = false

	RecordAutoDreamMemoryIngested(ctx, "ag1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamMemoryIngested")
	}
	called = false

	RecordAutoDreamConsolidation(ctx, "ag1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamConsolidation")
	}
	called = false

	RecordAutoDreamMemoryCompressed(ctx, "ag1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamMemoryCompressed")
	}
	called = false

	RecordPostgresLockContention(ctx, "op1")
	if !called {
		t.Errorf("expected buffer call for RecordPostgresLockContention")
	}
	called = false

	RecordLLMNetworkLatency(ctx, "mod1", 1.2)
	if !called {
		t.Errorf("expected buffer call for RecordLLMNetworkLatency")
	}
	called = false

	RecordLocalToCloudMissionSync(ctx, "m1")
	if !called {
		t.Errorf("expected buffer call for RecordLocalToCloudMissionSync")
	}
	called = false

	RecordQueueLength(ctx, 5)
	if !called {
		t.Errorf("expected buffer call for RecordQueueLength")
	}
	called = false

	RecordToolAutoCorrection(ctx, "ag1", "rol1", true)
	if !called {
		t.Errorf("expected buffer call for RecordToolAutoCorrection")
	}
	called = false

	RecordDeliberationPhaseDuration(ctx, "p1", "ph1", 5.0)
	if !called {
		t.Errorf("expected buffer call for RecordDeliberationPhaseDuration")
	}
	called = false

	RecordAgentExecutionTrace(ctx, "ag1", "tr1")
	if !called {
		t.Errorf("expected buffer call for RecordAgentExecutionTrace")
	}
	called = false

	RecordAutoDreamIngestionError(ctx, "ag1", "err1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamIngestionError")
	}
	called = false

	RecordAutoDreamCompressionError(ctx, "ag1", "err1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamCompressionError")
	}
	called = false

	RecordSubAgentQueueDelay(ctx, 1.5)
	if !called {
		t.Errorf("expected buffer call for RecordSubAgentQueueDelay")
	}
	called = false

	RecordTaskClaimContention(ctx, "mode1")
	if !called {
		t.Errorf("expected buffer call for RecordTaskClaimContention")
	}
	called = false

	RecordSandboxViolation(ctx, "v1", "ag1", "p1")
	if !called {
		t.Errorf("expected buffer call for RecordSandboxViolation")
	}
	called = false

	RecordAutoDreamSyncSuccess(ctx, "ag1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamSyncSuccess")
	}
	called = false

	RecordAutoDreamSyncError(ctx, "ag1", "err1")
	if !called {
		t.Errorf("expected buffer call for RecordAutoDreamSyncError")
	}
	called = false

	RecordBubblewrapViolation(ctx)
	if !called {
		t.Errorf("expected buffer call for RecordBubblewrapViolation")
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
