package growth

import (
	"context"
	"math/rand"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	assignmentCounter metric.Int64Counter
	conversionCounter metric.Int64Counter
)

func init() {
	// Seed global rand for older go versions
	rand.Seed(time.Now().UnixNano())
	meter := otel.Meter("github.com/onehumancorp/mono/ohc/growth")
	assignmentCounter, _ = meter.Int64Counter("growth_ab_assignments_total")
	conversionCounter, _ = meter.Int64Counter("growth_ab_conversions_total")
}

// AssignVariant assigns a user to a variant based on provided weights.
// Weights should add up to 100 for percentage-based assignments.
func AssignVariant(ctx context.Context, experiment string, variants []string, weights []int) string {
	if len(variants) == 0 || len(variants) != len(weights) {
		return "control" // Fallback
	}

	totalWeight := 0
	for _, w := range weights {
		totalWeight += w
	}

	if totalWeight == 0 {
		return variants[0]
	}

	r := rand.Intn(totalWeight)
	current := 0
	assigned := variants[0]

	for i, w := range weights {
		current += w
		if r < current {
			assigned = variants[i]
			break
		}
	}

	if assignmentCounter != nil {
		assignmentCounter.Add(ctx, 1, metric.WithAttributes(
			attribute.String("experiment", experiment),
			attribute.String("variant", assigned),
		))
	}

	return assigned
}

// TrackConversion tracks a successful conversion for an experiment variant.
func TrackConversion(ctx context.Context, experiment string, variant string) {
	if conversionCounter != nil {
		conversionCounter.Add(ctx, 1, metric.WithAttributes(
			attribute.String("experiment", experiment),
			attribute.String("variant", variant),
		))
	}
}
