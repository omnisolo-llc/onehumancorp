package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// TestSQLSyncLag validates that hybrid sync models handle lag appropriately.
// Simulates a slow local disk or overloaded SQLite instance.
func TestSQLSyncLag(t *testing.T) {
	sip, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sip.Close()
	ctx := context.Background()

	err = sip.UpsertMission(ctx, "lag-mission", "PENDING", `{"test":"lag"}`, true)
	if err != nil {
		t.Fatalf("failed to seed mission: %v", err)
	}

	for i := 0; i < 10; i++ {
		t.Run(fmt.Sprintf("Iteration_%d", i), func(t *testing.T) {
			// A 0-millisecond timeout effectively causes the context to be canceled immediately
			shortCtx, cancel := context.WithTimeout(ctx, 0*time.Millisecond)
			defer cancel()

			_, err := sip.SyncMissions(shortCtx, "http://localhost:invalid")
			if err == nil {
				t.Errorf("Expected an error due to invalid URL or context timeout, got nil")
			}
		})
	}
}
