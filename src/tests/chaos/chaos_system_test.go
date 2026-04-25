package chaos_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/lib/resilience/chaos"
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

	foundDrop := false
	for i := 0; i < 100; i++ {
		err := injector.Inject(context.Background())
		if err != nil {
			if _, ok := err.(*chaos.ChaosError); !ok {
				t.Fatalf("expected ChaosError, got %T", err)
			}
			foundDrop = true
			break
		}
	}

	if !foundDrop {
		t.Fatalf("expected connection drop to occur within 100 iterations")
	}
}

func TestChaosSystem_ResourceExhaustion(t *testing.T) {
	injector := chaos.NewInjector(chaos.ResourceExhaustion, 123)

	foundExhaust := false
	for i := 0; i < 200; i++ {
		err := injector.Inject(context.Background())
		if err != nil {
			if _, ok := err.(*chaos.ChaosError); !ok {
				t.Fatalf("expected ChaosError, got %T", err)
			}
			foundExhaust = true
			break
		}
	}

	if !foundExhaust {
		t.Fatalf("expected resource exhaustion to occur within 200 iterations")
	}
}

func TestChaosSystem_CorruptAgentLock(t *testing.T) {
	injector := chaos.NewInjector(chaos.CorruptAgentLock, 123)

	err := injector.Inject(context.Background())
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	if _, ok := err.(*chaos.ChaosError); !ok {
		t.Fatalf("expected ChaosError, got %T", err)
	}
}

func TestChaosSystem_CorruptMailbox(t *testing.T) {
	injector := chaos.NewInjector(chaos.CorruptMailbox, 123)

	err := injector.Inject(context.Background())
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	if _, ok := err.(*chaos.ChaosError); !ok {
		t.Fatalf("expected ChaosError, got %T", err)
	}
}
