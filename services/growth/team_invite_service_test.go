package growth

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/lib/analytics"
)

func TestTeamInviteService_ProcessTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	repo := NewTeamInviteRepository(nil)
	svc := NewTeamInviteService(tracker, repo)
	ctx := context.Background()

	err := svc.ProcessTeamInvite(ctx, "t-1", "u-1", "test@example.com")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	err = svc.ProcessTeamInvite(ctx, "", "u-1", "test@example.com")
	if err == nil {
		t.Fatal("Expected error for missing tenant ID")
	}
}

func TestTeamInviteService_AcceptTeamInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	repo := NewTeamInviteRepository(nil)
	svc := NewTeamInviteService(tracker, repo)
	ctx := context.Background()

	err := svc.AcceptTeamInvite(ctx, "inv-1")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	err = svc.AcceptTeamInvite(ctx, "")
	if err == nil {
		t.Fatal("Expected error for missing invite ID")
	}
}
