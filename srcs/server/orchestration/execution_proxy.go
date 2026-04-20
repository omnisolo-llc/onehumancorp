package orchestration

import (
	"context"
	"fmt"
	"log/slog"
)

// ExecutionProxy manages the decision-making for task execution locations.
type ExecutionProxy struct {
	resourceMonitor *ResourceMonitor
	cpuThreshold    float64
	memThreshold    float64
}

// NewExecutionProxy creates a new ExecutionProxy.
func NewExecutionProxy(rm *ResourceMonitor, cpuLimit, memLimit float64) *ExecutionProxy {
	if cpuLimit == 0 {
		cpuLimit = 80.0
	}
	if memLimit == 0 {
		memLimit = 85.0
	}
	return &ExecutionProxy{
		resourceMonitor: rm,
		cpuThreshold:    cpuLimit,
		memThreshold:    memLimit,
	}
}

// ShouldBurst decides if a task should be burst to the cloud based on local resource pressure.
func (p *ExecutionProxy) ShouldBurst(ctx context.Context) bool {
	if p.resourceMonitor == nil {
		return false
	}

	cpu, err := p.resourceMonitor.GetCPUUsage(ctx)
	if err == nil && cpu > p.cpuThreshold {
		slog.Info("Resource pressure detected: CPU above threshold", "usage", cpu, "threshold", p.cpuThreshold)
		return true
	}

	mem, err := p.resourceMonitor.GetMemoryUsage(ctx)
	if err == nil && mem > p.memThreshold {
		slog.Info("Resource pressure detected: Memory above threshold", "usage", mem, "threshold", p.memThreshold)
		return true
	}

	return false
}

// ExecuteTask manages the execution of a mission, potentially bursting it to the cloud.
func (p *ExecutionProxy) ExecuteTask(ctx context.Context, missionID string, sip *SIPDB) error {
	if p.ShouldBurst(ctx) {
		slog.Info("Bursting mission to cloud", "mission_id", missionID)
		// 1. Transition mission state to BURSTING locally
		// TriggerBurst already does part of this, but let's use the SIPDB for consistency if needed.
		// For this task, we'll leverage the existing TriggerBurst in mesh_client.go

		err := TriggerBurst(ctx, missionID)
		if err != nil {
			return fmt.Errorf("failed to burst mission %s: %w", missionID, err)
		}
		return nil
	}

	// If not bursting, normal local execution would be triggered here (handled by other parts of the system)
	return nil
}
