package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestAutoDreamWorker_StartStop(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db, nil)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	worker.Start(ctx, 10*time.Millisecond)

	// Let it run for a bit
	time.Sleep(50 * time.Millisecond)

	worker.Stop()
}

func TestAutoDreamWorker_PruneStaleSessionData(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db, nil)
	ctx := context.Background()

	// Insert stale heartbeat
	_, err = db.db.Exec(ctx, "INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ($1, $2, $3, $4)",
		"agent-stale", "TEST", "ACTIVE", time.Now().Add(-48*time.Hour).UTC().Format(time.RFC3339))
	if err != nil {
		t.Fatalf("failed to insert stale heartbeat: %v", err)
	}

	// Insert fresh heartbeat
	_, err = db.db.Exec(ctx, "INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ($1, $2, $3, $4)",
		"agent-fresh", "TEST", "ACTIVE", time.Now().UTC().Format(time.RFC3339))
	if err != nil {
		t.Fatalf("failed to insert fresh heartbeat: %v", err)
	}

	worker.pruneStaleSessionData(ctx)

	// Check results
	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_status WHERE agent_id = 'agent-stale'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query agent status: %v", err)
	}
	if count != 0 {
		t.Fatalf("expected stale agent to be pruned, got count %d", count)
	}

	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_status WHERE agent_id = 'agent-fresh'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query agent status: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected fresh agent to remain, got count %d", count)
	}
}

func TestAutoDreamWorker_InjectAndResolve(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db, nil)
	ctx := context.Background()

	err = worker.InjectTruth(ctx, "mem-1", "knowledge: true", []byte{1, 2, 3}, "plugin-a")
	if err != nil {
		t.Fatalf("failed to inject truth: %v", err)
	}

	// Wait slightly so timestamps differ
	time.Sleep(10 * time.Millisecond)

	err = worker.InjectTruth(ctx, "mem-2", "knowledge: false", []byte{1, 2, 3}, "plugin-a")
	if err != nil {
		t.Fatalf("failed to inject truth: %v", err)
	}

	worker.llm = &mockLLM{response: "[\"mem-1\"]"}

	worker.ResolveConflicts(ctx)

	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query embeddings: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected mock resolution to delete mem-1 and leave 1 row, got %d", count)
	}
}


type mockLLM struct {
	response string
}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return m.response, nil
}
