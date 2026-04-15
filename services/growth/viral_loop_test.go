package growth

import (
	"context"
	"os"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestViralLoop(t *testing.T) {
	ctx := context.Background()
	os.Setenv("DATABASE_URL", "sqlite://:memory:")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}

	// Create tables manually for the test
	_, err = database.Exec(ctx, "CREATE TABLE viral_conversions (source TEXT PRIMARY KEY, count INTEGER)")
	if err != nil {
		t.Fatalf("failed to create viral_conversions table: %v", err)
	}

	_, err = database.Exec(ctx, "CREATE TABLE viral_invites (source TEXT PRIMARY KEY, count INTEGER)")
	if err != nil {
		t.Fatalf("failed to create viral_invites table: %v", err)
	}

	a := analytics.NewViralAnalytics(database)
	err = a.RecordConversion(ctx, "email")
	if err != nil {
		t.Fatalf("failed to record conversion: %v", err)
	}

	vl := NewViralLoop(database, a)
	err = vl.RecordInvite(ctx, "email")
	if err != nil {
		t.Fatalf("failed to record invite 1: %v", err)
	}
	err = vl.RecordInvite(ctx, "email")
	if err != nil {
		t.Fatalf("failed to record invite 2: %v", err)
	}

	kFactor, err := vl.CalculateKFactor(ctx, "email")
	if err != nil {
		t.Fatalf("failed to calculate k-factor: %v", err)
	}

	if kFactor != 0.5 {
		t.Errorf("Expected 0.5, got %f", kFactor)
	}
}
