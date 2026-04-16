package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
)

type ReferralService struct {
	tracker *analytics.Tracker
}

func NewReferralService(tracker *analytics.Tracker) *ReferralService {
	return &ReferralService{
		tracker: tracker,
	}
}

func (s *ReferralService) ProcessInvite(ctx context.Context, senderID string, receiverEmail string) error {
	if senderID == "" || receiverEmail == "" {
		return fmt.Errorf("invalid invite parameters")
	}
	s.tracker.TrackEvent(ctx, "invite_sent", map[string]interface{}{
		"sender_id":      senderID,
		"receiver_email": receiverEmail,
	})
	return nil
}

func (s *ReferralService) AcceptInvite(ctx context.Context, inviteID string) error {
	if inviteID == "" {
		return fmt.Errorf("invalid invite ID")
	}
	s.tracker.TrackEvent(ctx, "invite_accepted", map[string]interface{}{
		"invite_id": inviteID,
	})
	return nil
}

func (s *ReferralService) ProcessBulkInvites(ctx context.Context, senderID string, receiverEmails []string) error {
	if senderID == "" || len(receiverEmails) == 0 {
		return fmt.Errorf("invalid bulk invite parameters")
	}

	for _, email := range receiverEmails {
		if err := s.ProcessInvite(ctx, senderID, email); err != nil {
			return fmt.Errorf("failed to process invite for %s: %w", email, err)
		}
	}

	s.tracker.TrackEvent(ctx, "bulk_invite_sent", map[string]interface{}{
		"sender_id": senderID,
		"count":     len(receiverEmails),
	})

	return nil
}
