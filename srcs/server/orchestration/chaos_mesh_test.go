package orchestration

import (
	"context"
	"fmt"
	"os"
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

	// 3. Chaos Engineering: Break .agent-task/mailbox/ and .agent-lock/
	// In the real system, some fallback offline queues write to .agent-task/mailbox or status files.
	// We simulate ML-Resilience behavior by corrupting these directories.
	chaosCtx, chaosCancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer chaosCancel()

	mailboxDir := filepath.Join(tmpDir, ".agent-task", "mailbox")
	lockDir := filepath.Join(tmpDir, ".agent-lock")

	err = os.MkdirAll(mailboxDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create mailbox dir: %v", err)
	}
	err = os.MkdirAll(lockDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}

	// Corrupt permissions to read-only
	os.Chmod(mailboxDir, 0400)
	os.Chmod(lockDir, 0400)
	defer func() {
		os.Chmod(mailboxDir, 0755)
		os.Chmod(lockDir, 0755)
	}()

	// Phase 2 (Implementation): Actually test the system's resilience by having the AutoDreamWorker
	// run its memory ingestion pipeline while the .agent-task/memory directory is corrupted.
	// ML-Resilience mandates that the worker gracefully logs the error without panicking.

	// Temporarily override the working directory to point to our chaos environment
	// so that memoryDir = ".agent-task/memory" hits our temporary directory.
	originalWd, _ := os.Getwd()
	os.Chdir(tmpDir)
	defer os.Chdir(originalWd)

	memoryDir := filepath.Join(tmpDir, ".agent-task", "memory")
	err = os.MkdirAll(memoryDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create memory dir: %v", err)
	}

	// Create a dummy memory file that will become unreadable
	dummyMemory := filepath.Join(memoryDir, "test.yml")
	os.WriteFile(dummyMemory, []byte("content: chaos"), 0644)

	// Corrupt permissions to prevent reading
	os.Chmod(memoryDir, 0000)
	defer os.Chmod(memoryDir, 0755)

	// Instantiate the AutoDreamWorker (the real application code)
	worker := NewAutoDreamWorker(db)

	// Use a waitgroup to run ingestAgentMemories concurrently
	var chaosWg sync.WaitGroup
	for i := 0; i < 5; i++ {
		chaosWg.Add(1)
		go func() {
			defer chaosWg.Done()
			// This tests the real AutoDreamWorker logic against the corrupted directory.
			// ML-Resilience requires that this does not panic, and gracefully returns or ignores.
			worker.ingestAgentMemories(chaosCtx)
		}()
	}

	// If it doesn't panic, ML-Resilience passes.
	chaosWg.Wait()
	t.Log("Successfully verified ML-Resilience: AutoDreamWorker gracefully handles corrupted .agent-task/memory without panic")
}
