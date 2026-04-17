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

func TestSqlSyncLag(t *testing.T) {
	inj := NewInjector(SqlSyncLag, 1)

	start := time.Now()
	err := inj.Inject(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	duration := time.Since(start)
	if duration < 50*time.Millisecond {
		t.Fatalf("expected delay > 50ms, got %v", duration)
	}
}

func TestSqlSyncLagContextCancellation(t *testing.T) {
	inj := NewInjector(SqlSyncLag, 1)

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

func TestNetworkPartition(t *testing.T) {
	inj := NewInjector(NetworkPartition, 1)
	partitioned := false
	for i := 0; i < 100; i++ {
		err := inj.Inject(context.Background())
		if err != nil {
			if e, ok := err.(*ChaosError); ok && e.Message == "chaos: simulated network partition" {
				partitioned = true
				break
			}
		}
	}
	if !partitioned {
		t.Fatal("expected a network partition error to occur within 100 attempts")
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
	_ = os.MkdirAll(".agent-lock", 0755)
	defer os.RemoveAll(".agent-lock")

	inj := NewInjector(CorruptAgentLock, 1)
	err := inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}

	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("unexpected error: %v", err)
	}

	_, err = os.Stat(".agent-lock/corrupt.lock")
	if os.IsNotExist(err) {
		t.Fatal("expected corrupt.lock to be created")
	}
}

func TestCorruptMailbox(t *testing.T) {
	_ = os.MkdirAll("mailbox", 0755)
	defer os.RemoveAll("mailbox")

	inj := NewInjector(CorruptMailbox, 1)
	err := inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}

	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated mailbox corruption" {
		t.Fatalf("unexpected error: %v", err)
	}

	_, err = os.Stat("mailbox/corrupt.mbox")
	if os.IsNotExist(err) {
		t.Fatal("expected corrupt.mbox to be created")
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
		SqlSyncLag:         "sql_sync_lag",
		NetworkPartition:   "network_partition",
		CorruptMailbox:     "corrupt_mailbox",
	}

	for mode, expected := range modes {
		if mode.String() != expected {
			t.Errorf("expected %s, got %s", expected, mode.String())
		}
	}
}
