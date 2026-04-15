package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type QuotaService struct {
	tracker *analytics.Tracker
	limits  map[string]int
}

func NewQuotaService(tracker *analytics.Tracker) *QuotaService {
	return &QuotaService{
		tracker: tracker,
		limits: map[string]int{
			"free": 100,
			"pro":  10000,
		},
	}
}

func (s *QuotaService) CheckQuota(ctx context.Context, tier string, usage int) error {
	limit, ok := s.limits[tier]
	if !ok {
		return fmt.Errorf("unknown tier")
	}

	if usage >= limit {
		s.tracker.TrackEvent(ctx, "quota_exceeded", map[string]interface{}{
			"tier":  tier,
			"usage": usage,
			"limit": limit,
		})
		return fmt.Errorf("quota exceeded")
	}

	s.tracker.TrackEvent(ctx, "quota_check_passed", map[string]interface{}{
		"tier":  tier,
		"usage": usage,
	})
	return nil
}
