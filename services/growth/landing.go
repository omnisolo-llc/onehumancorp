package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type LandingService struct {
	tracker *analytics.Tracker
}

func NewLandingService(tracker *analytics.Tracker) *LandingService {
	return &LandingService{
		tracker: tracker,
	}
}

func (s *LandingService) TrackVisit(ctx context.Context, pageID string, visitorID string) error {
	if pageID == "" || visitorID == "" {
		return fmt.Errorf("invalid visit parameters")
	}
	s.tracker.TrackEvent(ctx, "landing_visit", map[string]interface{}{
		"page_id":    pageID,
		"visitor_id": visitorID,
	})
	return nil
}

func (s *LandingService) TrackConversion(ctx context.Context, pageID string, visitorID string) error {
	if pageID == "" || visitorID == "" {
		return fmt.Errorf("invalid conversion parameters")
	}
	s.tracker.TrackEvent(ctx, "landing_conversion", map[string]interface{}{
		"page_id":    pageID,
		"visitor_id": visitorID,
	})
	return nil
}
