package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

func TestSIPDB_DelegateMission_Throttling(t *testing.T) {
	// Enable standalone mode which triggers the throttle
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	dbPath := filepath.Join(t.TempDir(), "test_throttle.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer db.Close()

	msg := Message{ID: "msg1", Content: "test content", Type: "task"}

	var wg sync.WaitGroup
	startChan := make(chan struct{})
	numConcurrent := 10

	errChan := make(chan error, numConcurrent)

	for i := 0; i < numConcurrent; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-startChan // wait for sync
			ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
			defer cancel()
			// Use unique mission ID for each call to avoid UNIQUE constraint violation on id
			errChan <- db.DelegateMission(ctx, "mission-id-"+string(rune('0'+idx)), "role", msg)
		}(i)
	}

	// Fire them all at once
	close(startChan)
	wg.Wait()
	close(errChan)

	for err := range errChan {
		if err != nil {
			t.Errorf("DelegateMission failed under concurrent load: %v", err)
		}
	}
}

func TestTokenBurnRateTracker_CalculateBurnRate(t *testing.T) {
	tracker := NewTokenBurnRateTracker()
	ctx := context.Background()

	// Initial empty history - no panic
	tracker.CalculateBurnRate(ctx)

	// Single point - shouldn't trigger calculation (needs at least 2 for rate)
	tracker.RecordUsage("org1", 100)
	tracker.CalculateBurnRate(ctx)

	// Two points - should calculate
	tracker.RecordUsage("org1", 200)
	tracker.CalculateBurnRate(ctx)

	// Ensure history is truncated to 5 items
	for i := 0; i < 10; i++ {
		tracker.RecordUsage("org2", int64(100+i*10))
	}

	tracker.mu.Lock()
	if len(tracker.history["org2"]) > 5 {
		t.Errorf("history should not exceed 5 items, got %d", len(tracker.history["org2"]))
	}
	tracker.mu.Unlock()

	tracker.CalculateBurnRate(ctx)
}
