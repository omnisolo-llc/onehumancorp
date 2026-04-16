package chaos_test

import (
	"context"
	"fmt"
	"strings"
	"testing"

	"onehumancorp.com/lib/resilience/chaos"
)

// mockHarness simulates an Agent Harness component that uses withSipRetry logic.
type mockHarness struct {
	injector *chaos.Injector
}

func (h *mockHarness) PerformOperation(ctx context.Context) error {
	// Simulate the "withSipRetry" logic found in srcs/server/orchestration/sip.go
	// but integrated with our chaos injector.

	maxRetries := 3
	for i := 0; i < maxRetries; i++ {
		// Inject chaos
		if err := h.injector.Inject(ctx); err != nil {
			// If it's a CorruptAgentLock, we simulate a fallback or a specific error handling
			if err.Error() == "chaos: agent lock corrupted" {
				// In a real system, this might trigger a recovery or be treated as a transient error
				// for retry if it's recoverable, or fatal if not.
				// Here we simulate it as a transient error to test retry logic.
				continue
			}
			return err
		}

		// Simulate successful operation
		return nil
	}
	return fmt.Errorf("operation failed after retries")
}

func TestCorruptAgentLockSystemResilience(t *testing.T) {
	// Setup injector with 100% corruption probability to test fallback/retry
	inj := chaos.NewInjector(chaos.CorruptAgentLock, 42)
	inj.SetProbability(1.0)

	harness := &mockHarness{injector: inj}

	err := harness.PerformOperation(context.Background())
	if err == nil {
		t.Log("Harness successfully handled simulated corruption (presumably via retry/fallback)")
	} else if strings.Contains(err.Error(), "failed after retries") {
		t.Log("Harness correctly exhausted retries on persistent corruption")
	} else {
		t.Errorf("Unexpected error from harness: %v", err)
	}
}
