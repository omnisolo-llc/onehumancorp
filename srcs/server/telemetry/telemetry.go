package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	agentCostEstimateUSD metric.Float64Counter
	missionCostCents     metric.Float64Counter
)

func init() {
	var err error
	agentCostEstimateUSD, err = meter.Float64Counter(
		"ohc_agent_cost_estimate_usd",
		metric.WithDescription("Estimated agent cost in USD"),
	)
	if err != nil {
		log.Printf("Failed to create agentCostEstimateUSD counter: %v", err)
	}

	missionCostCents, err = meter.Float64Counter(
		"ohc_mission_cost_cents",
		metric.WithDescription("Mission cost in cents"),
	)
	if err != nil {
		log.Printf("Failed to create missionCostCents counter: %v", err)
	}
}

// RecordAgentCostEstimateUSD records the estimated cost in USD.
func RecordAgentCostEstimateUSD(ctx context.Context, tenantID, agentID, role string, costUSD float64) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if agentCostEstimateUSD != nil {
		opts := metric.WithAttributes(
			attribute.String("tenant_id", tenantID),
			attribute.String("agent_id", agentID),
			attribute.String("role", role),
			getDeploymentModeAttribute(),
		)
		agentCostEstimateUSD.Add(ctx, costUSD, opts)
	}
	return nil
}

// RecordMissionCostCents records the mission cost in cents.
func RecordMissionCostCents(ctx context.Context, tenantID, missionID, agentID, role string, costCents float64) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if missionCostCents != nil {
		opts := metric.WithAttributes(
			attribute.String("tenant_id", tenantID),
			attribute.String("mission_id", missionID),
			attribute.String("agent_id", agentID),
			attribute.String("role", role),
			getDeploymentModeAttribute(),
		)
		missionCostCents.Add(ctx, costCents, opts)
	}
	return nil
}
