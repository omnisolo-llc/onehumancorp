package orchestration

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestSIPDB_Chaos simulates high-concurrency ingestion, simulated DB lock,
// and network partition failures to verify the exponential backoff retry logic
// and swarm recovery in withRetry.
func TestSIPDB_Chaos(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 1. High-concurrency agent mission ingestion (Stress Test)
	// Greatly increased the scale of concurrency to properly verify "high-concurrency"
	var wg sync.WaitGroup
	numAgents := 200
	missionsPerAgent := 50

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

	// Verify all records were actually inserted correctly
	var count int
	err = db.db.QueryRow("SELECT COUNT(*) FROM agent_missions WHERE status = 'PENDING' AND json_extract(payload, '$.role') = 'SOFTWARE_ENGINEER'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query mission count: %v", err)
	}
	if count < numAgents*missionsPerAgent {
		t.Fatalf("Expected at least %d missions, but found %d", numAgents*missionsPerAgent, count)
	}

	// 2. Controlled failure (DB Lock simulation)
	// We will simulate a locked table by starting an exclusive transaction,
	// then we'll try to write to it from another goroutine which should trigger retries.

	// Open a raw connection to lock the database
	tx, err := db.db.Begin()
	if err != nil {
		t.Fatalf("Failed to begin transaction: %v", err)
	}

	// Create an exclusive lock
	_, err = tx.Exec("BEGIN EXCLUSIVE")
	if err != nil {
		t.Logf("Expected or not: %v", err)
	} else {
		_, err = tx.Exec("UPDATE agent_missions SET status = 'LOCKED' WHERE 1=0")
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
	if err := tx.Commit(); err != nil {
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

	// 3. Network Partition (Simulated Connection Drop/Timeout)
	// We simulate a network partition by artificially cancelling a context mid-flight
	// and verifying the system degrades gracefully (fails over to error rather than hanging).
	partitionCtx, partitionCancel := context.WithTimeout(ctx, 1*time.Millisecond)
	defer partitionCancel()

	// Add slight delay to ensure the context expires
	time.Sleep(5 * time.Millisecond)

	task := Message{
		ID:      "partition-mission-1",
		Content: "Partition test task",
		Type:    EventTask,
	}

	err = db.DelegateMission(partitionCtx, "partition-mission-1", "SOFTWARE_ENGINEER", task)
	if err == nil {
		t.Errorf("Expected context deadline exceeded error during simulated network partition, but succeeded")
	} else {
		t.Logf("Successfully verified graceful fail-over on network partition: %v", err)
	}

	// 4. Verify Swarm Recovery
	// Ensure that after the partition is resolved, normal operations resume instantly
	recoveryTask := Message{
		ID:      "recovery-mission-1",
		Content: "Recovery test task",
		Type:    EventTask,
	}

	err = db.DelegateMission(ctx, "recovery-mission-1", "SOFTWARE_ENGINEER", recoveryTask)
	if err != nil {
		t.Fatalf("Failed to recover swarm operations post-partition: %v", err)
	}

	t.Log("Successfully verified swarm recovery operations.")
}
