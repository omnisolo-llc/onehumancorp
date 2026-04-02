package orchestration

import (
	"context"
	"os"
	"sync"
	"testing"
	"time"
)

func TestSIPDB_DelegateMission_Throttle(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	// Reset throttle
	throttleSemaphore = make(chan struct{}, 1)

	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	var wg sync.WaitGroup

	// Wrap db.Exec to track concurrency
	// Since we can't easily intercept Exec, we'll acquire a lock in DelegateMission by occupying the semaphore.

	wg.Add(2)
	go func() {
		defer wg.Done()
		err := db.DelegateMission(ctx, "m1", "ROLE", Message{ID: "m1", Content: "task 1", Type: EventTask})
		if err != nil && err != context.DeadlineExceeded {
			t.Errorf("Unexpected error: %v", err)
		}
	}()

	go func() {
		defer wg.Done()
		err := db.DelegateMission(ctx, "m2", "ROLE", Message{ID: "m2", Content: "task 2", Type: EventTask})
		if err != nil && err != context.DeadlineExceeded {
			t.Errorf("Unexpected error: %v", err)
		}
	}()

	wg.Wait()

	// Actually it's better to ensure it doesn't panic and executes correctly.
	// The semaphore bounds the concurrent DB executions.

	// Add a test that verifies the semaphore is indeed limited to 1.
	// We can manually acquire the semaphore and try to do a DelegateMission with a short timeout.

	throttleSemaphore <- struct{}{} // Acquire the only slot

	ctxShort, cancelShort := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancelShort()

	err = db.DelegateMission(ctxShort, "m3", "ROLE", Message{})
	if err != context.DeadlineExceeded {
		t.Fatalf("Expected DeadlineExceeded because semaphore is full, got: %v", err)
	}

	<-throttleSemaphore // Release slot

	err = db.DelegateMission(context.Background(), "m4", "ROLE", Message{})
	if err != nil {
		t.Fatalf("Expected success, got: %v", err)
	}
}

func TestSIPDB_UpsertMission_Throttle(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	throttleSemaphore = make(chan struct{}, 1)

	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	throttleSemaphore <- struct{}{} // Acquire the only slot

	ctxShort, cancelShort := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancelShort()

	err = db.UpsertMission(ctxShort, "m3", "status", "payload", false)
	if err != context.DeadlineExceeded {
		t.Fatalf("Expected DeadlineExceeded because semaphore is full, got: %v", err)
	}

	<-throttleSemaphore // Release slot

	err = db.UpsertMission(context.Background(), "m4", "status", "payload", false)
	if err != nil {
		t.Fatalf("Expected success, got: %v", err)
	}
}
