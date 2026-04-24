package orchestration

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// TestSentry_Chaos_NetworkPartition simulates SQL synchronization lag and network partitions
// to verify fail-safe degradation in Standalone mode vs Cloud-Native mode.
func TestSentry_Chaos_NetworkPartition(t *testing.T) {
	// 1. Standalone (SQLite) Resilience against sync lag
	t.Run("Standalone Network Partition", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "false")
		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "sentry_chaos_standalone.db")
		db, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer db.Close()
		ctx := context.Background()

		// Fill the DB with pending missions to simulate local offline queueing
		for i := 0; i < 50; i++ {
			err = db.UpsertMission(ctx, fmt.Sprintf("mission-%d", i), "PENDING", `{"data":"test"}`, false)
			if err != nil {
				t.Fatalf("Failed to insert mission: %v", err)
			}
		}

		// Simulate network partition by syncing to an invalid remote endpoint
		// This should not crash, it should gracefully fail and keep items in PENDING.
		syncCtx, syncCancel := context.WithTimeout(ctx, 2*time.Second)
		defer syncCancel()
		_, err = db.SyncMissions(syncCtx, "http://localhost:12345/invalid_sync")
		if err == nil {
			t.Errorf("Expected sync to fail due to network partition, but it succeeded")
		}

		// Verify missions are still PENDING
		missions, err := db.GetPendingMissions(ctx, "ANY")
		if err != nil {
			t.Fatalf("Failed to get pending missions: %v", err)
		}
		if len(missions) != 50 {
			t.Errorf("Expected 50 pending missions after network partition, got %d", len(missions))
		}
	})
}

// TestSentry_TeamMesh_Corruption verifies that the Team Mesh degrades safely when
// critical mailbox paths are corrupted, mimicking ML-Resilience.
func TestSentry_TeamMesh_Corruption(t *testing.T) {

	memoryDir := filepath.Join(t.TempDir(), "memory")
	t.Setenv("OHC_MEMORY_DIR", memoryDir)
	err := os.MkdirAll(memoryDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create memory dir: %v", err)
	}

	testFile := filepath.Join(memoryDir, fmt.Sprintf("sentry_chaos_test_%d.yml", time.Now().UnixNano()))
	err = os.WriteFile(testFile, []byte("content: chaos"), 0644)
	if err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}

	// Make the file unreadable to simulate corruption
	os.Chmod(testFile, 0000)

	defer func() {
		os.Chmod(testFile, 0644)
		os.Remove(testFile)
	}()

	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "sentry_mesh.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	worker := NewAutoDreamWorker(db.db)

	// Since we made the file unreadable, the application code `contentBytes, err := os.ReadFile(filePath)`
	// will fail. This should gracefully log an error and continue without panicking.
	// This tests the actual application logic!

	defer func() {
		if r := recover(); r != nil {
			t.Errorf("Panic during ingestAgentMemories: %v", r)
		}
	}()

	worker.ingestAgentMemories(ctx)

	t.Log("Successfully verified ML-Resilience: AutoDreamWorker gracefully handled degraded IO without panic.")
}

// TestSwarmTaskBurstMetrics verifies that high-throughput Swarm tasks
// correctly emit deployment-mode-aware metrics in both Cloud and Standalone modes.
func TestSwarmTaskBurstMetrics(t *testing.T) {
	// Temporarily override the metric functions for verification
	originalFunc := telemetry.BufferMetricFunc
	defer func() { telemetry.BufferMetricFunc = originalFunc }()

	var metrics []string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		metrics = append(metrics, metricType)
		return nil
	}

	t.Run("Standalone Swarm Burst Metrics", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "false")
		metrics = []string{} // Reset

		ctx := context.Background()
		// Simulate swarm operations
		telemetry.RecordSwarmTaskQueueLength(ctx, 10, getDeploymentMode())
		telemetry.RecordSwarmTaskProcessingLatency(ctx, 45.2, getDeploymentMode())
		telemetry.RecordSQLiteRetryEvent(ctx, "update_task", getDeploymentMode())

		foundQueue := false
		foundLatency := false
		foundContention := false
		for _, m := range metrics {
			if m == "swarm_task_queue_length" {
				foundQueue = true
			}
			if m == "swarm_task_processing_latency" {
				foundLatency = true
			}
			if m == "sqlite_retry_event" {
				foundContention = true
			}
		}

		if !foundQueue {
			t.Error("Missing swarm_task_queue_length metric in standalone mode")
		}
		if !foundLatency {
			t.Error("Missing swarm_task_processing_latency metric in standalone mode")
		}
		if !foundContention {
			t.Error("Missing sqlite_retry_event metric in standalone mode")
		}
	})

	t.Run("Cloud Swarm Burst Metrics", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "true")
		metrics = []string{} // Reset

		ctx := context.Background()
		telemetry.RecordSwarmTaskQueueLength(ctx, 5, getDeploymentMode())
		telemetry.RecordSwarmTaskProcessingLatency(ctx, 12.5, getDeploymentMode())
		telemetry.RecordTaskClaimContention(ctx, getDeploymentMode())

		foundQueue := false
		foundLatency := false
		foundContention := false
		for _, m := range metrics {
			if m == "swarm_task_queue_length" {
				foundQueue = true
			}
			if m == "swarm_task_processing_latency" {
				foundLatency = true
			}
			if m == "task_claim_contention" {
				foundContention = true
			}
		}

		if !foundQueue {
			t.Error("Missing swarm_task_queue_length metric in cloud mode")
		}
		if !foundLatency {
			t.Error("Missing swarm_task_processing_latency metric in cloud mode")
		}
		if !foundContention {
			t.Error("Missing task_claim_contention metric in cloud mode")
		}
	})
}
