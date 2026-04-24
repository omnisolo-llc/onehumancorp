package billing

import "context"

// UsageRepository defines the persistence contract for LLM token usage
// events.  The in-memory Tracker satisfies this interface by default; a
// Postgres-backed implementation enables horizontal scaling.
type UsageRepository interface {
	// Track persists a single usage event.  The implementation is
	// responsible for computing CostUSD from the pricing catalog.
	Track(ctx context.Context, usage Usage) (Usage, error)
	// Summary returns the aggregate cost and token metrics for an
	// organization.
	Summary(ctx context.Context, organizationID string) (Summary, error)
}
