package growth

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/lib/analytics"
)

// QuotaProvider is the interface that should be implemented
// by a Redis-backed store in Cloud-Native mode or a SQLite-backed
// store in Standalone mode to keep track of user invite quotas.
type QuotaProvider interface {
	// IncrementAndGet increments the usage count for the given userID
	// within the current time window (e.g., current month) and returns the new count.
	IncrementAndGet(ctx context.Context, userID string, window time.Time) (int, error)
}

type QuotaService struct {
	tracker  *analytics.Tracker
	limit    int
	provider QuotaProvider
}

func NewQuotaService(tracker *analytics.Tracker, limit int, provider QuotaProvider) *QuotaService {
	return &QuotaService{
		tracker:  tracker,
		limit:    limit,
		provider: provider,
	}
}

// CheckAndIncrement checks if the user has reached their quota.
// If they have, it returns an error and tracks the event.
// Otherwise, it increments their usage.
func (s *QuotaService) CheckAndIncrement(ctx context.Context, userID string) error {
	if userID == "" {
		return fmt.Errorf("invalid user ID")
	}

	// For simplicity, we use the start of the current month as the window
	now := time.Now().UTC()
	window := time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, now.Location())

	// Note: in a true billing-based system, we would check the limit before consuming a usage token.
	// For this growth referral logic, returning an error when incrementing beyond limit is acceptable.
	count, err := s.provider.IncrementAndGet(ctx, userID, window)
	if err != nil {
		return fmt.Errorf("failed to increment quota: %w", err)
	}

	if count > s.limit {
		s.tracker.TrackEvent(ctx, "quota_exceeded", map[string]interface{}{
			"user_id": userID,
		})
		return fmt.Errorf("quota exceeded")
	}

	return nil
}
