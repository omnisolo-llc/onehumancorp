package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestSendTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewTeamInviteService(tracker)

	err := service.SendTeamInvite(context.Background(), "tenant-1", "user-123", "test@example.com")
	if err != nil {
		t.Errorf("SendTeamInvite failed: %v", err)
	}

	err = service.SendTeamInvite(context.Background(), "", "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}

func TestAcceptTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewTeamInviteService(tracker)

	err := service.AcceptTeamInvite(context.Background(), "tenant-1", "invite-123")
	if err != nil {
		t.Errorf("AcceptTeamInvite failed: %v", err)
	}

	err = service.AcceptTeamInvite(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}
