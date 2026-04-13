package growth

import "testing"

func TestReferralTracker(t *testing.T) {
	tracker := NewReferralTracker()
	if tracker.GetTotalReferrals() != 0 {
		t.Errorf("Expected 0 referrals initially, got %d", tracker.GetTotalReferrals())
	}
	tracker.AddReferral()
	if tracker.GetTotalReferrals() != 1 {
		t.Errorf("Expected 1 referral after add, got %d", tracker.GetTotalReferrals())
	}
}

func TestTrackExperimentReferral(t *testing.T) {
	tracker := NewReferralTracker()
	if tracker.GetTotalReferrals() != 0 {
		t.Errorf("Expected 0 referrals initially, got %d", tracker.GetTotalReferrals())
	}
	tracker.TrackExperimentReferral("test_exp", "treatment")
	if tracker.GetTotalReferrals() != 1 {
		t.Errorf("Expected 1 referral after track experiment, got %d", tracker.GetTotalReferrals())
	}
}
