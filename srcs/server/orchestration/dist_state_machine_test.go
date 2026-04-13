package orchestration

import (
	"context"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestDistributedStateMachine_Transition(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared&_txlock=immediate&_busy_timeout=5000")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	// Insert test task
	_, err = pool.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task-1', 'org1', 'title', 'PENDING')")
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	meshTransport := &MemoryMeshTransport{
		topics: make(map[string][]chan []byte),
	}

	manager, err := NewDistributedStateMachineManager(ctx, pool.Provider, nil, meshTransport)
	if err != nil {
		t.Fatalf("failed to create manager: %v", err)
	}

	// Normal transition
	newState, err := manager.Transition(ctx, "shared_tasks", "task-1", "Start", "agent-1")
	if err != nil {
		t.Fatalf("failed to transition: %v", err)
	}

	if newState != "IN_PROGRESS" {
		t.Errorf("expected state IN_PROGRESS, got %s", newState)
	}

	// Verify DB state
	state, err := manager.GetState(ctx, "shared_tasks", "task-1")
	if err != nil {
		t.Fatalf("failed to get state: %v", err)
	}
	if state != "IN_PROGRESS" {
		t.Errorf("expected db state IN_PROGRESS, got %s", state)
	}

	// Verify audit log
	var count int
	err = pool.QueryRow(ctx, "SELECT count(*) FROM state_machine_transitions WHERE entity_id = 'task-1'").Scan(&count)
	if err != nil || count != 1 {
		t.Errorf("expected 1 audit log entry, got %d (err: %v)", count, err)
	}

	// Invalid transition
	_, err = manager.Transition(ctx, "shared_tasks", "task-1", "Start", "agent-1")
	if err == nil {
		t.Errorf("expected error for invalid transition")
	}

	// Concurrent transitions simulation
	var wg sync.WaitGroup
	successCount := 0
	var mu sync.Mutex

	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			_, err := manager.Transition(ctx, "shared_tasks", "task-1", "Complete", "agent-concurrent")
			if err == nil {
				mu.Lock()
				successCount++
				mu.Unlock()
			}
		}()
	}

	wg.Wait()

	// Only one should succeed
	if successCount != 1 {
		t.Errorf("expected 1 successful concurrent transition, got %d", successCount)
	}

	state, _ = manager.GetState(ctx, "shared_tasks", "task-1")
	if state != "COMPLETED" {
		t.Errorf("expected db state COMPLETED, got %s", state)
	}
}
