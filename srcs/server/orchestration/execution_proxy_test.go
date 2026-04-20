package orchestration

import (
	"context"
	"testing"
)

func TestExecutionProxy_ShouldBurst(t *testing.T) {
	rm := &ResourceMonitor{} // Stub resource monitor

	proxy := NewExecutionProxy(rm, 50.0, 50.0)

	// Default should be false as GetCPUUsage/GetMemoryUsage return 0 on non-linux or if not sampled
	if proxy.ShouldBurst(context.Background()) {
		t.Error("expected ShouldBurst to be false initially")
	}
}

func TestExecutionProxy_ExecuteTask(t *testing.T) {
	// This test would ideally mock TriggerBurst and ResourceMonitor
	// For now, it verifies it doesn't crash
	proxy := NewExecutionProxy(nil, 80.0, 80.0)
	err := proxy.ExecuteTask(context.Background(), "test-mission", nil)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}
