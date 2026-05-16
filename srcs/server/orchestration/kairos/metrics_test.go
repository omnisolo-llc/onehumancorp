package kairos

import (
	"context"
	"os"
	"testing"
	"sync"
)

func TestDeploymentMode(t *testing.T) {
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

func TestGetDeploymentMode_Coverage(t *testing.T) {
	mode := getDeploymentMode()
	if mode == "" { t.Fatal("mode is empty") }
}

// Add coverage for initMetrics mode branch
func TestInitMetrics_StandaloneBranch(t *testing.T) {
    // Save current values
    oldMode := deploymentMode
    oldMultitenant := os.Getenv("OHC_MULTITENANT")

    // Reset global initOnce to force initMetrics to run again
    initOnce = sync.Once{}
    os.Setenv("OHC_MULTITENANT", "false")

    initMetrics()
    if deploymentMode != "Standalone" { t.Fatal("not Standalone") }

    // Restore
    initOnce = sync.Once{}
    os.Setenv("OHC_MULTITENANT", oldMultitenant)
    deploymentMode = oldMode
}
