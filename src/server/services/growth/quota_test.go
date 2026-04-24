package growth

import "testing"

func TestQuotaTracker(t *testing.T) {
	tracker := NewQuotaTracker(100, 50)

	if q := tracker.CalculateQuota(0); q != 100 {
		t.Errorf("Expected 100, got %d", q)
	}

	if q := tracker.CalculateQuota(2); q != 200 {
		t.Errorf("Expected 200, got %d", q)
	}
}

func TestQuotaTracker_CheckLimit(t *testing.T) {
	tracker := NewQuotaTracker(100, 50)

	// User has used 50, quota is 100 (0 referrals). Under limit.
	if !tracker.CheckLimit(50, 0) {
		t.Errorf("Expected true, got false")
	}

	// User has used 150, quota is 100 (0 referrals). Over limit.
	if tracker.CheckLimit(150, 0) {
		t.Errorf("Expected false, got true")
	}

	// User has used 150, quota is 200 (2 referrals). Under limit.
	if !tracker.CheckLimit(150, 2) {
		t.Errorf("Expected true, got false")
	}
}
