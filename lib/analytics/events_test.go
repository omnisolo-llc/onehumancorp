package analytics

import (
	"context"
	"testing"
)

func TestTracker(t *testing.T) {
	tracker, err := NewTracker()
	if err != nil {
		t.Fatalf("failed to create tracker: %v", err)
	}

	tracker.Track(context.Background(), "signup", "user_123", map[string]string{"source": "referral"})
}
