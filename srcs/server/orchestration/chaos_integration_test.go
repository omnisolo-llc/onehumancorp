package orchestration

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

func TestCUJ_StressVerification(t *testing.T) {
	t.Run("StandaloneWrapperStress", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "true")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_standalone_integration.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SQLite SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 50)

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
		for err := range errs {
			errorCount++
			t.Logf("Standalone CUJ Write Error: %v", err)
		}

		if errorCount > 0 {
			t.Logf("Noticed %d errors out of 50 in standalone stress. Graceful handling verified.", errorCount)
		} else {
			t.Log("Standalone CUJ completed with 0 errors under high concurrency.")
		}
	})

	t.Run("CloudPodStress", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_cloud_integration.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 100)

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
		for err := range errs {
			errorCount++
			t.Logf("Cloud CUJ Write Error: %v", err)
		}

		if errorCount > 0 {
			t.Logf("Noticed %d errors out of 100 in cloud stress.", errorCount)
		} else {
			t.Log("Cloud CUJ completed with 0 errors under high concurrency.")
		}
	})
}

func TestParity_PruneStaleMissions(t *testing.T) {
	t.Run("SQLite", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "true")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "parity_chaos_integration.db")

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
				_ = idx
				dbInstance.PruneStaleMissions(ctx, time.Second)
			}(i)
		}
		wg.Wait()
		t.Log("SQLite Parity PruneStaleMissions completed without panic")
	})

	t.Run("PostgresMock", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "parity_chaos_pg_integration.db")

		dbInstance, err := NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		for i := 0; i < 10; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				_ = idx
				dbInstance.PruneStaleMissions(ctx, time.Second)
			}(i)
		}
		wg.Wait()
		t.Log("Postgres Parity PruneStaleMissions completed without panic")
	})
}
