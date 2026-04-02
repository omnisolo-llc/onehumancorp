package orchestration

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestSIPDB_DelegateMission_Throttling(t *testing.T) {
	// Enable standalone mode
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Fill the throttle semaphore
	throttleSemaphore <- struct{}{}
	throttleSemaphore <- struct{}{}

	db := &SIPDB{}

	errCh := make(chan error, 1)
	go func() {
		errCh <- db.DelegateMission(ctx, "mission-id", "role", Message{Content: "test"})
	}()

	select {
	case <-time.After(100 * time.Millisecond):
		// Expected, it should block because the semaphore is full
	case <-errCh:
		t.Fatalf("Expected DelegateMission to block due to throttling")
	}

	// Release one slot
	<-throttleSemaphore

	select {
	case err := <-errCh:
		if err != nil {
			t.Fatalf("DelegateMission failed: %v", err)
		}
	case <-time.After(500 * time.Millisecond):
		t.Fatalf("Expected DelegateMission to succeed after releasing semaphore")
	}

	// Ensure the slots are free for other tests
	select {
	case <-throttleSemaphore:
	default:
	}
}
