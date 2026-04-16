package growth

import (
	"context"
	"github.com/onehumancorp/mono/lib/analytics"
	"testing"
)

func TestViralLoopTracker(t *testing.T) {
	ctx := context.Background()
	analyticsTracker := analytics.NewTracker()
	tracker := NewViralLoopTracker(analyticsTracker)

	tracker.RecordInviteSent(ctx, "user1")
	tracker.RecordInviteSent(ctx, "user2")
	tracker.RecordInviteAccepted(ctx, "invitee1")

	kFactor := tracker.CalculateKFactor()
	if kFactor != 0.5 {
		t.Fatalf("Expected K-Factor 0.5, got %f", kFactor)
	}
}
