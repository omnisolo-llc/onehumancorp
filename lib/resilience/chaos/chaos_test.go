package chaos

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestNoChaos(t *testing.T) {
	inj := NewInjector(NoChaos, 1)
	err := inj.Inject(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestLatencySpike(t *testing.T) {
	inj := NewInjector(LatencySpike, 1)

	start := time.Now()
	err := inj.Inject(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	duration := time.Since(start)
	if duration < 10*time.Millisecond {
		t.Fatalf("expected delay > 10ms, got %v", duration)
	}
}

func TestContextCancellation(t *testing.T) {
	inj := NewInjector(LatencySpike, 1)

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	err := inj.Inject(ctx)
	if err == nil {
		t.Fatal("expected context error, got nil")
	}
}

func TestConnectionDrop(t *testing.T) {
	// Use a fixed seed known to trigger the drop quickly, or run it a few times
	inj := NewInjector(ConnectionDrop, 1)
	dropped := false
	for i := 0; i < 100; i++ {
		err := inj.Inject(context.Background())
		if err != nil {
			if e, ok := err.(*ChaosError); ok && e.Message == "chaos: simulated connection drop" {
				dropped = true
				break
			}
		}
	}
	if !dropped {
		t.Fatal("expected a connection drop to occur within 100 attempts")
	}
}

func TestResourceExhaustion(t *testing.T) {
	inj := NewInjector(ResourceExhaustion, 2)
	exhausted := false
	for i := 0; i < 100; i++ {
		err := inj.Inject(context.Background())
		if err != nil {
			if e, ok := err.(*ChaosError); ok && e.Message == "chaos: simulated resource exhaustion" {
				exhausted = true
				break
			}
		}
	}
	if !exhausted {
		t.Fatal("expected a resource exhaustion error to occur within 100 attempts")
	}
}

func TestCorruptAgentLock(t *testing.T) {
	// Setup: Test when directory does not exist
	// Ensure cleanup first just in case
	os.RemoveAll(".agent-lock/")

	inj := NewInjector(CorruptAgentLock, 1)
	err := inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected a corruption error, got nil")
	}
	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("unexpected error message: %v", err)
	}

	// Setup: Test when directory exists
	err = os.MkdirAll(".agent-lock/", 0755)
	if err != nil {
		t.Fatalf("failed to create .agent-lock/ directory: %v", err)
	}
	defer os.RemoveAll(".agent-lock/") // cleanup after test

	err = inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected a corruption error when directory exists, got nil")
	}
	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("unexpected error message: %v", err)
	}

	// Verify the corruption file was written
	content, err := os.ReadFile(".agent-lock/corrupt.lock")
	if err != nil {
		t.Fatalf("expected corrupt.lock to be created, but got error: %v", err)
	}
	if string(content) != "chaos corrupted this lock" {
		t.Fatalf("unexpected corrupt.lock content: %s", string(content))
	}
}

func TestUnknownModeString(t *testing.T) {
	mode := ChaosMode(999)
	if mode.String() != "unknown" {
		t.Fatalf("expected unknown, got %s", mode.String())
	}
}

func TestErrorString(t *testing.T) {
	err := &ChaosError{Message: "test error"}
	if err.Error() != "test error" {
		t.Fatalf("expected 'test error', got %s", err.Error())
	}
}

func TestAllModeStrings(t *testing.T) {
	modes := map[ChaosMode]string{
		NoChaos:            "no_chaos",
		LatencySpike:       "latency_spike",
		ConnectionDrop:     "connection_drop",
		ResourceExhaustion: "resource_exhaustion",
		CorruptAgentLock:   "corrupt_agent_lock",
	}

	for mode, expected := range modes {
		if mode.String() != expected {
			t.Errorf("expected %s, got %s", expected, mode.String())
		}
	}
}

func TestSetProbability(t *testing.T) {
	// Test probability 1.0 (always injects)
	inj := NewInjector(ConnectionDrop, 1)
	inj.SetProbability(1.0)

	err := inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error with probability 1.0, got nil")
	}

	// Test probability 0.0 (never injects)
	inj = NewInjector(ResourceExhaustion, 1)
	inj.SetProbability(0.0)

	for i := 0; i < 100; i++ {
		err = inj.Inject(context.Background())
		if err != nil {
			t.Fatalf("expected no error with probability 0.0, got %v", err)
		}
	}

	// Test CorruptAgentLock probability
	inj = NewInjector(CorruptAgentLock, 1)
	inj.SetProbability(0.0)
	err = inj.Inject(context.Background())
	if err != nil {
		t.Fatalf("expected no error with probability 0.0, got %v", err)
	}

	// Setup directory for CorruptAgentLock test
	err = os.MkdirAll(".agent-lock/", 0755)
	if err != nil {
		t.Fatalf("failed to create .agent-lock/ directory: %v", err)
	}
	defer os.RemoveAll(".agent-lock/") // cleanup after test

	inj.SetProbability(1.0)
	err = inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error with probability 1.0, got nil")
	}
}
