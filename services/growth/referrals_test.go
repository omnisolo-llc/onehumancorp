package growth

import (
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestReferralTracker_Advanced(t *testing.T) {
	tracker := NewReferralTracker()

	tracker.TrackReferral("user-1", analytics.SourceStandalone)
	tracker.TrackReferral("user-1", analytics.SourceStandalone)
	tracker.TrackReferral("user-2", analytics.SourceCloud)

	metrics := tracker.GetMetrics()
	if metrics.TotalReferrals != 3 {
		t.Errorf("Expected 3 referrals, got %d", metrics.TotalReferrals)
	}
	if metrics.UniqueInviters != 2 {
		t.Errorf("Expected 2 unique inviters, got %d", metrics.UniqueInviters)
	}
	if metrics.TotalConversions != 0 {
		t.Errorf("Expected 0 conversions, got %d", metrics.TotalConversions)
	}

	tracker.MarkConverted("user-1")
	tracker.MarkConverted("user-2")

	metrics = tracker.GetMetrics()
	if metrics.TotalConversions != 2 {
		t.Errorf("Expected 2 conversions, got %d", metrics.TotalConversions)
	}
	// K-factor = 2 conversions / 2 unique inviters = 1.0
	if metrics.KFactor != 1.0 {
		t.Errorf("Expected KFactor 1.0, got %f", metrics.KFactor)
	}
}
