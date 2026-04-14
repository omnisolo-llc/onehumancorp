package analytics

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("growth.analytics")

type Tracker struct {
	eventsTotal metric.Int64Counter
}

func NewTracker() (*Tracker, error) {
	eventsTotal, err := meter.Int64Counter(
		"analytics.events.total",
		metric.WithDescription("Total number of tracked events"),
	)
	if err != nil {
		return nil, err
	}

	return &Tracker{
		eventsTotal: eventsTotal,
	}, nil
}

func (t *Tracker) Track(ctx context.Context, name, userID string, metadata map[string]string) {
	opts := make([]attribute.KeyValue, 0, len(metadata)+2)
	opts = append(opts, attribute.String("event_name", name))
	opts = append(opts, attribute.String("user_id", userID))
	for k, v := range metadata {
		opts = append(opts, attribute.String("meta."+k, v))
	}
	t.eventsTotal.Add(ctx, 1, metric.WithAttributes(opts...))
}
