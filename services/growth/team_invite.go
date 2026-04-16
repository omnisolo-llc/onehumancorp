package growth

import (
	"context"
	"fmt"
	"strings"

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

func (s *TeamInviteService) ProcessBulkInvites(ctx context.Context, senderID string, emails string) error {
	if senderID == "" || emails == "" {
		return fmt.Errorf("invalid bulk invite parameters")
	}

	emailList := strings.Split(emails, ",")
	validEmails := make([]string, 0)

	for _, e := range emailList {
		e = strings.TrimSpace(e)
		if e != "" && strings.Contains(e, "@") {
			validEmails = append(validEmails, e)
		}
	}

	if len(validEmails) == 0 {
		return fmt.Errorf("no valid emails provided")
	}

	s.tracker.TrackEvent(ctx, "team_invites_sent", map[string]interface{}{
		"sender_id":  senderID,
		"count":      len(validEmails),
		"emails":     validEmails,
	})
	return nil
}
