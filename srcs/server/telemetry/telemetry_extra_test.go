package telemetry

import (
	"context"
	"testing"
	"time"
	"os"
	"fmt"
	"github.com/prometheus/client_golang/prometheus"
)

func TestRecordOtherMetrics(t *testing.T) {
	origReg := prometheus.DefaultRegisterer
	defer func() { prometheus.DefaultRegisterer = origReg }()
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	t.Setenv("OHC_MULTITENANT", "false")

	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}
	defer cleanup()

	ctx := context.Background()

	RecordAgentApiError(ctx, "agent-1", "dev", "api-1")
	RecordCacheHit(ctx, "op1", "type1")
	RecordApiRateLimitExceeded(ctx, "endpoint1")
	RecordTeammateMeshBroadcast(ctx, "channel1")
	RecordTeammateMeshDirectMessage(ctx)
	RecordAutoDreamMemoryIngested(ctx, "agent1")
	RecordAutoDreamMemoryCompressed(ctx, "agent1")
	RecordTaskQueueLength(ctx, 10)
	RecordTaskProcessed(ctx, 5 * time.Second)
	RecordSyncEscalation(ctx, 1)
	RecordSyncLatency(ctx, 1.5)
	RecordSyncPayloadSize(ctx, 100)
	RecordSyncDaemonBatchSize(ctx, 50)
	RecordSwarmTaskTransition(ctx, "mission1", "open", "done")
	RecordSwarmTaskQueueLength(ctx, 1)
	RecordSwarmTaskProcessingLatency(ctx, 100.5)
	RecordTaskEnqueued(ctx, "task1")
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
}

func TestRecordOtherMetricsUninitialized(t *testing.T) {
	ctx := context.Background()

	// Capture originals
	origAgentApiErrors := agentApiErrorsCounter
	origCacheHits := cacheHitsCounter
	origRateLimit := RateLimitExceededCount
	origMeshBroadcast := TeammateMeshBroadcastsCounter
	origMeshDirect := TeammateMeshDirectMessagesCounter
	origAutoDreamIngest := AutoDreamMemoriesIngestedCounter
	origAutoDreamCompress := AutoDreamMemoriesCompressedCounter
	origTaskQueue := TaskQueueLengthGauge
	origTaskProcess := TaskProcessingLatency
	origSyncEscalation := SyncEscalationsCount
	origSyncLatency := SyncLatency
	origSyncPayload := SyncPayloadSize
	origSyncDaemon := syncDaemonBatchSize
	origSwarmTrans := swarmTaskTransitionsCounter
	origSwarmQueue := swarmTaskQueueLengthGauge
	origSwarmProcess := swarmTaskProcessingLatency
	origTaskEnq := taskEnqueuedCounter
	origTaskFail := taskFailedCounter
	origCacheMiss := cacheMissesCounter

	// Nullify all
	agentApiErrorsCounter = nil
	cacheHitsCounter = nil
	RateLimitExceededCount = nil
	TeammateMeshBroadcastsCounter = nil
	TeammateMeshDirectMessagesCounter = nil
	AutoDreamMemoriesIngestedCounter = nil
	AutoDreamMemoriesCompressedCounter = nil
	TaskQueueLengthGauge = nil
	TaskProcessingLatency = nil
	SyncEscalationsCount = nil
	SyncLatency = nil
	SyncPayloadSize = nil
	syncDaemonBatchSize = nil
	swarmTaskTransitionsCounter = nil
	swarmTaskQueueLengthGauge = nil
	swarmTaskProcessingLatency = nil
	taskEnqueuedCounter = nil
	taskFailedCounter = nil
	cacheMissesCounter = nil

	defer func() {
		agentApiErrorsCounter = origAgentApiErrors
		cacheHitsCounter = origCacheHits
		RateLimitExceededCount = origRateLimit
		TeammateMeshBroadcastsCounter = origMeshBroadcast
		TeammateMeshDirectMessagesCounter = origMeshDirect
		AutoDreamMemoriesIngestedCounter = origAutoDreamIngest
		AutoDreamMemoriesCompressedCounter = origAutoDreamCompress
		TaskQueueLengthGauge = origTaskQueue
		TaskProcessingLatency = origTaskProcess
		SyncEscalationsCount = origSyncEscalation
		SyncLatency = origSyncLatency
		SyncPayloadSize = origSyncPayload
		syncDaemonBatchSize = origSyncDaemon
		swarmTaskTransitionsCounter = origSwarmTrans
		swarmTaskQueueLengthGauge = origSwarmQueue
		swarmTaskProcessingLatency = origSwarmProcess
		taskEnqueuedCounter = origTaskEnq
		taskFailedCounter = origTaskFail
		cacheMissesCounter = origCacheMiss
	}()

	RecordAgentApiError(ctx, "agent-1", "dev", "api-1")
	RecordCacheHit(ctx, "op1", "type1")
	RecordApiRateLimitExceeded(ctx, "endpoint1")
	RecordTeammateMeshBroadcast(ctx, "channel1")
	RecordTeammateMeshDirectMessage(ctx)
	RecordAutoDreamMemoryIngested(ctx, "agent1")
	RecordAutoDreamMemoryCompressed(ctx, "agent1")
	RecordTaskQueueLength(ctx, 10)
	RecordTaskProcessed(ctx, 5 * time.Second)
	RecordSyncEscalation(ctx, 1)
	RecordSyncLatency(ctx, 1.5)
	RecordSyncPayloadSize(ctx, 100)
	RecordSyncDaemonBatchSize(ctx, 50)
	RecordSwarmTaskTransition(ctx, "mission1", "open", "done")
	RecordSwarmTaskQueueLength(ctx, 1)
	RecordSwarmTaskProcessingLatency(ctx, 100.5)
	RecordTaskEnqueued(ctx, "task1")
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
}

func TestLogAgentExecutionFallback(t *testing.T) {
	ctx := context.Background()
	// Pass an invalid JSON so it falls back to RedactPII string
	invalidJSON := "not json user@example.com"
	LogAgentExecution(ctx, "agent1", "role", "api", "event", invalidJSON)

	// Pass a valid JSON that can't be marshalled after parsing (mocking is hard, but we test the valid json string)
	validJSON := `{"email": "user@example.com"}`
	LogAgentExecution(ctx, "agent1", "role", "api", "event", validJSON)
}

func TestMinimaxMetrics(t *testing.T) {
	// Call init and record to cover minimax_metrics.go
	origReg := prometheus.DefaultRegisterer
	defer func() { prometheus.DefaultRegisterer = origReg }()
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	t.Setenv("OHC_MULTITENANT", "false")

	cleanup, _ := InitTelemetry()
	if cleanup != nil {
		defer cleanup()
	}

	RecordMinimaxCall(context.Background(), "model1", 1.5, fmt.Errorf("error"))
}

func TestMinimaxMetricsUninitialized(t *testing.T) {
	origMinimaxCalls := minimaxCallsCounter
	minimaxCallsCounter = nil
	RecordMinimaxCall(context.Background(), "model1", 1.5, nil)
	minimaxCallsCounter = origMinimaxCalls
}

func TestRecordAgentExecutionTrace(t *testing.T) {
	// Enable metrics for this test
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	InitWithMeter(meter)

	// If metrics initialization failed (e.g., no provider setup), skip the active record
	if agentExecutionTracesTotal == nil {
		t.Skip("Metrics provider not initialized, skipping RecordAgentExecutionTrace test")
	}

	ctx := context.Background()
	RecordAgentExecutionTrace(ctx, "agent-456", "deliberation")
	// Since we are mocking the meter under the hood in proper tests, this just verifies no panic.
}

func TestPushMetrics(t *testing.T) {
	ctx := context.Background()
	os.Setenv("PROMETHEUS_PUSHGATEWAY_URL", "") // Empty should return nil
	t.Cleanup(func() { os.Unsetenv("PROMETHEUS_PUSHGATEWAY_URL") })
	err := PushMetrics(ctx, "test_job")
	if err != nil {
		t.Fatalf("Expected nil when URL is empty, got: %v", err)
	}
}
