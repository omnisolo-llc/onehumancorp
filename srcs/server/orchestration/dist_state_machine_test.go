package orchestration

import (
	"context"
	"database/sql"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"fmt"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"
	_ "modernc.org/sqlite"
)

type mockMeshTransport struct {
	events [][]byte
	mu     sync.Mutex
}

func (m *mockMeshTransport) BroadcastTask(ctx context.Context, task Task) error { return nil }
func (m *mockMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) { return nil, nil }
func (m *mockMeshTransport) BroadcastCoordination(ctx context.Context, msg MeshMessage) error { return nil }
func (m *mockMeshTransport) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) { return nil, nil }
func (m *mockMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error { return nil }
func (m *mockMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) { return nil, nil }
func (m *mockMeshTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) { return nil, nil }
func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.events = append(m.events, payload)
	return nil
}

func setupTestDBForSM(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file:memdb_sm?mode=memory&cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			agent_id TEXT,
			updated_at DATETIME
		);
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT NOT NULL,
			reason TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS distributed_locks (
			lock_key TEXT PRIMARY KEY,
			owner_id TEXT NOT NULL,
			expires_at DATETIME NOT NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create test tables: %v", err)
	}

	return provider
}

func TestDistributedStateMachine_Transition(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDBForSM(t)

	// Insert initial data
	entityID := "task-1"
	_, err := provider.Exec(ctx, `INSERT INTO shared_tasks (id, status) VALUES ($1, $2)`, entityID, statemachine.StatePending)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	mesh := &mockMeshTransport{}
	sm, err := NewDistributedStateMachine(ctx, provider, nil, mesh)
	if err != nil {
		t.Fatalf("failed to create DistributedStateMachine: %v", err)
	}

	// Test 1: Successful transition
	err = sm.Transition(ctx, entityID, "SHARED_TASK", statemachine.StateInProgress, "agent-1", "starting work")
	if err != nil {
		t.Fatalf("unexpected error on valid transition: %v", err)
	}

	state, err := sm.GetState(ctx, entityID, "SHARED_TASK")
	if err != nil {
		t.Fatalf("failed to get state: %v", err)
	}
	if state != statemachine.StateInProgress {
		t.Errorf("expected state %s, got %s", statemachine.StateInProgress, state)
	}

	mesh.mu.Lock()
	if len(mesh.events) != 1 {
		t.Errorf("expected 1 mesh event, got %d", len(mesh.events))
	}
	mesh.mu.Unlock()

	// Test 2: Invalid transition
	err = sm.Transition(ctx, entityID, "SHARED_TASK", statemachine.StatePending, "agent-1", "going backwards")
	if err == nil {
		t.Fatalf("expected error on invalid transition, got nil")
	}

	// Test 3: Concurrent transitions
	entityID2 := "task-2"
	_, _ = provider.Exec(ctx, `INSERT INTO shared_tasks (id, status) VALUES ($1, $2)`, entityID2, statemachine.StatePending)

	var wg sync.WaitGroup
	errs := make(chan error, 5)

	// Simulate 5 concurrent attempts to claim the same task
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(agentIdx int) {
			defer wg.Done()
			agent := fmt.Sprintf("concurrent-agent-%d", agentIdx)
			time.Sleep(10 * time.Millisecond)
			err := sm.Transition(ctx, entityID2, "SHARED_TASK", statemachine.StateInProgress, agent, "concurrent claim")
			if err != nil {
				errs <- err
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	var transitionCount int
	err = provider.QueryRow(ctx, `SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id = $1`, entityID2).Scan(&transitionCount)
	if err != nil {
		t.Fatalf("failed to query transition count: %v", err)
	}

	if transitionCount != 1 {
		t.Errorf("expected exactly 1 recorded transition, got %d", transitionCount)
	}
}
