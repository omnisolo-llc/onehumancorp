package telemetry

import (
	"context"
	"encoding/json"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

func recordInt64Count(ctx context.Context, counter metric.Int64Counter, metricName string, amount int64, attrs []attribute.KeyValue) {
	if counter != nil {
		counter.Add(ctx, amount, metric.WithAttributes(attrs...))
	}

	if BufferMetricFunc != nil {
		payloadMap := make(map[string]interface{})
		for _, attr := range attrs {
			payloadMap[string(attr.Key)] = attr.Value.AsInterface()
		}
		payloadMap["count"] = amount

		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, metricName, string(payloadBytes))
	}
}

func recordFloat64Count(ctx context.Context, counter metric.Float64Counter, metricName string, amount float64, attrs []attribute.KeyValue) {
	if counter != nil {
		counter.Add(ctx, amount, metric.WithAttributes(attrs...))
	}

	if BufferMetricFunc != nil {
		payloadMap := make(map[string]interface{})
		for _, attr := range attrs {
			payloadMap[string(attr.Key)] = attr.Value.AsInterface()
		}
		payloadMap["count"] = amount

		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, metricName, string(payloadBytes))
	}
}

func recordFloat64Histogram(ctx context.Context, histogram metric.Float64Histogram, metricName string, value float64, attrs []attribute.KeyValue) {
	if histogram != nil {
		histogram.Record(ctx, value, metric.WithAttributes(attrs...))
	}

	if BufferMetricFunc != nil {
		payloadMap := make(map[string]interface{})
		for _, attr := range attrs {
			payloadMap[string(attr.Key)] = attr.Value.AsInterface()
		}
		payloadMap["value"] = value

		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, metricName, string(payloadBytes))
	}
}

func recordInt64Histogram(ctx context.Context, histogram metric.Int64Histogram, metricName string, value int64, attrs []attribute.KeyValue) {
	if histogram != nil {
		histogram.Record(ctx, value, metric.WithAttributes(attrs...))
	}

	if BufferMetricFunc != nil {
		payloadMap := make(map[string]interface{})
		for _, attr := range attrs {
			payloadMap[string(attr.Key)] = attr.Value.AsInterface()
		}
		payloadMap["value"] = value

		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, metricName, string(payloadBytes))
	}
}

// Support for Mode override

func RecordTokenUsageWithMode(ctx context.Context, agentID, role, model, tokenType string, count int64, mode string) {
	attrs := []attribute.KeyValue{
		attribute.String("agent_id", agentID),
		attribute.String("role", role),
		attribute.String("model", model),
		attribute.String("type", tokenType),
		attribute.String("deployment_mode", mode),
	}
	if tokenUsageCounter != nil {
		tokenUsageCounter.Add(ctx, count, metric.WithAttributes(attrs...))
	}
}

func RecordAgentTokenUsageWithMode(ctx context.Context, agentID, organizationID, role, model string, count int64, mode string) {
	attrs := []attribute.KeyValue{
		attribute.String("agent_id", agentID),
		attribute.String("organization_id", organizationID),
		attribute.String("role", role),
		attribute.String("model", model),
		attribute.String("deployment_mode", mode),
	}
	if AgentTokenUsageTotal != nil {
		AgentTokenUsageTotal.Add(ctx, count, metric.WithAttributes(attrs...))
	}
}

func RecordAgentCostWithMode(ctx context.Context, agentID, organizationID, role, model string, cost float64, mode string) {
	attrs := []attribute.KeyValue{
		attribute.String("agent_id", agentID),
		attribute.String("organization_id", organizationID),
		attribute.String("role", role),
		attribute.String("model", model),
		attribute.String("deployment_mode", mode),
	}
	if AgentCostEstimateUSD != nil {
		AgentCostEstimateUSD.Add(ctx, cost, metric.WithAttributes(attrs...))
	}
}

func RecordSwarmTaskCompletedWithMode(ctx context.Context, missionID string, mode string) {
	attrs := []attribute.KeyValue{
		attribute.String("mission_id", missionID),
		attribute.String("deployment_mode", mode),
	}
	if swarmTasksCompletedCounter != nil {
		swarmTasksCompletedCounter.Add(ctx, 1, metric.WithAttributes(attrs...))
	}
}

func RecordAgentApiCallWithMode(ctx context.Context, agentID, role, api string, mode string) {
	attrs := []attribute.KeyValue{
		attribute.String("agent_id", agentID),
		attribute.String("role", role),
		attribute.String("api", api),
		attribute.String("deployment_mode", mode),
	}
	if agentApiCallsCounter != nil {
		agentApiCallsCounter.Add(ctx, 1, metric.WithAttributes(attrs...))
	}
}
