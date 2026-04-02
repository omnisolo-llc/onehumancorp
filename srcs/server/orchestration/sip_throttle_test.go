package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
)

func TestSIPDB_StandaloneThrottle(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	dbPath := filepath.Join(t.TempDir(), "test_throttle.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Use WaitGroup to run concurrent DelegateMission calls
	var wg sync.WaitGroup
	const numConcurrent = 10
	errChan := make(chan error, numConcurrent)

	// Since SQLite is locked without throttling, having a max concurrency of 1 ensures no DB errors
	for i := 0; i < numConcurrent; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			missionID := "mission-throttle-" + string(rune(id))
			msg := Message{Content: "throttle task"}
			err := db.DelegateMission(ctx, missionID, "THROTTLED_AGENT", msg)
			if err != nil {
				errChan <- err
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("Unexpected error during throttled DelegateMission: %v", err)
	}
}

func TestSIPDB_UpsertMission_StandaloneThrottle(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	dbPath := filepath.Join(t.TempDir(), "test_throttle_upsert.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	var wg sync.WaitGroup
	const numConcurrent = 10
	errChan := make(chan error, numConcurrent)

	for i := 0; i < numConcurrent; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			missionID := "mission-throttle-upsert-" + string(rune(id))
			err := db.UpsertMission(ctx, missionID, "PENDING", `{"payload": "test"}`, true)
			if err != nil {
				errChan <- err
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("Unexpected error during throttled UpsertMission: %v", err)
	}
}
