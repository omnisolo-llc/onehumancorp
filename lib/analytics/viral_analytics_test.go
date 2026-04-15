package analytics

import (
	"context"
	"os"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestViralAnalytics(t *testing.T) {
	ctx := context.Background()
	os.Setenv("DATABASE_URL", "sqlite://:memory:")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}

	_, err = database.Exec(ctx, "CREATE TABLE viral_conversions (source TEXT PRIMARY KEY, count INTEGER)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	va := NewViralAnalytics(database)

	err = va.RecordConversion(ctx, "twitter")
	if err != nil {
		t.Fatalf("failed to record conversion: %v", err)
	}

	count, err := va.GetConversions(ctx, "twitter")
	if err != nil {
		t.Fatalf("failed to get conversions: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1, got %d", count)
	}
}
