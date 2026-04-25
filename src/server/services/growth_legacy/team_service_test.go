package growth

import (
	"context"
	"github.com/onehumancorp/mono/src/server/lib/analytics"
	"testing"
)

func TestSendTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	repo := NewReferralRepository(nil)
	service := NewTeamService(tracker, repo)

	_, err := service.SendTeamInvite(context.Background(), "team-1", "user-1", "test@example.com")
	if err != nil {
		t.Errorf("SendTeamInvite failed: %v", err)
	}

	_, err = service.SendTeamInvite(context.Background(), "", "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}

func TestAcceptTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	repo := NewReferralRepository(nil)
	service := NewTeamService(tracker, repo)

	// First send an invite to get a valid referral ID
	ref, err := service.SendTeamInvite(context.Background(), "team-1", "user-1", "test@example.com")
	if err != nil {
		t.Fatalf("Failed to setup referral: %v", err)
	}

	err = service.AcceptTeamInvite(context.Background(), ref.ID, "spiffe://example.org/newuser")
	if err != nil {
		t.Errorf("AcceptTeamInvite failed: %v", err)
	}

	err = service.AcceptTeamInvite(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty invite ID")
	}
}
