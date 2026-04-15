package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type TeamService struct {
	tracker *analytics.Tracker
}

func NewTeamService(tracker *analytics.Tracker) *TeamService {
	return &TeamService{
		tracker: tracker,
	}
}

func (s *TeamService) InviteToTeam(ctx context.Context, teamID string, inviterID string, inviteeEmail string) error {
	if teamID == "" || inviterID == "" || inviteeEmail == "" {
		return fmt.Errorf("invalid team invite parameters")
	}
	s.tracker.TrackEvent(ctx, "team_invite_sent", map[string]interface{}{
		"team_id":       teamID,
		"inviter_id":    inviterID,
		"invitee_email": inviteeEmail,
	})
	return nil
}

func (s *TeamService) AcceptTeamInvite(ctx context.Context, inviteID string, inviteeID string) error {
	if inviteID == "" || inviteeID == "" {
		return fmt.Errorf("invalid team invite acceptance parameters")
	}
	s.tracker.TrackEvent(ctx, "team_invite_accepted", map[string]interface{}{
		"invite_id": inviteID,
		"invitee_id": inviteeID,
	})
	return nil
}
