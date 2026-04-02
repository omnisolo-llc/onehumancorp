package orchestration_test

import (
	"context"
	"os"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestDelegateMissionThrottling(t *testing.T) {
	// Setup test database
	dbPath := "file:test_throttle.db?mode=memory&cache=shared"
	sipdb, err := orchestration.NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}

	// Create tables if not exists using UpsertMission or similar to make sure table is ready
	_ = sipdb.UpsertMission(context.Background(), "init", "PENDING", "{}", false)

	// Enable standalone mode using t.Setenv for better test safety
	t.Setenv("OHC_STANDALONE", "true")

	// We expect 2 concurrent executions to be allowed at max due to semaphore size = 2
	var wg sync.WaitGroup
	numWorkers := 10
	errChan := make(chan error, numWorkers)

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
			defer cancel()

			err := sipdb.DelegateMission(ctx, "mission-123", "SYSTEM", orchestration.Message{
				Content: "Test content",
			})
			if err != nil {
				errChan <- err
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	var errs []error
	for err := range errChan {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		t.Errorf("expected 0 errors, got %d. First error: %v", len(errs), errs[0])
	}
}
