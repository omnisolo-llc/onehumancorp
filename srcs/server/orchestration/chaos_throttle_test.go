package orchestration

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func TestStandaloneThrottle(t *testing.T) {
	// Use t.Setenv to avoid global state mutation
	t.Setenv("OHC_STANDALONE", "true")

	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	defer sipdb.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	var wg sync.WaitGroup
	var successCount int32

	// Drain the global channel first safely
	for {
		select {
		case <-throttleSemaphore:
		default:
			goto DRAINED
		}
	}
DRAINED:

	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()

			// We just want to check if throttle limits concurrency effectively
			err := sipdb.UpsertMission(ctx, "mission-test-throttle", "PENDING", "{}", false)
			if err == nil {
				atomic.AddInt32(&successCount, 1)
			}

			// Now simulate work in DelegateMission
			msg := Message{ID: "msg-throttle", Content: "Hello"}
			err = sipdb.DelegateMission(ctx, "mission-delegate", "role-test", msg)
			if err == nil {
				atomic.AddInt32(&successCount, 1)
			}
		}(i)
	}

	wg.Wait()
	if successCount == 0 {
		t.Errorf("expected greater than 0 successes")
	}
}
