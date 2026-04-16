package growth

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestInviteTracker(t *testing.T) {
	ctx := context.Background()

	os.Setenv("DATABASE_URL", "sqlite://:memory:")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}

	// Run migrations
	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tracker := NewInviteTracker(database)

	count, err := tracker.GetTotalInvitesCount(ctx)
	if err != nil {
		t.Fatalf("failed to get total invites count: %v", err)
	}
	if count != 0 {
		t.Fatalf("Expected 0 invites initially, got %d", count)
	}

	teamID := "team123"
	inviterID := "user1"
	inviteeID := "user2"

	err = tracker.RecordInvite(ctx, teamID, inviterID, inviteeID)
	if err != nil {
		t.Fatalf("failed to record invite: %v", err)
	}

	count, err = tracker.GetTotalInvitesCount(ctx)
	if err != nil {
		t.Fatalf("failed to get total invites count: %v", err)
	}
	if count != 1 {
		t.Fatalf("Expected 1 total invite after record, got %d", count)
	}

	count, err = tracker.GetTeamInvitesCount(ctx, teamID)
	if err != nil {
		t.Fatalf("failed to get team invites count: %v", err)
	}
	if count != 1 {
		t.Fatalf("Expected 1 invite for team, got %d", count)
	}
}
