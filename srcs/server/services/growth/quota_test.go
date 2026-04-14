package growth

import (
    "context"
    "os"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestQuotaTracker(t *testing.T) {
    ctx := context.Background()
    os.Setenv("DATABASE_URL", "sqlite://:memory:")
    database, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to connect to memory db: %v", err)
    }
    if err := database.RunMigrations(ctx); err != nil {
        t.Fatalf("failed to run migrations: %v", err)
    }

    tracker := NewQuotaTracker(database)

    used, max, err := tracker.GetQuota(ctx, "org1", "compute")
    if err != nil {
        t.Fatalf("failed to get default quota: %v", err)
    }
    if used != 0 || max != 100 {
        t.Fatalf("expected 0/100, got %d/%d", used, max)
    }

    err = tracker.IncrementQuota(ctx, "org1", "compute", 10)
    if err != nil {
        t.Fatalf("failed to increment quota: %v", err)
    }

    used, max, err = tracker.GetQuota(ctx, "org1", "compute")
    if err != nil {
        t.Fatalf("failed to get quota: %v", err)
    }
    if used != 10 || max != 100 {
        t.Fatalf("expected 10/100, got %d/%d", used, max)
    }
}
