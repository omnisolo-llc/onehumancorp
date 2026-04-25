package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/src/server/lib/analytics"
	"time"
)

type TeamService struct {
	tracker *analytics.Tracker
	repo    *ReferralRepository
}

func NewTeamService(tracker *analytics.Tracker, repo *ReferralRepository) *TeamService {
	return &TeamService{
		tracker: tracker,
		repo:    repo,
	}
}

func (s *TeamService) SendTeamInvite(ctx context.Context, teamID string, inviterID string, inviteeEmail string) (*GrowthReferral, error) {
	if teamID == "" || inviterID == "" || inviteeEmail == "" {
		return nil, fmt.Errorf("invalid team invite parameters")
	}

	referral := &GrowthReferral{
		ID:           fmt.Sprintf("team-ref-%d", time.Now().UnixNano()),
		InviterID:    inviterID,
		InviteeEmail: inviteeEmail,
		Status:       "PENDING",
		CreatedAt:    time.Now(),
	}

	if s.repo != nil {
		err := s.repo.SaveReferral(ctx, referral)
		if err != nil {
			return nil, err
		}
	}

	s.tracker.TrackEvent(ctx, "team_invite_sent", map[string]interface{}{
		"team_id":       teamID,
		"inviter_id":    inviterID,
		"invitee_email": inviteeEmail,
		"referral_id":   referral.ID,
	})
	return referral, nil
}

func (s *TeamService) AcceptTeamInvite(ctx context.Context, inviteID string, spiffeID string) error {
	if inviteID == "" {
		return fmt.Errorf("invalid team invite ID")
	}

	if s.repo != nil {
		ref, err := s.repo.GetReferralByID(ctx, inviteID)
		if err != nil {
			return fmt.Errorf("referral not found")
		}

		if ref.Status == "SIGNED_UP" {
			return nil
		}

		ref.Status = "SIGNED_UP"
		err = s.repo.SaveReferral(ctx, ref)
		if err != nil {
			return err
		}
	}

	s.tracker.TrackEvent(ctx, "team_invite_accepted", map[string]interface{}{
		"invite_id": inviteID,
		"spiffe_id": spiffeID,
	})
	return nil
}
