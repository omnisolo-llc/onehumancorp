package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type TeamInviteService struct {
	tracker *analytics.Tracker
	repo    *TeamInviteRepository
}

func NewTeamInviteService(tracker *analytics.Tracker, repo *TeamInviteRepository) *TeamInviteService {
	return &TeamInviteService{
		tracker: tracker,
		repo:    repo,
	}
}

func (s *TeamInviteService) ProcessTeamInvite(ctx context.Context, tenantID string, inviterID string, inviteeEmail string) error {
	if tenantID == "" || inviterID == "" || inviteeEmail == "" {
		return fmt.Errorf("invalid team invite parameters")
	}

	s.tracker.TrackEvent(ctx, "team_invite_sent", map[string]interface{}{
		"tenant_id":      tenantID,
		"inviter_id":     inviterID,
		"invitee_email":  inviteeEmail,
	})
	return nil
}

func (s *TeamInviteService) AcceptTeamInvite(ctx context.Context, inviteID string) error {
	if inviteID == "" {
		return fmt.Errorf("invalid team invite ID")
	}

	s.tracker.TrackEvent(ctx, "team_invite_accepted", map[string]interface{}{
		"invite_id": inviteID,
	})
	return nil
}
