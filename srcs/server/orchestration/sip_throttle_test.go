package orchestration

import (
	"context"
	"os"
	"sync"
	"testing"
	"time"


	_ "modernc.org/sqlite"
)

func TestStandaloneThrottle(t *testing.T) {
	// Set OHC_STANDALONE to true
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	// Create a dummy SIPDB
	// Using memory mode
	sipDB, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}

	var wg sync.WaitGroup
	start := make(chan struct{})

	// Record execution times
	executionTimes := make([]time.Time, 5)

	// Redefine throttle semaphore just for tests if needed, or simply test behavior
	// Here we just test that multiple UpsertMissions do not fail under "standalone" mode,
	// verifying the code runs through the throttle path without deadlocking.
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-start
			err := sipDB.UpsertMission(context.Background(), "test-mission", "PENDING", "{}", false)
			if err != nil {
				t.Errorf("UpsertMission failed: %v", err)
			}
			executionTimes[idx] = time.Now()
		}(i)
	}

	close(start)

	// Use a timeout to ensure it doesn't deadlock
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		// Success
	case <-time.After(2 * time.Second):
		t.Fatal("Test deadlocked on UpsertMission")
	}

	// Test DelegateMission similarly
	wg = sync.WaitGroup{}
	start = make(chan struct{})

	msg := Message{ID: "m1", Content: "hello[Feature: missing]"}
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-start
			// Feature gate expects docs, so we will expect an error "missing required documentation: docs/features/missing/design-doc.md"
			// But the throttle occurs BEFORE the documentation gate in DelegateMission
			err := sipDB.DelegateMission(context.Background(), "mission-id", "role", msg)
			if err != nil && err.Error() != "missing required documentation: docs/features/missing/design-doc.md" {
				t.Errorf("Unexpected error: %v", err)
			}
		}(i)
	}
	close(start)

	done2 := make(chan struct{})
	go func() {
		wg.Wait()
		close(done2)
	}()

	select {
	case <-done2:
		// Success
	case <-time.After(2 * time.Second):
		t.Fatal("Test deadlocked on DelegateMission")
	}
}
