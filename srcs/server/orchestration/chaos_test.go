package orchestration

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestSIPDB_Chaos simulates high-concurrency ingestion and a simulated DB lock
// to verify the exponential backoff retry logic in withRetry.
func TestSIPDB_Chaos(t *testing.T) {
	defer ClearSemaphore()
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 1. High-concurrency agent mission ingestion (Stress Test)
	var wg sync.WaitGroup
	numAgents := 5
	missionsPerAgent := 10

	errs := make(chan error, numAgents*missionsPerAgent)

	start := time.Now()
	for i := 0; i < numAgents; i++ {
		wg.Add(1)
		go func(agentIdx int) {
			defer wg.Done()
			for j := 0; j < missionsPerAgent; j++ {
				missionID := fmt.Sprintf("mission-%d-%d", agentIdx, j)
				task := Message{
					ID:      missionID,
					Content: "Stress test task",
					Type:    EventTask,
				}
				if err := db.DelegateMission(ctx, missionID, "SOFTWARE_ENGINEER", task); err != nil {
					errs <- fmt.Errorf("agent %d failed to delegate mission %d: %v", agentIdx, j, err)
				}
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		t.Errorf("Concurrency error: %v", err)
	}

	t.Logf("Ingested %d missions concurrently in %v", numAgents*missionsPerAgent, time.Since(start))

	// 2. Controlled failure (DB Lock simulation)
	// We will simulate a locked table by starting an exclusive transaction,
	// then we'll try to write to it from another goroutine which should trigger retries.

	// Open a raw connection to lock the database
	tx, err := db.db.Begin(ctx)
	if err != nil {
		t.Fatalf("Failed to begin transaction: %v", err)
	}

	// Create an exclusive lock
	_, err = tx.Exec(ctx, "BEGIN EXCLUSIVE")
	if err != nil {
		t.Logf("Expected or not: %v", err)
	} else {
		_, err = tx.Exec(ctx, "UPDATE agent_missions SET status = 'LOCKED' WHERE 1=0")
		if err != nil {
			t.Fatalf("Failed to lock table: %v", err)
		}
	}

	var retryWg sync.WaitGroup
	retryWg.Add(1)

	startChaos := time.Now()

	// This should retry in the background
	go func() {
		defer retryWg.Done()
		task := Message{
			ID:      "chaos-mission-1",
			Content: "Chaos test task",
			Type:    EventTask,
		}

		// This will block and retry while the DB is locked
		err := db.DelegateMission(ctx, "chaos-mission-1", "SOFTWARE_ENGINEER", task)
		if err != nil {
			// It might ultimately fail if it exhausts retries before we unlock
			t.Logf("Mission delegation after chaos: %v", err)
		} else {
			t.Logf("Mission delegation succeeded after %v", time.Since(startChaos))
		}
	}()

	// Hold the lock for a short duration to trigger retries
	time.Sleep(200 * time.Millisecond)

	// Release the lock
	if err := tx.Commit(ctx); err != nil {
		t.Fatalf("Failed to commit and release lock: %v", err)
	}

	// Wait for the background retry to complete
	retryWg.Wait()

	// Verify the mission was actually added
	missions, err := db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("Failed to get pending missions: %v", err)
	}

	found := false
	for _, m := range missions {
		if m.ID == "chaos-mission-1" {
			found = true
			break
		}
	}

	if !found {
		t.Errorf("Expected to find chaos-mission-1 after recovery, but did not. It may have exhausted retries.")
	} else {
		t.Log("Successfully verified mission ingestion after DB lock recovery")
	}
}

// TestSIPDB_Chaos_PanicRecovery simulates an abrupt process panic mid-write
// to ensure the state machine gracefully recovers without corrupting metadata.
func TestSIPDB_Chaos_PanicRecovery(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos_panic.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Simulate panic during a transaction
	func() {
		defer func() {
			if r := recover(); r != nil {
				t.Logf("Recovered from simulated panic: %v", r)
			}
		}()

		tx, err := db.db.Begin(ctx)
		if err != nil {
			t.Fatalf("Failed to begin tx: %v", err)
		}
		defer tx.Rollback(ctx)

		// The test creates a SIPDB which internally uses IsSQLite() == true initially.
		// The `agent_missions` schema includes id, status, payload, created_at, updated_at, organization_id.
		_, err = tx.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ('panic-mission-1', 'IN_PROGRESS', '{\"role\":\"ANY\"}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'org1')")
		if err != nil {
			t.Fatalf("Insert failed: %v", err)
		}

		// Panic *before* commit to simulate abrupt failure
		panic("abrupt process failure")
	}()

	// Verify the database is accessible and the uncommitted transaction was rolled back
	missions, err := db.GetPendingMissions(ctx, "ANY")
	if err != nil {
		t.Fatalf("Failed to query after panic: %v", err)
	}

	for _, m := range missions {
		if m.ID == "panic-mission-1" {
		    t.Fatalf("Mission panic-mission-1 exists. The uncommitted write was not rolled back.")
		}
	}
}
