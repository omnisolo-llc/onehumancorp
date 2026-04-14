package hybridfsmcp

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type Escalator interface {
	Escalate(ctx context.Context, query string) bool
}

type DynamicEscalator struct {
	escalationCounter metric.Int64Counter
	threshold         int
}

func NewDynamicEscalator(threshold int) (*DynamicEscalator, error) {
	meter := otel.Meter("hybridfsmcp")
	counter, err := meter.Int64Counter("rag_escalation_count")
	if err != nil {
		return nil, err
	}
	return &DynamicEscalator{
		escalationCounter: counter,
		threshold:         threshold,
	}, nil
}

func (e *DynamicEscalator) Escalate(ctx context.Context, query string) bool {
	if len(query) > e.threshold {
		e.escalationCounter.Add(ctx, 1)
		return true
	}
	return false
}
