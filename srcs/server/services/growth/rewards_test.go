package growth

import (
	"context"
	"os"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRewardsManager(t *testing.T) {
	ctx := context.Background()
	os.Setenv("DATABASE_URL", "sqlite://:memory:")
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}
	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tracker := NewInviteTracker(database)
	rewards := NewRewardsManager(tracker)

	tracker.RecordInvite(ctx, "team-1", "user-1", "user-2")

	reward, err := rewards.CheckAndGrantReward(ctx, "team-1")
	if err != nil {
		t.Fatalf("expected reward, got error: %v", err)
	}
	if reward != "active_team_badge" {
		t.Fatalf("expected active_team_badge, got %s", reward)
	}
}
