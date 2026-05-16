package kairos

import (
	"context"
	"os"
	"sync"
)

var (
	initOnce       sync.Once
	deploymentMode string
)

func initMetrics() {
	initOnce.Do(func() {
		isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"
		if isMultitenant {
			deploymentMode = "Cloud"
		} else {
			deploymentMode = "Standalone"
		}
	})
}

func getDeploymentMode() string {
	initMetrics()
	return deploymentMode
}

func RecordTaskQueueLength(ctx context.Context, mode string, length int64) error {
	return nil
}

func RecordSwarmTaskQueueLength(ctx context.Context, length int64) error {
	return nil
}

func RecordSubAgentQueueDelay(ctx context.Context, delay float64) error {
	return nil
}

func RecordTaskLatency(ctx context.Context, mode string, latency float64) error {
	return nil
}

func RecordTaskThroughput(ctx context.Context, mode string, amount float64) error {
	return nil
}
