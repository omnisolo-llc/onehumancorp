package analytics

import (
	"testing"
)

func TestViralLoopTracker(t *testing.T) {
	tracker := NewViralLoopTracker()

	if kFactor := tracker.GetKFactor(); kFactor != 0.0 {
		t.Fatalf("expected initial K-Factor to be 0.0, got %f", kFactor)
	}

	tracker.RecordReferral("user1")
	tracker.RecordReferral("user2")

	tracker.RecordConversion("user1")
	tracker.RecordConversion("user1")
	tracker.RecordConversion("user2")

	kFactor := tracker.GetKFactor()
	if kFactor != 1.5 {
		t.Fatalf("expected K-Factor to be 1.5, got %f", kFactor)
	}
}
