package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestTokenBurnRateForecaster(t *testing.T) {
	var mu sync.Mutex
	var lastRate float64
	var callbackCalled bool

	// Override telemetry function to intercept the metric
	oldRecordTokenBurnRate := telemetry.RecordTokenBurnRate
	telemetry.RecordTokenBurnRate = func(ctx context.Context, orgID string, rate float64) {
		mu.Lock()
		defer mu.Unlock()
		lastRate = rate
		callbackCalled = true
	}
	defer func() {
		telemetry.RecordTokenBurnRate = oldRecordTokenBurnRate
	}()

	// Short window and update interval for testing
	forecaster := NewTokenBurnRateForecaster(1*time.Second, 100*time.Millisecond)
	forecaster.Start()
	defer forecaster.Stop()

	// Record some usage
	forecaster.RecordUsage("test-tenant", 1000)
	forecaster.RecordUsage("test-tenant", 2000)

	// Wait for worker to calculate
	time.Sleep(300 * time.Millisecond)

	mu.Lock()
	if !callbackCalled {
		t.Fatal("expected RecordTokenBurnRate to be called")
	}
	if lastRate <= 0 {
		t.Fatalf("expected positive burn rate, got %f", lastRate)
	}
	mu.Unlock()
}

func TestDelegateMissionConcurrencyThrottling(t *testing.T) {
	// Enable standalone mode which triggers the throttle
	oldVal := os.Getenv("OHC_STANDALONE")
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Setenv("OHC_STANDALONE", oldVal)

	dbPath := filepath.Join(t.TempDir(), "test_throttle.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	// Since throttleSemaphore has capacity 2, running 5 concurrent delegates
	// should not result in "database is locked" errors due to proper throttling.
	var wg sync.WaitGroup
	errCh := make(chan error, 5)

	msg := Message{ID: "msg", Content: "test task"}
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			err := db.DelegateMission(context.Background(), "mission-1", "AGENT", msg)
			if err != nil {
				errCh <- err
			}
		}(i)
	}

	wg.Wait()
	close(errCh)

	for err := range errCh {
		if err != nil {
			t.Fatalf("Unexpected error under concurrency: %v", err)
		}
	}
}
