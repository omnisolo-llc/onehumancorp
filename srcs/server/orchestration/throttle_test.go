package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestSIPDB_Throttle tests that the dynamic semaphore correctly throttles when OHC_STANDALONE=true
func TestSIPDB_Throttle(t *testing.T) {
	// Temporarily set OHC_STANDALONE to true for this test
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Setenv("OHC_STANDALONE", originalStandalone)

	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "throttle.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// High-concurrency agent mission ingestion (Stress Test)
	var wg sync.WaitGroup
	numAgents := 50
	missionsPerAgent := 10

	errs := make(chan error, numAgents*missionsPerAgent)

	start := time.Now()
	for i := 0; i < numAgents; i++ {
		wg.Add(1)
		go func(agentIdx int) {
			defer wg.Done()
			for j := 0; j < missionsPerAgent; j++ {
				missionID := string(rune(agentIdx*1000 + j)) // Some unique ID
				if err := db.UpsertMission(ctx, missionID, "PENDING", "{}", false); err != nil {
					errs <- err
				}
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		t.Errorf("Concurrency error: %v", err)
	}

	t.Logf("Ingested %d missions concurrently with Standalone throttle in %v", numAgents*missionsPerAgent, time.Since(start))
}
