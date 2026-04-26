package orchestration

import (
	"context"
	"path/filepath"
	"sync"
	"sync/atomic"
	"testing"
	"time"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestSIPDB_DelegateMission_ConcurrencyThrottle(t *testing.T) {
	// Clear the package-level semaphore after the test.
	defer ClearSemaphore()
	ClearSemaphore() // Ensure it's clear before starting

	// Temporarily enable OHC_STANDALONE to trigger the throttle
	t.Setenv("OHC_STANDALONE", "true")

	dbPath := filepath.Join(t.TempDir(), "throttle.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	msg := Message{
		ID:      "m1",
		Content: "Test content",
		Type:    EventTask,
	}

	var wg sync.WaitGroup
	startChan := make(chan struct{})

	concurrency := 10
	errChan := make(chan error, concurrency)

	// We simulate many concurrent DelegateMission calls.
	// The semaphore ensures they don't hit "database is locked" errors and crash.
	for i := 0; i < concurrency; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			<-startChan
			// Execute
			err := db.DelegateMission(context.Background(), "m"+string(rune(i)), "ROLE", msg)
			if err != nil {
				errChan <- err
			}
		}(i)
	}

	// Release all goroutines at once
	close(startChan)

	// Wait with timeout
	doneChan := make(chan struct{})
	go func() {
		wg.Wait()
		close(doneChan)
	}()

	select {
	case <-time.After(5 * time.Second):
		t.Fatal("Test timed out, possible deadlock in throttle semaphore")
	case <-doneChan:
	}

	close(errChan)
	for err := range errChan {
		t.Fatalf("Unexpected error from DelegateMission under load: %v", err)
	}
}

func TestSIPDB_DelegateMission_ConcurrencyThrottleTelemetry(t *testing.T) {
	defer ClearSemaphore()
	ClearSemaphore()

	t.Setenv("OHC_STANDALONE", "true")

	// Set up telemetry mock
	var called int32
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		if metricType == "sqlite_throttled_request" {
			atomic.AddInt32(&called, 1)
		}
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	dbPath := filepath.Join(t.TempDir(), "throttle_telemetry.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	msg := Message{
		ID:      "m1",
		Content: "Test content",
		Type:    EventTask,
	}

	var wg sync.WaitGroup
	startChan := make(chan struct{})

	concurrency := 3
	// Hold the lock to force contention
	acquireThrottle(context.Background())
	errChan := make(chan error, concurrency)

	for i := 0; i < concurrency; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			<-startChan
			err := db.DelegateMission(context.Background(), "m"+string(rune(i)), "ROLE", msg)
			if err != nil {
				errChan <- err
			}
		}(i)
	}

	close(startChan)
	// Wait briefly to allow goroutines to hit the throttle and increment the counter
	time.Sleep(50 * time.Millisecond)
	releaseThrottle()
	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Fatalf("Unexpected error: %v", err)
	}

	if atomic.LoadInt32(&called) == 0 {
		t.Errorf("Expected sqlite_throttled_request to be called, but got %d", called)
	}
}
