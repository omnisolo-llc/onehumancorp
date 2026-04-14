package growth

import (
	"fmt"
	"github.com/one-human-corp/ohc-ha/lib/analytics"
)

// GenerateInviteLink generates a team invite link for a given user.
func GenerateInviteLink(userID string) string {
	analytics.RecordEvent("invite_generated", userID)
	return fmt.Sprintf("https://ohc.os/invite/%s-team", userID)
}
