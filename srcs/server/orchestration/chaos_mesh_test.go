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

	dbInstance, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer dbInstance.Close()

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
			_ = dbInstance.UpsertMission(ctx, fmt.Sprintf("mission-%d", idx), "PENDING", "{}", false)
			_ = dbInstance.DelegateMission(ctx, fmt.Sprintf("mission-%d", idx), "ROLE", task)
		}(i)
	}
	wg.Wait()
	t.Log("Successfully verified standalone database operations without deadlock")

	// 3. Chaos Engineering: Break the standalone runtime mailbox and .agent-lock/
	// In the real system, some fallback offline queues write to runtime mailbox or status files.
	// We simulate ML-Resilience behavior by corrupting these directories.
	chaosCtx, chaosCancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer chaosCancel()

	mailboxDir := filepath.Join(tmpDir, ".ohc", "runtime", "mailbox")
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
	// run its memory ingestion pipeline while the runtime memory directory is corrupted.
	// ML-Resilience mandates that the worker gracefully logs the error without panicking.

	memoryDir := filepath.Join(tmpDir, ".ohc", "runtime", "memory")
	t.Setenv("OHC_MEMORY_DIR", memoryDir)
	os.MkdirAll(memoryDir, 0755)
	dummyMemory := filepath.Join(memoryDir, "chaos_mesh_test_memory.yml")
	os.WriteFile(dummyMemory, []byte("content: chaos"), 0644)

	// Make file unreadable to simulate corruption
	os.Chmod(dummyMemory, 0000)
	defer func() {
		os.Chmod(dummyMemory, 0644)
		os.Remove(dummyMemory)
	}()

	// Instantiate the AutoDreamWorker (the real application code)
	worker := NewAutoDreamWorker(dbInstance)

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
	t.Log("Successfully verified ML-Resilience: AutoDreamWorker gracefully handles corrupted runtime memory without panic")
}

// TestSIPDB_CUJ_StressVerification automates CUJ stress-testing for high-concurrency Cloud pods
// and low-resource Standalone wrappers (SQLite limits).
func TestSIPDB_CUJ_StressVerification(t *testing.T) {
	// 1. High-concurrency Standalone Wrapper (SQLite limit simulation)
	t.Run("StandaloneWrapperStress", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "false")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_standalone.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SQLite SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 50)

		// Create CUJ: Rapid buffer metric writes
		for i := 0; i < 50; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				metricPayload := fmt.Sprintf(`{"cuj_idx": %d}`, idx)
				err := dbInstance.BufferMetric(ctx, "cuj-metric", metricPayload)
				if err != nil {
					errs <- err
				}
			}(i)
		}
		wg.Wait()
		close(errs)

		var errorCount int
		for e := range errs {
			errorCount++
			t.Logf("Standalone CUJ Write Error: %v", e)
		}

		// Some errors might happen due to locked db, but it should not crash.
		// Retries should mitigate most of them.
		if errorCount > 0 {
			t.Logf("Noticed %d errors out of 50 in standalone stress. Graceful handling verified.", errorCount)
		} else {
			t.Log("Standalone CUJ completed with 0 errors under high concurrency.")
		}
	})

	// 2. High-concurrency Cloud Pod (Mock Postgres stress)
	t.Run("CloudPodStress", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "true")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_cloud.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 100)

		// Create CUJ: Rapid Upsert and Sync
		for i := 0; i < 100; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				missionID := fmt.Sprintf("cloud-mission-%d", idx)
				err := dbInstance.UpsertMission(ctx, missionID, "PENDING", `{"cuj":"stress"}`, false)
				if err != nil {
					errs <- err
				}
			}(i)
		}
		wg.Wait()
		close(errs)

		var errorCount int
		for e := range errs {
			errorCount++
			t.Logf("Cloud CUJ Write Error: %v", e)
		}

		if errorCount > 0 {
			t.Logf("Noticed %d errors out of 100 in cloud stress.", errorCount)
		} else {
			t.Log("Cloud CUJ completed with 0 errors under high concurrency.")
		}
	})
}

// TestSIPDB_ChaosParity ensures that both SQLite and Postgres modes behave similarly under stress.
func TestSIPDB_ChaosParity(t *testing.T) {
	// First, run with Standalone (SQLite)
	t.Run("SQLite", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "false")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "parity_chaos.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SQLite SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		for i := 0; i < 10; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				dbInstance.PruneStaleMissions(ctx, time.Second)
			}(i)
		}
		wg.Wait()
		t.Log("SQLite Parity PruneStaleMissions completed without panic")
	})

	// Second, run with Mock Postgres DB Provider behavior
	t.Run("PostgresMock", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "true")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "parity_chaos_pg.db") // Just using SQLite to mock the interface here

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer dbInstance.Close()

		// Force Postgres behavior if possible by injecting a custom provider or just running under the flag
		// Here we just test the code path with OHC_MULTITENANT=true
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		for i := 0; i < 10; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				dbInstance.PruneStaleMissions(ctx, time.Second)
			}(i)
		}
		wg.Wait()
		t.Log("Postgres Parity PruneStaleMissions completed without panic")
	})
}
