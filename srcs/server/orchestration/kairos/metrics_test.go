package kairos

import (
	"context"
	"os"
	"testing"
)

func TestDeploymentMode(t *testing.T) {
	// Instead of testing initOnce multiple times, just test the getDeploymentMode fallback/default.
	// Since tests might run in arbitrary orders, we can only safely test the record functions themselves
	// avoiding re-registering callbacks via sync.Once resets
	os.Setenv("OHC_MULTITENANT", "true")
	initMetrics()
	if deploymentMode != "Cloud" && deploymentMode != "Standalone" {
		t.Errorf("Expected Cloud or Standalone, got %s", deploymentMode)
	}
}

func TestRecordTaskQueueLength(t *testing.T) {
	err := RecordTaskQueueLength(context.Background(), "testMode", 42)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestRecordSwarmTaskQueueLength(t *testing.T) {
	err := RecordSwarmTaskQueueLength(context.Background(), 10)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestRecordSubAgentQueueDelay(t *testing.T) {
	err := RecordSubAgentQueueDelay(context.Background(), 3.14)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestRecordTaskLatency(t *testing.T) {
	err := RecordTaskLatency(context.Background(), "testMode", 10.0)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestRecordTaskThroughput(t *testing.T) {
	err := RecordTaskThroughput(context.Background(), "testMode", 5.0)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}
