package kairos

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/db"
	"testing"
	"time"
)

func TestKairosSharedTaskRepo(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create the table just like the other tests do, in case migrations drop it.
	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME
        );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewSharedTaskRepo(provider, nil)
	task := &SharedTask{
		ID:        "test-uuid",
		AgentID:   "agent-1",
		Status:    "PENDING",
		Payload:   []byte(`{"hello":"world"}`),
		CreatedAt: time.Now().Truncate(time.Second).UTC(),
	}

	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	fetched, err := repo.Get(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get: %v", err)
	}

	if fetched.ID != task.ID || fetched.AgentID != task.AgentID || fetched.Status != task.Status {
		t.Errorf("mismatch: %+v != %+v", fetched, task)
	}
	if string(fetched.Payload) != string(task.Payload) {
		t.Errorf("payload mismatch: %s != %s", string(fetched.Payload), string(task.Payload))
	}
	if !fetched.CreatedAt.Equal(task.CreatedAt) {
		t.Errorf("created_at mismatch: %v != %v", fetched.CreatedAt, task.CreatedAt)
	}
}

type MockMutex struct {
	Locked bool
}

func (m *MockMutex) Lock(ctx context.Context, ttl time.Duration) error {
	m.Locked = true
	return nil
}

func (m *MockMutex) Unlock(ctx context.Context) error {
	m.Locked = false
	return nil
}

type MockMutexProvider struct {
	Mutexes map[string]*MockMutex
}

func (p *MockMutexProvider) NewMutex(key string) Mutex {
	if p.Mutexes == nil {
		p.Mutexes = make(map[string]*MockMutex)
	}
	if _, ok := p.Mutexes[key]; !ok {
		p.Mutexes[key] = &MockMutex{}
	}
	return p.Mutexes[key]
}

func TestClaimTask(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME
        );
    `)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	_, err = provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at DATETIME
        );
    `)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions table: %v", err)
	}

	mockMutexProvider := &MockMutexProvider{}
	repo := NewSharedTaskRepo(provider, mockMutexProvider)

	task := &SharedTask{
		ID:        "test-claim-uuid",
		AgentID:   "",
		Status:    "PENDING",
		Payload:   []byte(`{"hello":"world"}`),
		CreatedAt: time.Now().Truncate(time.Second).UTC(),
	}

	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	claimedTask, err := repo.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}

	if claimedTask.Status != "IN_PROGRESS" || claimedTask.AgentID != "agent-1" {
		t.Errorf("task not claimed correctly: %+v", claimedTask)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id = $1", "test-claim-uuid").Scan(&count)
	if err != nil || count != 1 {
		t.Errorf("expected 1 state transition log, got %d, err: %v", count, err)
	}
}

func TestTransitionTask(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME
        );
    `)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	_, err = provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at DATETIME
        );
    `)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions table: %v", err)
	}

	mockMutexProvider := &MockMutexProvider{}
	repo := NewSharedTaskRepo(provider, mockMutexProvider)

	task := &SharedTask{
		ID:        "test-transition-uuid",
		AgentID:   "agent-1",
		Status:    "IN_PROGRESS",
		Payload:   []byte(`{"hello":"world"}`),
		CreatedAt: time.Now().Truncate(time.Second).UTC(),
	}

	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	err = repo.TransitionTask(ctx, "test-transition-uuid", "agent-1", "IN_PROGRESS", "COMPLETED", "task finished")
	if err != nil {
		t.Fatalf("failed to transition task: %v", err)
	}

	fetched, err := repo.Get(ctx, "test-transition-uuid")
	if err != nil {
		t.Fatalf("failed to get task: %v", err)
	}

	if fetched.Status != "COMPLETED" {
		t.Errorf("task not transitioned correctly: %+v", fetched)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM state_machine_transitions WHERE entity_id = $1", "test-transition-uuid").Scan(&count)
	if err != nil || count != 1 {
		t.Errorf("expected 1 state transition log, got %d, err: %v", count, err)
	}
}
