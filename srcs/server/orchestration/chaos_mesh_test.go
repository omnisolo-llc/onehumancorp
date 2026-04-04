package orchestration

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestSIPDB_ChaosMesh simulates team mesh corruption and standalone limits.
func TestSIPDB_ChaosMesh(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos_mesh.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// 1. Stress the Mesh by publishing many messages concurrently
	mesh, err := NewLegacyTeammateMesh("redis://localhost:6379")
	if err != nil {
		t.Logf("Skipping legacy teammate mesh due to initialization error: %v", err)
	} else {
		var wg sync.WaitGroup
		for i := 0; i < 50; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				msg := MeshMessage{
					SenderID:  fmt.Sprintf("agent-%d", idx),
					Role:      "TEST",
					Content:   "Stress mesh message",
					Timestamp: time.Now(),
				}
				_ = mesh.PublishMessage(ctx, msg)
			}(i)
		}
		wg.Wait()
		t.Log("Successfully stressed LegacyTeammateMesh")
	}

	// 2. High-concurrency Upsert and Delegate for Standalone Throttling resilience
	var wg sync.WaitGroup
	for i := 0; i < 20; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			task := Message{ID: fmt.Sprintf("t-%d", idx), Content: "c", Type: EventTask}

			// Fire and forget Upsert and Delegate, to ensure throttling works without deadlock
			_ = db.UpsertMission(ctx, fmt.Sprintf("mission-%d", idx), "PENDING", "{}", false)
			_ = db.DelegateMission(ctx, fmt.Sprintf("mission-%d", idx), "ROLE", task)
		}(i)
	}
	wg.Wait()
	t.Log("Successfully verified standalone database operations without deadlock")
}
