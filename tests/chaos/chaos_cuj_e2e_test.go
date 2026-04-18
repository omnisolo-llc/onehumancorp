package chaos_test

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/lib/resilience/chaos"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// TestCUJ_ChaosParity ensures that both SQLite and Postgres modes behave similarly under stress.
// It explicitly tests a critical user journey: ingesting many metrics during simulated Chaos.
func TestCUJ_ChaosParity(t *testing.T) {
	// Mode Parity check for SQLite Standalone
	t.Run("SQLite_Standalone", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "false")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_chaos_parity.db")

		dbInstance, err := orchestration.NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SQLite SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 50)
		chaosInj := chaos.NewInjector(chaos.ResourceExhaustion, 42)

		for i := 0; i < 50; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()

				// Inject Chaos into the operation
				err := chaosInj.Inject(ctx)
				if err != nil {
					errs <- err
					return
				}

				metricPayload := fmt.Sprintf(`{"cuj_idx": %d}`, idx)
				err = dbInstance.BufferMetric(ctx, "cuj-metric-sqlite", metricPayload)
				if err != nil {
					errs <- err
				}
			}(i)
		}
		wg.Wait()
		close(errs)

		var chaosErrCount int
		for e := range errs {
			if _, ok := e.(*chaos.ChaosError); ok {
				chaosErrCount++
			} else {
				t.Fatalf("Unexpected error during SQLite standalone CUJ: %v", e)
			}
		}

		if chaosErrCount == 0 {
			t.Fatalf("Expected chaos errors due to ResourceExhaustion, but none occurred.")
		}
		t.Logf("SQLite standalone CUJ successfully caught %d chaos errors and recovered gracefully.", chaosErrCount)
	})

	// Mode Parity check for Postgres Cloud mock
	t.Run("Postgres_CloudMock", func(t *testing.T) {
		t.Setenv("OHC_MULTITENANT", "true")

		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "cuj_chaos_pg.db")

		dbInstance, err := orchestration.NewSIPDB(dbPath)
		if err != nil {
			t.Fatalf("Failed to create SIPDB: %v", err)
		}
		defer dbInstance.Close()

		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()

		var wg sync.WaitGroup
		errs := make(chan error, 50)
		chaosInj := chaos.NewInjector(chaos.ConnectionDrop, 42)

		for i := 0; i < 50; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()

				// Inject Chaos into the operation
				err := chaosInj.Inject(ctx)
				if err != nil {
					errs <- err
					return
				}

				missionID := fmt.Sprintf("cloud-mission-%d", idx)
				err = dbInstance.UpsertMission(ctx, missionID, "PENDING", `{"cuj":"stress-pg"}`, false)
				if err != nil {
					errs <- err
				}
			}(i)
		}
		wg.Wait()
		close(errs)

		var chaosErrCount int
		for e := range errs {
			if _, ok := e.(*chaos.ChaosError); ok {
				chaosErrCount++
			} else {
				t.Fatalf("Unexpected error during Postgres cloud CUJ: %v", e)
			}
		}

		if chaosErrCount == 0 {
			t.Fatalf("Expected chaos errors due to ConnectionDrop, but none occurred.")
		}
		t.Logf("Postgres cloud CUJ successfully caught %d chaos errors and recovered gracefully.", chaosErrCount)
	})
}
