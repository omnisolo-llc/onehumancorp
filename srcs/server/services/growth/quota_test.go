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
