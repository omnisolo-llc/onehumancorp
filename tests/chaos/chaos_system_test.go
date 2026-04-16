package chaos_test

import (
	"context"
	"fmt"
	"strings"
	"testing"
	"time"

	"onehumancorp.com/lib/resilience/chaos"
)

func TestChaosSystem_LatencySpike(t *testing.T) {
	injector := chaos.NewInjector(chaos.LatencySpike, 123)

	start := time.Now()
	err := injector.Inject(context.Background())
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if duration < 10*time.Millisecond || duration > 100*time.Millisecond {
		t.Fatalf("expected duration between 10ms and 100ms, got %v", duration)
	}
}

func TestChaosSystem_ConnectionDrop(t *testing.T) {
	injector := chaos.NewInjector(chaos.ConnectionDrop, 123)
	injector.SetProbability(1.0) // Force trigger for deterministic test

	err := injector.Inject(context.Background())
	if err == nil {
		t.Fatalf("expected connection drop to occur")
	}
}

func TestChaosSystem_ResourceExhaustion(t *testing.T) {
	injector := chaos.NewInjector(chaos.ResourceExhaustion, 123)
	injector.SetProbability(1.0) // Force trigger

	err := injector.Inject(context.Background())
	if err == nil {
		t.Fatalf("expected resource exhaustion to occur")
	}
}

// mockHarness simulates an Agent Harness component that uses withSipRetry logic.
type mockHarness struct {
	injector *chaos.Injector
}

func (h *mockHarness) PerformOperation(ctx context.Context) error {
	maxRetries := 3
	for i := 0; i < maxRetries; i++ {
		if err := h.injector.Inject(ctx); err != nil {
			if err.Error() == "chaos: agent lock corrupted" {
				continue
			}
			return err
		}
		return nil
	}
	return fmt.Errorf("operation failed after retries")
}

func TestCorruptAgentLockSystemResilience(t *testing.T) {
	inj := chaos.NewInjector(chaos.CorruptAgentLock, 42)
	inj.SetProbability(1.0)

	harness := &mockHarness{injector: inj}

	err := harness.PerformOperation(context.Background())
	if err != nil && !strings.Contains(err.Error(), "failed after retries") {
		t.Errorf("Unexpected error from harness: %v", err)
	}
}
