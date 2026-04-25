package hybridfsmcp

import (
	"context"
)

// Escalator defines an interface for analyzing if a query should be escalated.
type Escalator interface {
	// Analyze determines if the given query should be escalated to the Cloud.
	// Returns true if escalation is required.
	Analyze(ctx context.Context, query string) bool
}

// DefaultEscalator is the default implementation of the Escalator interface.
type DefaultEscalator struct {
	MaxLocalQueryLength int
}

// NewDefaultEscalator creates a new DefaultEscalator.
func NewDefaultEscalator(maxLocalQueryLength int) *DefaultEscalator {
	if maxLocalQueryLength <= 0 {
		maxLocalQueryLength = 100 // default
	}
	return &DefaultEscalator{
		MaxLocalQueryLength: maxLocalQueryLength,
	}
}

// Analyze evaluates the complexity of the query based on length.
func (e *DefaultEscalator) Analyze(ctx context.Context, query string) bool {
	// Simple heuristic: if the query is very long, it likely requires complex RAG
	// processing or massive corpus search better suited for the pgvector Swarm.
	return len(query) > e.MaxLocalQueryLength
}
