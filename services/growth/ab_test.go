package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type ABTestService struct {
	tracker *analytics.Tracker
}

func NewABTestService(tracker *analytics.Tracker) *ABTestService {
	return &ABTestService{
		tracker: tracker,
	}
}

func (s *ABTestService) RecordImpression(ctx context.Context, experimentID string, variant string) error {
	if experimentID == "" || variant == "" {
		return fmt.Errorf("invalid experiment parameters")
	}
	s.tracker.TrackEvent(ctx, "ab_test_impression", map[string]interface{}{
		"experiment_id": experimentID,
		"variant":       variant,
	})
	return nil
}

func (s *ABTestService) RecordConversion(ctx context.Context, experimentID string, variant string) error {
	if experimentID == "" || variant == "" {
		return fmt.Errorf("invalid experiment parameters")
	}
	s.tracker.TrackEvent(ctx, "ab_test_conversion", map[string]interface{}{
		"experiment_id": experimentID,
		"variant":       variant,
	})
	return nil
}
