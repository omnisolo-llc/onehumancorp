package growth

import (
	"context"
	"sync"
	"github.com/onehumancorp/mono/lib/analytics"
)

type ViralLoopTracker struct {
	mu              sync.RWMutex
	invitesSent     int
	invitesAccepted int
	analytics       *analytics.Tracker
}

func NewViralLoopTracker(tracker *analytics.Tracker) *ViralLoopTracker {
	return &ViralLoopTracker{
		analytics: tracker,
	}
}

func (v *ViralLoopTracker) RecordInviteSent(ctx context.Context, userID string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.invitesSent++
	if v.analytics != nil {
		v.analytics.TrackEvent(ctx, "invite_sent", map[string]interface{}{
			"user_id": userID,
		})
	}
}

func (v *ViralLoopTracker) RecordInviteAccepted(ctx context.Context, inviteeID string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.invitesAccepted++
	if v.analytics != nil {
		v.analytics.TrackEvent(ctx, "invite_accepted", map[string]interface{}{
			"invitee_id": inviteeID,
		})
	}
}

func (v *ViralLoopTracker) CalculateKFactor() float64 {
	v.mu.RLock()
	defer v.mu.RUnlock()
	if v.invitesSent == 0 {
		return 0.0
	}
	return float64(v.invitesAccepted) / float64(v.invitesSent)
}
