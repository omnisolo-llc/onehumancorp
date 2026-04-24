package local

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

// CostTrackerInterceptor wraps an LLMClient to record real-time token and cost tracking.
type CostTrackerInterceptor struct {
	target LLMClient
	agentID string
	orgID string
	role string
	model string
}

// NewCostTrackerInterceptor creates a new CostTrackerInterceptor.
func NewCostTrackerInterceptor(target LLMClient, agentID, orgID, role, model string) *CostTrackerInterceptor {
	return &CostTrackerInterceptor{
		target: target,
		agentID: agentID,
		orgID: orgID,
		role: role,
		model: model,
	}
}

// Complete delegates to the underlying LLMClient and intercepts the response to track usage and cost.
func (i *CostTrackerInterceptor) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	tracer := otel.Tracer("ohc.agent.llm")
	ctx, span := tracer.Start(ctx, "LLMClient.Complete")
	defer span.End()

	span.SetAttributes(
		attribute.String("agent_id", i.agentID),
		attribute.String("model", i.model),
	)

	start := time.Now()
	resp, err := i.target.Complete(ctx, req)
	duration := time.Since(start).Seconds()

	span.SetAttributes(attribute.Float64("duration_s", duration))

	if err != nil {
		span.RecordError(err)
		return resp, err
	}

	if resp != nil {
		totalTokens := resp.InputTokens + resp.OutputTokens

		span.SetAttributes(
			attribute.Int64("input_tokens", resp.InputTokens),
			attribute.Int64("output_tokens", resp.OutputTokens),
			attribute.Int64("total_tokens", totalTokens),
		)

		telemetry.RecordAgentTokenUsage(ctx, i.agentID, i.orgID, i.role, i.model, totalTokens)

		if price, ok := billing.DefaultCatalog[i.model]; ok {
			cost := (float64(resp.InputTokens) / 1_000_000.0) * price.InputPerMillionUSD +
				(float64(resp.OutputTokens) / 1_000_000.0) * price.OutputPerMillionUSD
			span.SetAttributes(attribute.Float64("estimated_cost_usd", cost))
			telemetry.RecordAgentCost(ctx, i.agentID, i.orgID, i.role, i.model, cost)
		}
	}

	return resp, nil
}

// ToolCostTrackerInterceptor wraps a Tool to record executions via OpenTelemetry spans.
type ToolCostTrackerInterceptor struct {
	target Tool
	agentID string
}

// NewToolCostTrackerInterceptor creates a new ToolCostTrackerInterceptor.
func NewToolCostTrackerInterceptor(target Tool, agentID string) *ToolCostTrackerInterceptor {
	return &ToolCostTrackerInterceptor{
		target: target,
		agentID: agentID,
	}
}

// Definition delegates to the underlying tool.
func (i *ToolCostTrackerInterceptor) Definition() ToolDefinition {
	return i.target.Definition()
}

// Execute delegates to the underlying tool and tracks the execution.
func (i *ToolCostTrackerInterceptor) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	tracer := otel.Tracer("ohc.agent.tools")
	ctx, span := tracer.Start(ctx, "Tool.Execute")
	defer span.End()

	toolName := i.Definition().Name
	span.SetAttributes(
		attribute.String("agent_id", i.agentID),
		attribute.String("tool_name", toolName),
	)

	start := time.Now()
	res, err := i.target.Execute(ctx, workDir, input)
	duration := time.Since(start).Seconds()

	// In the future, specific tool cost logic could be applied here.
	// For now we just emit the trace with agent_id and tool_name as requested.
	span.SetAttributes(attribute.Float64("duration_s", duration))

	if err != nil {
		span.RecordError(err)
	}

	return res, err
}
