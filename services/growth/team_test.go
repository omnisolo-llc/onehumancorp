package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestTeamService_InviteToTeam(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewTeamService(tracker)
	ctx := context.Background()

	err := service.InviteToTeam(ctx, "team1", "user1", "user2@example.com")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = service.InviteToTeam(ctx, "", "user1", "user2@example.com")
	if err == nil {
		t.Fatalf("expected error for empty teamID")
	}
}

func TestTeamService_AcceptTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewTeamService(tracker)
	ctx := context.Background()

	err := service.AcceptTeamInvite(ctx, "invite1", "user2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = service.AcceptTeamInvite(ctx, "", "user2")
	if err == nil {
		t.Fatalf("expected error for empty inviteID")
	}
}
