package chaos

import (
	"context"
	"os"
	"path/filepath"
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

func TestCorruptAgentLock(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	err := os.MkdirAll(lockDir, 0755)
	if err != nil {
		t.Fatalf("failed to create directory: %v", err)
	}

	inj := NewInjectorWithBasePath(CorruptAgentLock, 3, tmpDir)
	err = inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("expected simulated agent lock corruption error, got %v", err)
	}

	if _, err := os.Stat(filepath.Join(lockDir, "corrupt.lock")); os.IsNotExist(err) {
		t.Fatalf("expected corrupt.lock file to be created, but it was not")
	}
}

func TestSQLSyncLag(t *testing.T) {
	inj := NewInjector(SQLSyncLag, 1)

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

func TestNetworkPartition(t *testing.T) {
	inj := NewInjector(NetworkPartition, 1)
	dropped := false
	for i := 0; i < 100; i++ {
		err := inj.Inject(context.Background())
		if err != nil {
			if e, ok := err.(*ChaosError); ok && e.Message == "chaos: simulated network partition" {
				dropped = true
				break
			}
		}
	}
	if !dropped {
		t.Fatal("expected a network partition error to occur within 100 attempts")
	}
}

func TestCorruptMailbox(t *testing.T) {
	tmpDir := t.TempDir()
	mailboxDir := filepath.Join(tmpDir, ".agent-task/mailbox")
	err := os.MkdirAll(mailboxDir, 0755)
	if err != nil {
		t.Fatalf("failed to create directory: %v", err)
	}

	inj := NewInjectorWithBasePath(CorruptMailbox, 4, tmpDir)
	err = inj.Inject(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated mailbox corruption" {
		t.Fatalf("expected simulated mailbox corruption error, got %v", err)
	}

	if _, err := os.Stat(filepath.Join(mailboxDir, "corrupt.msg")); os.IsNotExist(err) {
		t.Fatalf("expected corrupt.msg file to be created, but it was not")
	}
}


func TestAllModeStrings(t *testing.T) {
	modes := map[ChaosMode]string{
		NoChaos:            "no_chaos",
		LatencySpike:       "latency_spike",
		ConnectionDrop:     "connection_drop",
		ResourceExhaustion: "resource_exhaustion",
		CorruptAgentLock:   "corrupt_agent_lock",
		SQLSyncLag:         "sql_sync_lag",
		NetworkPartition:   "network_partition",
		CorruptMailbox:     "corrupt_mailbox",
	}

	for mode, expected := range modes {
		if mode.String() != expected {
			t.Errorf("expected %s, got %s", expected, mode.String())
		}
	}
}
