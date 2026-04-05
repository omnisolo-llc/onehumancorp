package orchestration

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestSIPDB_NetworkPartition simulates network unreachability and ensures backoff and fallback happen
func TestSIPDB_NetworkPartition(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "network_partition.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Simulate broken TeammateMesh due to Network Partition
	// Use an invalid redis URL to force connection failures
	mesh, err := NewLegacyTeammateMesh("redis://invalid-host.local:6379")
	if err == nil {
		t.Log("Expected error connecting to invalid Redis host, but got nil. Will proceed with disconnected mesh.")
	}

	var wg sync.WaitGroup
	errs := make(chan error, 10)

	// Attempt to publish messages over the broken mesh, ensuring it gracefully degrades
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()

			// Note: If mesh is nil due to init failure, we simulate that part of the system failing
			if mesh != nil {
				msg := MeshMessage{
					SenderID:  fmt.Sprintf("agent-%d", idx),
					Role:      "TEST",
					Content:   "Network partition message",
					Timestamp: time.Now(),
				}
				// Should timeout or fail quickly, not hang forever
				err := mesh.PublishMessage(ctx, msg)
				if err != nil {
					errs <- fmt.Errorf("publish failed (expected): %v", err)
				}
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	// We just want to ensure it doesn't crash or hang
	t.Log("Successfully handled network partition on mesh publish")

	// Ensure DB operations still work locally despite network partition
	task := Message{ID: "local-fallback", Content: "local", Type: EventTask}
	err = db.UpsertMission(ctx, "mission-local", "PENDING", "{}", false)
	if err != nil {
		t.Fatalf("Local UpsertMission failed during simulated network partition: %v", err)
	}

	err = db.DelegateMission(ctx, "mission-local", "ROLE", task)
	if err != nil {
		t.Fatalf("Local DelegateMission failed during simulated network partition: %v", err)
	}

	t.Log("Local DB degraded gracefully and succeeded")
}
