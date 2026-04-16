package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type TeamInviteService struct {
	tracker *analytics.Tracker
}

func NewTeamInviteService(tracker *analytics.Tracker) *TeamInviteService {
	return &TeamInviteService{
		tracker: tracker,
	}
}

func (s *TeamInviteService) SendTeamInvite(ctx context.Context, tenantID string, senderID string, receiverEmail string) error {
	if tenantID == "" || senderID == "" || receiverEmail == "" {
		return fmt.Errorf("invalid team invite parameters")
	}
	s.tracker.TrackEvent(ctx, "team_invite_sent", map[string]interface{}{
		"tenant_id":      tenantID,
		"sender_id":      senderID,
		"receiver_email": receiverEmail,
	})
	return nil
}

func (s *TeamInviteService) AcceptTeamInvite(ctx context.Context, tenantID string, inviteID string) error {
	if tenantID == "" || inviteID == "" {
		return fmt.Errorf("invalid team invite ID")
	}
	s.tracker.TrackEvent(ctx, "team_invite_accepted", map[string]interface{}{
		"tenant_id": tenantID,
		"invite_id": inviteID,
	})
	return nil
}
