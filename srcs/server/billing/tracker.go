package billing

import (
	"context"
	"onehumancorp/srcs/server/telemetry"
)

// Tracker tracks billing and costs.
type Tracker struct{}

// NewTracker creates a new Tracker.
func NewTracker() *Tracker {
	return &Tracker{}
}

// RecordMissionCost calculates and records the mission cost.
func (t *Tracker) RecordMissionCost(ctx context.Context, tenantID, missionID, agentID, role string, costCents float64) error {
	// Emit the metric
	return telemetry.RecordMissionCostCents(ctx, tenantID, missionID, agentID, role, costCents)
}
