package orchestration

import (
	"context"
	"log/slog"
	"time"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"

	"github.com/onehumancorp/mono/srcs/server/telemetry"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

// StartTokenBurnForecaster initiates a background worker to extrapolate LLM token usage
// and forecast burn rates. This is a critical component for Cloud-Native mode observability,
// providing predictive alerts for per-tenant LLM token budgets.
func StartTokenBurnForecaster(ctx context.Context, getActiveOrgs func(context.Context) []string, getTokens func(string) int64) {
	// The default tick interval is 1 minute for moving average calculation.
	StartTokenBurnForecasterWithTicker(ctx, getActiveOrgs, getTokens, 1*time.Minute)
}

// StartTokenBurnForecasterWithTicker is the core implementation of the forecasting engine
// that supports a configurable tick duration for easier unit testing.
func StartTokenBurnForecasterWithTicker(ctx context.Context, getActiveOrgs func(context.Context) []string, getTokens func(string) int64, tickDuration time.Duration) {
	forecasterTicker := time.NewTicker(tickDuration)
	defer forecasterTicker.Stop()

	// history tracks the accumulated token counts per organization over recent ticks
	// to enable moving average calculations.
	history := make(map[string][]int64)

	for {
		select {
		case <-ctx.Done():
			return
		case <-forecasterTicker.C:
			ProcessForecastTick(ctx, history, getActiveOrgs, getTokens)
		}
	}
}

// ProcessForecastTick evaluates the token burn rate for all active organizations in a single tick.
// It maintains a moving average and emits predictive cost alerts when burn rates are detected.
func ProcessForecastTick(ctx context.Context, history map[string][]int64, getActiveOrgs func(context.Context) []string, getTokens func(string) int64) {
	if getActiveOrgs == nil || getTokens == nil {
		return
	}

	activeOrganizations := getActiveOrgs(ctx)
	currentActiveMap := make(map[string]bool)

	for _, orgID := range activeOrganizations {
		currentActiveMap[orgID] = true
		tokensUsed := getTokens(orgID)

		if tokensUsed <= 0 {
			delete(history, orgID)
			continue
		}

		orgHistory := history[orgID]
		orgHistory = append(orgHistory, tokensUsed)

		// Retain only the most recent 5 data points for the moving average calculation
		if len(orgHistory) > 5 {
			orgHistory = orgHistory[1:]
		}
		history[orgID] = orgHistory

		if len(orgHistory) > 1 {
			// Compute the moving average burn rate (tokens per interval)
			burnRate := float64(orgHistory[len(orgHistory)-1]-orgHistory[0]) / float64(len(orgHistory)-1)
			telemetry.RecordTokenBurnRate(ctx, orgID, burnRate)

			// Assuming a 1-minute interval, project the burn rate to a 24-hour forecast
			projected24hUsage := burnRate * 60 * 24
			telemetry.RecordTokenBurnRatePredicted24h(ctx, orgID, projected24hUsage)

			// Trigger predictive cost alerts based on the extrapolated 24h usage
			if projected24hUsage > 0 {
				slog.WarnContext(ctx, "predictive cost alert emitted", "organization_id", orgID, "prediction_24h", projected24hUsage)
				if telemetry.TokenBudgetAlertTotal != nil {
					telemetry.TokenBudgetAlertTotal.Add(ctx, 1, metric.WithAttributes(
						attribute.String("organization_id", orgID),
					))
				}
			}
		}
	}

	// Clean up history for organizations that are no longer active
	for orgID := range history {
		if !currentActiveMap[orgID] {
			delete(history, orgID)
		}
	}
}

type MeshTransport interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
	BroadcastCoordination(ctx context.Context, msg MeshMessage) error
	SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)
	AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error
	SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error)
	BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error
	SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error)
	PublishTeammateMeshEvent(ctx context.Context, channel string, agentID, action, status string, payload []byte) error
	SubscribeTeammateMesh(ctx context.Context, channel string) (<-chan []byte, error)
	Publish(topic string, data []byte) error
	Subscribe(topic string) (<-chan []byte, error)
}
