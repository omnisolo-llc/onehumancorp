package orchestration

import (
	"context"
	"os"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestStandaloneSQLiteConcurrencyThrottling(t *testing.T) {
	// Setup Standalone Env
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	sqliteDB, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to initialize SIPDB: %v", err)
	}
	defer sqliteDB.Close()

	// Drain any existing tokens in the semaphore to ensure clean state
	for {
		select {
		case <-throttleSemaphore:
		default:
			goto DRAINED
		}
	}
DRAINED:

	var wg sync.WaitGroup
	var completedCount int
	var mu sync.Mutex

	// We start 10 concurrent writes
	numWrites := 10
	wg.Add(numWrites)

	start := time.Now()

	for i := 0; i < numWrites; i++ {
		go func(id int) {
			defer wg.Done()

			// Create dummy mission
			err := sqliteDB.DelegateMission(context.Background(), "mission-1", "role-1", Message{ID: "m1", Content: "test"})
			if err != nil {
				t.Errorf("DelegateMission failed: %v", err)
				return
			}

			mu.Lock()
			completedCount++
			mu.Unlock()
		}(i)
	}

	wg.Wait()
	duration := time.Since(start)

	if completedCount != numWrites {
		t.Fatalf("Expected %d completed writes, got %d", numWrites, completedCount)
	}

	// This is a rough check to ensure things ran correctly
	t.Logf("Throttled writes completed successfully in %v", duration)
}
